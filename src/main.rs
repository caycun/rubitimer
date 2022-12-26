mod barchart;
mod scramble;
mod stopwatch;
use crate::barchart::make_chart;
use crate::scramble::scramble;
use crate::stopwatch::StopWatch;
use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::{thread, time};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame, Terminal,
};

pub struct App {
    started: bool,
    stopwatch: StopWatch,
    display: String,
    scramble: Vec<&'static str>,
    time_data: Vec<u64>,
    show_chart: bool
}

fn stopwatch(app: &mut App) {
    let sw = StopWatch::start();
    app.stopwatch = sw;
    thread::sleep(time::Duration::from_millis(100));
}

fn end(app: &mut App) -> Option<u64> {
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
        time_data: vec![],
        show_chart: false
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
        while app.show_chart {
        make_chart(terminal, &mut app);
        }
        terminal.draw(|f| ui(f, &mut app))?;
        if poll(time::Duration::from_millis(100))? {
            match event::read()? {
    Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => {
        return Ok(());
    },
    Event::Key(KeyEvent { code: KeyCode::Char('s'), .. }) => {
                app.show_chart = true;
    },
    Event::Key(KeyEvent { code: KeyCode::Char(' '), .. }) => {
        if app.started {
            app.started = false;
            let duration = end(&mut app);
            match duration {
                Some(dur) => {
                    app.scramble = scramble();
                    app.display = format!("{:?}", dur);
                    app.time_data.insert(0, dur);
                }
                _ => (),
            }
        } else {
            app.display = format!("{:?}", app.stopwatch.duration().unwrap());
            app.started = true;
            stopwatch(&mut app);
        }
    },
    _ => ()
}
                   }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    if app.started {
        app.display = format!("{:?}", app.stopwatch.duration().unwrap())
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(vec![Constraint::Percentage(25); 5])
        .split(f.size());

    let paragraph = Paragraph::new(&*app.display).alignment(Alignment::Center);
    let paragraph2 =
        Paragraph::new(format!("{:?}", app.scramble.join(", "))).alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[2]);
    f.render_widget(paragraph2, chunks[1]);
}
