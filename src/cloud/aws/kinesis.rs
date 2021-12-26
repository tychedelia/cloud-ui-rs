use std::any::Any;
use crate::cloud::aws::{AwsProvider, AwsService};
use crate::service;
use crate::cloud::aws;
use crate::service::resource::{Resource, ResourceDescription, ResourceType};
use async_trait::async_trait;
use crate::service::Service;

pub(crate) struct Kinesis {
    client: Option<aws_sdk_kinesis::Client>,
    provider: AwsProvider,
}


#[async_trait]
impl service::Service for Kinesis {
    type Provider = AwsProvider;

    fn new() -> Kinesis {
        Self {
            client: None,
            provider: AwsProvider::new(),
        }
    }

    async fn run(self)  -> anyhow::Result<()> {
        todo!()
    }

    fn get_resources(&self) -> anyhow::Result<Vec<ResourceType>> {
        Ok(vec![
            ResourceType("Stream".to_string())
        ])
    }
}

#[async_trait]
impl aws::AwsService<aws_sdk_kinesis::Client> for Kinesis {
    async fn new_client() -> anyhow::Result<aws_sdk_kinesis::Client> {
        let config= Self::Provider::new().get_config().await;
        let client = aws_sdk_kinesis::Client::new(&config);
        Ok(client)
    }
}

pub(crate) struct Streams {
    svc: Kinesis,
}

impl Streams {
    fn new() -> Self {
        Self {
            svc: Kinesis::new()
        }
    }
}

#[async_trait]
impl service::resource::ResourceController<Stream> for Streams {
    async fn list(&self) -> anyhow::Result<Vec<Stream>> {
        let streams = self.svc.client.as_ref().unwrap().list_streams().send().await?
            .stream_names
            .unwrap_or(vec![])
            .iter()
            .map(|x| {
                Stream {
                    name: x.clone(),
                }
            })
            .collect();
        Ok(streams)
    }

    async fn describe(&self, id: String) -> anyhow::Result<Option<ResourceDescription<Stream>>> {
        let description = self.svc.client.as_ref().unwrap().describe_stream()
            .stream_name(&id)
            .send().await?
            .stream_description
            .map(|x| {
               ResourceDescription {
                   id: id.clone(),
                   name: Some(id.clone()),
                   props: Default::default()
               }
            });
        Ok(description)
    }
}

pub(crate) struct Stream {
    name: String,
}

impl Resource for Stream {
    type Id = String;

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

service::resource::resources! {
    Streams, Stream,
}