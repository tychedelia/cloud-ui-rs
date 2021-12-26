use std::io::Stdout;
use std::marker::PhantomData;
use futures::StreamExt;
use tui::backend::{Backend, CrosstermBackend};
use tui::buffer::Buffer;
use tui::Frame;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};
use crate::app::StatefulList;
use crate::service;
use crate::service::{Service};
use crate::service::resource::{Resource, ResourceController};
use crate::ui;
use crate::ui::component::TableList;

pub(crate) struct ResourceState {
    pub(crate) items: StatefulList<String>,
}

pub(crate) struct ResourceUi<'a, R, T>
{
    ctrl: &'a R,
    state: ResourceState,
    _t: PhantomData<T>,
}

impl <'a, R, T> ResourceUi<'a, R, T>
    where R: ResourceController<T>,
          T: Resource {
    pub fn new(ctrl: &'a R) -> Self {
        Self {
            ctrl,
            state: ResourceState { items: StatefulList::with_items(vec![]) },
            _t: Default::default()
        }
    }

    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.state.items.items = self.ctrl.list().await?
            .iter()
            .map(|x| x.get_name())
            .collect();
        Ok(())
    }
}

impl <'a, R, T> ui::Ui<()> for ResourceUi<'a, R, T>
    where R: ResourceController<T>,
          T: Resource,
{
    fn ui<B>(&mut self, f: &mut Frame<B>, area: Rect, state: &mut ()) -> anyhow::Result<()>
        where B: Backend {
        let mut tl = TableList {};
        tl.ui(f, area, &mut self.state.items)?;
        Ok(())
    }
}