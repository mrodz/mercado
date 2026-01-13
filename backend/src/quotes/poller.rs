use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use reqwest::Client;
use rocket::tokio::{
    spawn,
    sync::{Mutex, RwLock, broadcast},
    task::JoinHandle,
    time::sleep,
};

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

type Callback<T, State> = dyn FnMut(&Client, Vec<State>) -> BoxFuture<T> + Send + 'static;

pub trait StateLike: Send + Sync + PartialEq + 'static {}

impl<T> StateLike for T where T: Send + Sync + PartialEq + 'static {}

struct Inner<T, State>
where
    T: Clone + Send + 'static,
    State: StateLike + Sized,
{
    // subscription fanout
    tx: broadcast::Sender<T>,

    // lifecycle
    handle: Mutex<Option<JoinHandle<()>>>,
    subscribers: AtomicUsize,

    // shared state
    queued: RwLock<Vec<State>>,
    delay: Duration,

    // FnMut needs interior mutability
    cb: Mutex<Box<Callback<T, State>>>,
}

#[derive(Clone)]
pub struct Poller<T, State>
where
    T: Clone + Send + 'static,
    State: StateLike,
{
    inner: Arc<Inner<T, State>>,
}

impl<T, State> Debug for Poller<T, State>
where
    T: Clone + Send + 'static,
    State: StateLike + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Poller")
            .field("delay", &self.inner.delay)
            .field("state", &self.inner.queued.try_read())
            .finish()
    }
}

pub struct Subscription<T, State>
where
    T: Clone + Send + 'static,
    State: StateLike,
{
    rx: broadcast::Receiver<T>,
    inner: Arc<Inner<T, State>>,
}

impl<T, State> Subscription<T, State>
where
    T: Clone + Send + 'static,
    State: StateLike,
{
    pub async fn recv(&mut self) -> Result<T, broadcast::error::RecvError> {
        self.rx.recv().await
    }
}

impl<T, State> Drop for Subscription<T, State>
where
    T: Clone + Send + 'static,
    State: StateLike,
{
    fn drop(&mut self) {
        // If this was the last subscriber, stop the poll loop.
        let prev = self.inner.subscribers.fetch_sub(1, Ordering::AcqRel);
        if prev == 1 {
            // last one out turns off the lights
            // abort asynchronously-safe: just abort the JoinHandle if present
            let inner = self.inner.clone();
            // We can’t .await in Drop; spawn a tiny task to do it.
            spawn(async move {
                let mut h = inner.handle.lock().await;
                if let Some(handle) = h.take() {
                    handle.abort();
                }
                let mut q = inner.queued.write().await;
                q.clear();
            });
        }
    }
}

impl<T, State> Poller<T, State>
where
    T: Clone + Send + 'static,
    State: StateLike,
{
    fn spawn_if_needed(inner: Arc<Inner<T, State>>) {
        spawn(async move {
            let mut guard = inner.handle.lock().await;

            // already running?
            if guard.as_ref().is_some_and(|h| !h.is_finished()) {
                return;
            }

            // only run if someone is actually subscribed
            if inner.subscribers.load(Ordering::Acquire) == 0 {
                return;
            }

            let task_inner = inner.clone();
            let handle = spawn(async move {
                let client = Client::new();

                loop {
                    // If nobody is listening, stop.
                    if task_inner.subscribers.load(Ordering::Acquire) == 0 {
                        break;
                    }

                    let owned = {
                        let mut q = task_inner.queued.write().await;
                        std::mem::take(&mut *q)
                    };

                    let value = {
                        let mut cb = task_inner.cb.lock().await;
                        (cb)(&client, owned).await
                    };

                    // broadcast::Sender::send is synchronous; ignore "no receivers" errors
                    let _ = task_inner.tx.send(value);

                    sleep(task_inner.delay).await;
                }
            });

            *guard = Some(handle);
        });
    }

    pub fn new<F>(buffer: usize, delay: Duration, cb: F) -> Self
    where
        F: FnMut(&Client, Vec<State>) -> BoxFuture<T> + Send + 'static,
    {
        let (tx, _rx_unused) = broadcast::channel::<T>(buffer);

        let inner = Arc::new(Inner {
            tx,
            handle: Mutex::new(None),
            subscribers: AtomicUsize::new(0),
            queued: RwLock::new(Vec::new()),
            delay,
            cb: Mutex::new(Box::new(cb)),
        });

        Self { inner }
    }

    pub async fn set_state(&self, states: Vec<State>) {
        let mut q = self.inner.queued.write().await;
        *q = states;
    }

    pub async fn extend_unique(&self, states: Vec<State>) {
        let mut q = self.inner.queued.write().await;

        for state in states {
            if !q.contains(&state) {
                q.push(state);
            }
        }
    }

    /// Create a subscriber. If this is the first subscriber, the poller spawns.
    pub fn subscribe(&self) -> Subscription<T, State> {
        let prev = self.inner.subscribers.fetch_add(1, Ordering::AcqRel);

        // each subscriber gets its own receiver
        let rx = self.inner.tx.subscribe();

        // if we went 0 -> 1, ensure the task is running
        if prev == 0 {
            Self::spawn_if_needed(self.inner.clone());
        }

        Subscription {
            rx,
            inner: self.inner.clone(),
        }
    }
}

impl<T, State> Drop for Poller<T, State>
where
    T: Clone + Send + 'static,
    State: StateLike,
{
    fn drop(&mut self) {
        let inner = self.inner.clone();

        let Ok(mut x) = inner.handle.try_lock() else {
            eprintln!("could not get handle to gracefully shutdown Poller");
            return;
        };

        if let Some(handle) = x.take() {
            if !handle.is_finished() {
                handle.abort();
            }
        }
    }
}
