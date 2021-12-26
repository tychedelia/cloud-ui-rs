use std::any;
use std::io::Stdout;
use tokio::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use tui::widgets::ListState;
use crate::service::resource::{Resource, ResourceController, ResourceKind};
use crate::service::{Service, ServiceKind, ServiceType};
use crate::ui;
use crate::ui::service::ServiceState;

pub trait GetItems {
    fn get_items() -> Vec<String>;
}

pub struct StatefulEnum<T> {
    pub current: Option<T>,
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
        if let Some(idx) = self.items.state.selected() {
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
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
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
pub(crate) struct App<Svc, Res> {
    pub(crate) state: AppState<Svc, Res>
}

pub(crate) enum AppState<Svc, Res> {
    Services { service: StatefulEnum<Svc> },
    Resources { resources: StatefulEnum<Res> },
}

impl <Svc, Res> App<Svc, Res>
    where Svc: ServiceKind,
          Res: ResourceKind,
{
    pub(crate) fn new() -> App<Svc, Res> {
        App {
            state: AppState::Services { service: StatefulEnum::new() }
        }
    }

    fn on_esc(&mut self) {
        match self.state {
            AppState::Services { .. } => {}
            AppState::Resources { .. } => {
                self.state = AppState::Services { service: StatefulEnum::new() }
            }
        }
    }

    fn on_up(&mut self) {
        match &mut self.state {
            AppState::Services {service } => {
                service.items.next();
            }
            AppState::Resources { resources } => {
                resources.items.next()
            }
        }
    }

    fn on_down(&mut self) {
        match &mut self.state {
            AppState::Services {service } => {
                service.items.previous();
            }
            AppState::Resources { resources } => {
                resources.items.previous()
            }
        }
    }

    fn on_select(&mut self) {
        match &mut self.state {
            AppState::Services { service } => {
                service.select();
                self.state = AppState::Resources { resources: StatefulEnum::new() }
            }
            AppState::Resources { resources } => {
                // resources.select()
            }
        }
    }

    fn on_unselect(&mut self) {
        match &mut self.state {
            AppState::Services {service } => {
                service.items.unselect();
            }
            AppState::Resources { resources } => {
                resources.items.unselect()
            }
        }
    }

    fn on_tick(&mut self) {
    }
}


pub(crate) async fn run_app<B, Svc, Res>(
    mut terminal: Terminal<B>,
    mut app: App<Svc, Res>,
    mut rx: tokio::sync::mpsc::Receiver<Vec<String>>,
    mut shutdown_tx: tokio::sync::broadcast::Sender<()>,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
    tick_rate: Duration,
) -> anyhow::Result<Terminal<B>>
    where B: Backend,
          Svc: ServiceKind,
          Res: ResourceKind,
{
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
                terminal.draw(|f| crate::ui::ui(f, &mut app).unwrap())?;

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
                            crossterm::event::KeyCode::Esc => app.on_esc(),
                            crossterm::event::KeyCode::Char('j') => app.on_up(),
                            crossterm::event::KeyCode::Char('k') => app.on_down(),
                            crossterm::event::KeyCode::Char('h') => app.on_unselect(),
                            crossterm::event::KeyCode::Enter => app.on_select(),
                            crossterm::event::KeyCode::Left => app.on_unselect(),
                            crossterm::event::KeyCode::Up => app.on_up(),
                            crossterm::event::KeyCode::Down => app.on_down(),
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

