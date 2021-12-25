pub(crate) mod service;

use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem};

pub trait Ui<T> {
    fn ui<B>(&self, f: &mut Frame<B>, area: Rect, state: T) -> anyhow::Result<()>
        where B: Backend;
}

pub(crate) fn ui<B: Backend>(f: &mut Frame<B>, app: &mut crate::app::App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app
        .services
        .items
        .iter()
        .map(|service| {
            let mut lines = vec![Spans::from(service.clone())];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    let services = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");


    f.render_widget(services, chunks[0]);
}