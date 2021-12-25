
struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
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
struct App<'a> {
    streams: StatefulList<String>,
    events: Vec<(&'a str, &'a str)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            streams: StatefulList::with_items(vec![]),
            events: vec![
                ("Event1", "INFO"),
                ("Event2", "INFO"),
                ("Event3", "CRITICAL"),
                ("Event4", "ERROR"),
                ("Event5", "INFO"),
                ("Event6", "INFO"),
                ("Event7", "WARNING"),
                ("Event8", "INFO"),
                ("Event9", "INFO"),
                ("Event10", "INFO"),
                ("Event11", "CRITICAL"),
                ("Event12", "INFO"),
                ("Event13", "INFO"),
                ("Event14", "INFO"),
                ("Event15", "INFO"),
                ("Event16", "INFO"),
                ("Event17", "ERROR"),
                ("Event18", "ERROR"),
                ("Event19", "INFO"),
                ("Event20", "INFO"),
                ("Event21", "WARNING"),
                ("Event22", "INFO"),
                ("Event23", "INFO"),
                ("Event24", "WARNING"),
                ("Event25", "INFO"),
                ("Event26", "INFO"),
            ],
        }
    }

    /// Rotate through the event list.
    /// This only exists to simulate some kind of "progress"
    fn on_tick(&mut self) {
        let event = self.events.remove(0);
        self.events.push(event);
    }
}


async fn run_app<B: Backend>(
    mut terminal: Terminal<B>,
    mut app: App<'static>,
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
            items = rx.recv() => {
                if let Some(items) = items {
                    app.streams.items = items;
                }
            },
            _ = futures::future::ready(()) => {
                terminal.draw(|f| ui::ui(f, &mut app))?;

                // let items = rx.recv().await;

                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                if crossterm::event::poll(timeout)? {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Char('q') => {
                                shutdown_tx.send(())?;
                                return Ok(terminal)
                            },
                            KeyCode::Left => app.streams.unselect(),
                            KeyCode::Down => app.streams.next(),
                            KeyCode::Up => app.streams.previous(),
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

