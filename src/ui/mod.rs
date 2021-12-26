pub(crate) mod service;
mod component;
pub(crate) mod resource;

use std::io::Stdout;
use tui::backend::{Backend, CrosstermBackend};
use tui::Frame;
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use crate::app::AppState;
use crate::cloud::aws::Services;
use crate::ui::component::TableList;

pub trait Ui<T> {
    fn ui<B>(&mut self, f: &mut Frame<B>, area: Rect, state: &mut T) -> anyhow::Result<()> where B: Backend;
}

pub(crate) fn ui<B: Backend, Svc, Res>(f: &mut Frame<B>, app: &mut crate::app::App<Svc, Res>) -> anyhow::Result<()> {
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

    match &mut app.state {
        AppState::Services { service } => {
            let mut tl = TableList {};
            tl.ui(f, chunks[1], &mut service.items)?;
        }
        AppState::Resources { resources } => {
            let mut tl = TableList {};
            tl.ui(f, chunks[1], &mut resources.items)?;
        }
    };

    Ok(())
}