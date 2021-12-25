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
use crate::ui;

struct ServiceState {
    resources: StatefulList<String>,
}

pub(crate) struct ServiceUi<S>
{
    svc: S,
    state: ServiceState,
}

impl <S> ui::Ui<ServiceState> for ServiceUi<S>
    where S: service::Service
{
    fn ui<B>(&self, f: &mut Frame<B>, area: Rect, mut state: ServiceState) -> anyhow::Result<()>
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
            .block(Block::default().borders(Borders::ALL).title("Services"))
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