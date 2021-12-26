use std::collections::HashMap;
use async_trait::async_trait;

pub struct ResourceType(pub String);

pub(crate) trait ResourceKind
    where
        Self: Ui<()> + Send + GetItems {
}

macro_rules! resources {
    ($($c:ident, $r:ident),*$(,)*) => {
        pub(crate) enum Resources { $($c($c),)* }

        impl crate::ui::Ui<()> for Resources {
            fn ui<B>(&mut self, f: &mut tui::Frame<B>, area: tui::layout::Rect, state: &mut ()) -> anyhow::Result<()>
                where B: tui::backend::Backend {
                match self {
                    $(Resources::$c(ctrl) => crate::ui::resource::ResourceUi::<$c, $r>::new(ctrl).ui(f, area, state),)*
                }
            }
        }

        impl crate::app::GetItems for Resources {
            fn get_items() -> Vec<String> {
                vec![
                    $(stringify!($c).to_string(),)*
                ]
            }
        }

        impl From<String> for Resources {
            fn from(str: String) -> Resources {
                match str.as_str() {
                    $(stringify!($c) => Resources::$c($c::new()),)*
                    _ => panic!(),
                }
            }
        }

        impl crate::service::resource::ResourceKind for Resources {}
    };
}

pub(crate) use resources;
use crate::app::GetItems;
use crate::ui::Ui;

pub(crate) trait Resource
    // where Self: Sized
{
    type Id;

    fn get_name(&self) -> String;
}

pub(crate) struct ResourceDescription<T>
    where T: Resource
{
    pub(crate) id: T::Id,
    pub(crate) name: Option<String>,
    pub(crate) props: HashMap<String, String>,
}

#[async_trait]
pub(crate) trait ResourceController<T>
    where T: Resource
{
    async fn list(&self) -> anyhow::Result<Vec<T>>;
    async fn describe(&self, id: T::Id) -> anyhow::Result<Option<ResourceDescription<T>>>;
}