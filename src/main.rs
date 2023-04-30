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
            panic!("UH OH!");
            format!("Hello World")
        };
        println!("{}", to_print);
        Ok(None)
    }
}

// #[derive(Copy, Clone)]
// struct Example;

// #[async_trait]
// impl ClientMiddleware for Example {
//     async fn transform(
//         &self,
//         from: Box<dyn Job>,
//         input: Option<String>,
//     ) -> Result<(Box<dyn Job>, Option<String>), CwabError> {
//         match from.name() {
//             "HelloJob" => Ok((from, Some("override".to_string()))),
//             _ => Ok((from, input)),
//         }
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new(None)?;
    let cwab = Cwab::new(&config)?;
    // cwab.register_middleware(Example);
    let mut worker = cwab.worker();
    worker.register(HelloJob);

    cwab.perform_async(HelloJob, None).await?;
    cwab.perform_async(HelloJob, Some("Bob".to_string()))
        .await?;

    worker.start_working().await?;
    Ok(())
}
