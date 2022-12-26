use crate::App;
use tui::{
    backend::{Backend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, Borders},
    Frame, Terminal,
};
use std::{time, collections::HashMap};
use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


pub fn make_chart<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {

    let mut data: Vec<(&str, u64)> = vec![];

    for i in &app.time_data {
        data.push(("seconds", *i));
    }

    data.reverse();

   terminal.draw(|f| {
          let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

         let barchart = BarChart::default()
        .block(Block::default().title("Chart").borders(Borders::ALL))
        .bar_width(3)
        .bar_style(Style::default().fg(Color::Yellow))
            .data(&data)
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    f.render_widget(barchart, chunks[0]);
    });

            match event::read() {
    Ok(Event::Key(KeyEvent { code: KeyCode::Char('s' | 'q'), .. })) => {
                app.show_chart = false;
             },
       _ => ()
}
    ()
}
