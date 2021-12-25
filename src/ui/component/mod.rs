mod table;
mod header;

use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{List, ListItem};
use crate::app::StatefulList;
use tui::text::{Span, Spans};
use crate::ui;

pub struct TableList {
}


impl ui::Ui<StatefulList<String>> for TableList {
    fn ui<B>(&mut self, f: &mut Frame<B>, area: Rect, state: &mut StatefulList<String>) -> anyhow::Result<()>
        where B: Backend
    {
        let items: Vec<ListItem> = state
            .items
            .iter()
            .map(|r| {
                let mut lines = vec![Spans::from(r.clone())];
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        let table = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(table, area, &mut state.state);
        Ok(())
    }
}

