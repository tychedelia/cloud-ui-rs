use std::any::Any;
use crate::cloud::aws::{AwsProvider, AwsService};
use crate::service;
use crate::cloud::aws;
use crate::service::resource::{Resource, ResourceDescription, ResourceType};
use async_trait::async_trait;

pub struct Ec2 {
    client: Option<aws_sdk_ec2::Client>,
    provider: AwsProvider,
}


#[async_trait]
impl service::Service for Ec2 {
    type Provider = AwsProvider;

    fn new() -> Ec2 {
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
            ResourceType("Instances".to_string())
        ])
    }
}

#[async_trait]
impl aws::AwsService<aws_sdk_ec2::Client> for Ec2 {
    async fn new_client() -> anyhow::Result<aws_sdk_ec2::Client> {
        let config= Self::Provider::new().get_config().await;
        let client = aws_sdk_ec2::Client::new(&config);
        Ok(client)
    }
}

struct StreamCrud {
    svc: Ec2,
}

#[async_trait]
impl service::resource::ResourceController<Instance> for StreamCrud {
    async fn list(&self) -> anyhow::Result<Vec<Instance>> {
        let streams = self.svc.client.as_ref().unwrap().describe_instances().send().await?
            .reservations
            .unwrap_or(vec![])
            .iter()
            .map(|x| {
                Instance {
                    name: x.reservation_id.as_ref().unwrap().clone(),
                }
            })
            .collect();
        Ok(streams)
    }

    async fn describe(&self, id: String) -> anyhow::Result<Option<ResourceDescription<Instance>>> {
        Ok(None)
    }
}

struct Instance {
    name: String,
}

impl Resource for Instance {
    type Id = String;

    fn get_name(&self) -> String {
        self.name.clone()
    }
}