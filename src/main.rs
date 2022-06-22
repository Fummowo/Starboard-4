pub mod client;
pub mod events;
pub mod interactions;

use anyhow::Result;

use crate::client::bot::Starboard;
use crate::client::config::Config;
use crate::client::runner::run;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env();
    let (events, starboard) = Starboard::new(config).await?;
    run(events, starboard).await;

    Ok(())
}
