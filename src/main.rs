mod scramble;
mod stopwatch;
use crate::scramble::scramble;
use crate::stopwatch::StopWatch;
use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;
use std::{thread, time};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame, Terminal,
};

struct App {
    started: bool,
    stopwatch: StopWatch,
    display: String,
    scramble: Vec<&'static str>,
}

fn stopwatch(app: &mut App) {
    let sw = StopWatch::start();
    app.stopwatch = sw;
    thread::sleep(time::Duration::from_millis(100));
}

fn end(app: &mut App) -> Option<Duration> {
    let sw = &app.stopwatch;
    sw.duration()
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app: App = App {
        started: false,
        stopwatch: StopWatch { time_started: None },
        display: String::from("Press spacebar to start and stop."),
        scramble: scramble(),
    };

    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if poll(time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                } else if let KeyCode::Char(' ') = key.code {
                    if app.started {
                        app.started = false;
                        let duration = end(&mut app);
                        match duration {
                            Some(dur) => {
                                app.scramble = scramble();
                                app.display = String::from(format!("{:?}", dur));
                            }
                            _ => (),
                        }
                    } else {
                        app.display =
                            String::from(format!("{:?}", app.stopwatch.duration().unwrap()));
                        app.started = true;
                        stopwatch(&mut app);
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    if app.started {
        app.display = String::from(format!("{:?}", app.stopwatch.duration().unwrap()))
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(f.size());

    let paragraph = Paragraph::new(&*app.display).alignment(Alignment::Center);
    let paragraph2 =
        Paragraph::new(format!("{:?}", app.scramble.join(", "))).alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[2]);
    f.render_widget(paragraph2, chunks[1]);
}
