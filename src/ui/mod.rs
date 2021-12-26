pub(crate) mod service;
mod component;
pub(crate) mod resource;

use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use crate::cloud::aws::Services;
use crate::ui::component::TableList;

pub trait Ui<T> {
    fn ui<B>(&mut self, f: &mut Frame<B>, area: Rect, state: &mut T) -> anyhow::Result<()> where B: Backend;
}

pub(crate) fn ui<B: Backend>(f: &mut Frame<B>, app: &mut crate::app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());

    let header = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[0]);


    let title = Paragraph::new("")
        .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: true });

    f.render_widget(title, chunks[0]);

    match &mut app.service.current {
        None => {
            let mut tl = TableList {};
            tl.ui(f, chunks[1], &mut app.service.items);
        }
        Some(svc) => {
            svc.ui(f, chunks[1], &mut ());
        }
    }
}