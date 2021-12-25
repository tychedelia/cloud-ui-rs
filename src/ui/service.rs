use futures::StreamExt;
use tui::backend::Backend;
use tui::buffer::Buffer;
use tui::Frame;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Table};
use crate::app::StatefulList;
use crate::service;
use crate::service::Service;
use crate::ui;
use crate::ui::component::TableList;

pub(crate) struct ServiceState {
    pub(crate) resources: StatefulList<String>,
}

pub(crate) struct ServiceUi<'a, S>
{
    svc: &'a S,
    state: ServiceState,
}

impl <'a, S> ServiceUi<'a, S>
    where S: Service {
    pub fn new(svc: &'a S) -> Self {
        let items = svc.get_resources().unwrap()
            .iter()
            .map(|x| x.0.clone())
            .collect();
        Self {
            svc,
            state: ServiceState { resources: StatefulList::with_items(items) }
        }
    }
}

impl <'a, S> ui::Ui<()> for ServiceUi<'a, S>
    where S: service::Service
{
    fn ui<B>(&mut self, f: &mut Frame<B>, area: Rect, state: &mut ()) -> anyhow::Result<()>
        where B: Backend {
        let mut tl = TableList {};
        tl.ui(f, area, &mut self.state.resources);
        Ok(())
    }
}