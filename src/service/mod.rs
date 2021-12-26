use std::any;
use std::future::Future;
use async_trait::async_trait;
use crate::service::resource::{ResourceKind, ResourceType};

pub(crate) mod resource;

pub(crate) struct ServiceType(pub String);

pub(crate) trait ServiceKind
    where Self: GetItems + From<String>
{}

macro_rules! services {
    ($($p:ident, $s:ident),*$(,)*) => {
        pub(crate) enum Services { $($s($s),)* }

        impl crate::service::ServiceKind for Services {}

        impl crate::ui::Ui<()> for Services {
            fn ui<B>(&mut self, f: &mut tui::Frame<B>, area: tui::layout::Rect, state: &mut ()) -> anyhow::Result<()>
                where B: tui::backend::Backend {
                match self {
                    $(Services::$s(svc) => crate::ui::service::ServiceUi::new(svc).ui(f, area, state),)*
                }
            }
        }

        impl crate::app::GetItems for Services {
            fn get_items() -> Vec<String> {
                vec![
                    $(stringify!($s).to_string(),)*
                ]
            }
        }

        impl From<String> for Services {
            fn from(str: String) -> Services {
                match str.as_str() {
                    $(stringify!($s) => Services::$s($s::new()),)*
                    _ => panic!(),
                }
            }
        }
    };
}


pub(crate) use services;
use crate::app::GetItems;

#[async_trait]
pub(crate) trait Service<'a> {
    type Provider: Provider + ?Sized;
    type Resources: ResourceKind;

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
