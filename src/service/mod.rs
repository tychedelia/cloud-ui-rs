use std::any;
use std::future::Future;
use async_trait::async_trait;
use crate::service::resource::{Resource, ResourceType};

pub(crate) mod resource;

pub(crate) struct ServiceType(pub String);

#[async_trait]
pub(crate) trait Service
    where Self: Sized
{
    type Provider: Provider;

    async fn new() -> anyhow::Result<Self>;

    fn get_resources(&self) -> anyhow::Result<Vec<ResourceType>>;
}

#[async_trait]
pub(crate) trait Provider
    where Self: Sized
{
    async fn new() -> anyhow::Result<Self>;
}
