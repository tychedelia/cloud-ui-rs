use futures::StreamExt;
use tui::backend::Backend;
use tui::buffer::Buffer;
use tui::Frame;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};
use crate::app::StatefulList;
use crate::service;
use crate::service::Resource;
use crate::ui;

pub(crate) struct ResourceState {
    pub(crate) resources: StatefulList<String>,
}

pub(crate) struct ResourceUi<'a, S>
{
    svc: &'a S,
    state: ResourceState,
}

impl <'a, S> ResourceUi<'a, S>
    where S: Resource {
    pub fn new(svc: &'a S) -> Self {
        let items = svc.get_resources().unwrap()
            .iter()
            .map(|x| x.0.clone())
            .collect();
        Self {
            svc,
            state: ResourceState { resources: StatefulList::with_items(items) }
        }
    }
}

impl <'a, S> ui::Ui<ResourceState> for ResourceUi<'a, S>
    where S: service::Resource
{
    fn ui<B>(&self, f: &mut Frame<B>, area: Rect, state: &mut ResourceState) -> anyhow::Result<()>
        where B: Backend {
        let resource_names: Vec<String> = self.svc
            .get_resources()?
            .iter()
            .map(|x| x.0.clone())
            .collect();

        let resources: Vec<ListItem> = resource_names
            .iter()
            .map(|r| {
                let mut lines = vec![Spans::from(r.clone())];
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        let resources = List::new(resources)
            .block(Block::default().borders(Borders::TOP).title("Resources"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        // We can now render the item list
        f.render_stateful_widget(resources, area, &mut state.resources.state);
        Ok(())
    }
}