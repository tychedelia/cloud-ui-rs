use std::future::Future;
use crate::service;
use async_trait::async_trait;
use crate::service::Service;

mod kinesis;

struct Arn(String);

pub struct AwsProvider {

}

#[async_trait]
trait AwsService<T>
    where Self: Service
{
    async fn new_client() -> anyhow::Result<T>;
}

trait AwsResource {}

impl AwsProvider {
    fn new() -> Self {
        AwsProvider {}
    }

    async fn get_config(&self) -> aws_config::Config {
        aws_config::load_from_env().await
    }
}

#[async_trait]
impl service::Provider for AwsProvider {
    async fn new() -> anyhow::Result<Self> {
        Ok(AwsProvider {})
    }
}