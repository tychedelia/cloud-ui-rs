use std::any;
use std::future::Future;
use async_trait::async_trait;
use crate::service::resource::{ResourceType};

pub(crate) mod resource;

pub(crate) struct ServiceType(pub String);

macro_rules! services {
    ($($body:tt)*) => {
        pub(crate) enum Services { $($body($body),)* }

        impl crate::ui::Ui<()> for Services {
            fn ui<B>(&mut self, f: &mut tui::Frame<B>, area: tui::layout::Rect, state: &mut ()) -> anyhow::Result<()>
                where B: tui::backend::Backend {
                match self {
                    $(Services::$body(svc) => crate::ui::service::ServiceUi::new(svc).ui(f, area, state),)*
                }
            }
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

#[async_trait]
pub(crate) trait Service
{
    type Provider: Provider + ?Sized;

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
