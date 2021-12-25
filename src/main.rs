mod ui;
mod streams;
mod app;

use aws_sdk_kinesis::output::ListStreamsOutput;
use aws_sdk_kinesis::Client;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::FutureExt;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::{Receiver, Sender};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();

    let (tx, rx) = tokio::sync::mpsc::channel(1);
    let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel(1);

    let streams_handle = tokio::task::spawn(streams::Streams::new().await.run(tx, shutdown_tx.subscribe()));
    let app_handle = tokio::task::spawn(run_app(
        terminal,
        app,
        rx,
        shutdown_tx.clone(),
        shutdown_tx.subscribe(),
        tick_rate,
    ));
    let (_, res) = tokio::try_join!(streams_handle, app_handle)?;

    match res {
        Ok(mut terminal) => {
            // restore terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
        }
        Err(err) => {
            println!("{:?}", err)
        }
    }

    Ok(())
}
