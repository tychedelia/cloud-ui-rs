use aws_sdk_kinesis::Client;

pub(crate) struct Streams {
    client: Client,
}

impl Streams {
    pub(crate) async fn new() -> Self {
        let client = Self::get_client().await;
        Streams {
            client,
        }
    }

    pub(crate) async fn run(
        self,
        tx: tokio::sync::mpsc::Sender<Vec<String>>,
        mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        let mut tick = tokio::time::interval(tokio::time::Duration::from_secs(10));
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    return Ok(())
                }
                _ = tick.tick() => {
                    let streams = self.client.list_streams().send().await?;
                    tx.send(streams.stream_names.unwrap_or(vec![])).await?;
                }
            }
        }
    }

    async fn get_client() -> aws_sdk_kinesis::Client {
        Client::new(&Self::get_config().await)
    }

    async fn get_config() -> aws_config::Config {
        aws_config::load_from_env().await
    }
}
