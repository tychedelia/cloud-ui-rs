use std::any;
use std::future::Future;
use async_trait::async_trait;
use crate::service::resource::{Resource, ResourceType};

pub(crate) mod resource;

pub(crate) struct ServiceType(pub String);

macro_rules! as_item {
    ($i:item) => { $i };
}

macro_rules! services {
    ($($body:tt)*) => {
        crate::service::as_item! {
            pub enum Services { $($body($body),)* }
        }

        impl crate::app::GetItems for Services {
            fn get_items() -> Vec<String> {
                vec![
                    $(stringify!($body).to_string(),)*
                ]
            }
        }

        impl From<String> for Services {
            fn from(str: String) -> Services {
                match str.as_str() {
                    $(stringify!($body) => Services::$body($body::new()),)*
                    _ => panic!(),
                }
            }
        }
    };
}

pub(crate) use services;
pub(crate) use as_item;

#[async_trait]
pub(crate) trait Service
    where Self: Sized
{
    type Provider: Provider;

    fn new() -> Self;

    async fn run(self)  -> anyhow::Result<()>;

    fn get_resources(&self) -> anyhow::Result<Vec<ResourceType>>;
}

#[async_trait]
pub(crate) trait Provider
    where Self: Sized
{
    async fn new() -> anyhow::Result<Self>;
}
