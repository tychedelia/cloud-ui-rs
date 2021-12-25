use std::collections::HashMap;
use async_trait::async_trait;

pub struct ResourceType(pub String);

pub trait Resource
    // where Self: Sized
{
    type Id;

    fn get_name(&self) -> String;
}

pub struct ResourceDescription<T>
    where T: Resource
{
    pub(crate) id: T::Id,
    pub(crate) name: Option<String>,
    pub(crate) props: HashMap<String, String>,
}

#[async_trait]
pub trait ResourceCrud<T>
    where T: Resource
{
    async fn list(&self) -> anyhow::Result<Vec<T>>;
    async fn get(&self, id: T::Id) -> anyhow::Result<Option<T>>;
    async fn describe(&self, id: T::Id) -> anyhow::Result<Option<ResourceDescription<T>>>;
}