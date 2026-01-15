use barter_data::exchange::schwab::Schwab;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    {
        let mut stream = Schwab::stream("NVDA", "NVDA").await.unwrap();

        while let Some(event) = stream.next().await {
            println!("{event:?}");
        }
    }
}
