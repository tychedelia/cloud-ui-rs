use std::any;
use tokio::time::{Duration, Instant};
use tui::backend::Backend;
use tui::Terminal;
use tui::widgets::ListState;
use crate::service::resource::{Resource, ResourceCrud};
use crate::service::{Service, ServiceType};
use crate::ui;

pub trait GetItems {
    fn get_items() -> Vec<String>;
}

pub struct StatefulEnum<T> {
    current: Option<T>,
    pub(crate) items: StatefulList<String>,
    state: ListState,
}

impl <T> StatefulEnum<T>
    where T: GetItems
{
    fn new() -> Self {
        Self {
            current: None,
            items: StatefulList::with_items(T::get_items()),
            state: ListState::default()
        }
    }
}

impl <T> StatefulEnum<T>
    where T: From<String>
{
    fn select(&mut self) {
        if let Some(idx) = self.state.selected() {
            let name = self.items.items[idx].clone();
            self.current = Some(T::from(name));
        }
    }
}

pub struct StatefulList<T> {
    pub(crate) state: ListState,
    pub(crate) items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub(crate) struct App {
    pub(crate) service: StatefulEnum<crate::cloud::aws::Services>
}

impl App {
    pub(crate) fn new() -> App {
        App {
            service: StatefulEnum::new()
        }
    }

    fn on_tick(&mut self) {
    }
}


pub(crate) async fn run_app<B: Backend>(
    mut terminal: Terminal<B>,
    mut app: App,
    mut rx: tokio::sync::mpsc::Receiver<Vec<String>>,
    mut shutdown_tx: tokio::sync::broadcast::Sender<()>,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
    tick_rate: Duration,
) -> anyhow::Result<Terminal<B>> {
    let mut last_tick = Instant::now();

    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                return Ok(terminal)
            }
            // items = rx.recv() => {
            //     if let Some(items) = items {
            //         // app.streams.items = items;
            //     }
            // },
            _ = futures::future::ready(()) => {
                terminal.draw(|f| crate::ui::ui(f, &mut app))?;

                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                if crossterm::event::poll(timeout)? {
                    if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                        match key.code {
                            crossterm::event::KeyCode::Char('q') => {
                                shutdown_tx.send(())?;
                                return Ok(terminal)
                            },
                            crossterm::event::KeyCode::Enter => app.service.select(),
                            crossterm::event::KeyCode::Left => app.service.items.unselect(),
                            crossterm::event::KeyCode::Down => app.service.items.next(),
                            crossterm::event::KeyCode::Up => app.service.items.previous(),
                            _ => {}
                        }
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    app.on_tick();
                    last_tick = Instant::now();
                }
            }
        }
    }
}

