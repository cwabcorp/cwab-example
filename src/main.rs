use anyhow::Result;
use async_trait::async_trait;
use cwab::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct HelloJob;

#[async_trait]
impl Job for HelloJob {
    fn name(&self) -> &'static str {
        "HelloJob"
    }

    async fn perform(&self, input: Option<String>) -> Result<Option<String>, anyhow::Error> {
        let to_print = if let Some(i) = input {
            format!("Hello {}", i)
        } else {
            "Hello World".to_string()
        };
        println!("{}", to_print);
        Ok(None)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new(None)?;
    let cwab = Cwab::new(&config)?;
    let mut worker = cwab.worker();
    worker.register(HelloJob);

    cwab.perform_async(HelloJob, None).await?;
    cwab.perform_async(HelloJob, Some("Bob".to_string()))
        .await?;

    worker.start_working().await?;
    Ok(())
}
