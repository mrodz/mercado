mod poller;

use std::{collections::HashMap, sync::Arc, time::Duration};

pub use poller::Poller;
use rocket::tokio::sync::RwLock;

use crate::{
    errors::ApplicationError,
    oauth::Credentials,
    quotes::poller::Subscription,
    schwab::{self, schema::QuoteResponse},
};

#[derive(Debug)]
pub struct QuotesState {
    poller: Poller<Result<QuoteResponse, Arc<ApplicationError>>, String>,
    credentials: Arc<RwLock<Option<Credentials>>>,
}

impl QuotesState {
    pub fn new() -> Self {
        let credentials: Arc<RwLock<Option<Credentials>>> = Arc::default();
        let c2 = credentials.clone();

        let poller = Poller::new(16, Duration::from_millis(500), move |client, states| {
            let client = client.clone();
            let credentials = credentials.clone();
            Box::pin(async move {
                let credentials = credentials.read().await;
                let Some(credentials) = credentials.clone() else {
                    panic!("credentials were not set");
                };

                if states.is_empty() {
                    return Ok(QuoteResponse {
                        quotes: HashMap::new(),
                    });
                }

                let response = schwab::get_quote(credentials, Some(client), states).await?;

                Ok(response)
            })
        });

        Self {
            poller,
            credentials: c2,
        }
    }

    pub async fn set_quotes(&self, quotes: Vec<String>) {
        self.poller.set_state(quotes).await
    }

    pub async fn extend_quotes(&self, quotes: Vec<String>) {
        self.poller.extend_unique(quotes).await
    }

    pub async fn subscribe(
        &self,
    ) -> Subscription<Result<QuoteResponse, Arc<ApplicationError>>, String> {
        self.poller.subscribe()
    }

    pub async fn set_credentials(&self, credentials: Credentials) {
        let mut w = self.credentials.write().await;

        *w = Some(credentials);
    }
}
