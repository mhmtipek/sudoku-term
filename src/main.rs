#![allow(non_snake_case)]

use std::io;
use std::fmt;
use clap::Parser;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    text::{Line, Text},
    widgets::Paragraph,
    DefaultTerminal,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Difficulty
    #[arg(short, long, default_value_t = 4)]
    difficulty: u8,
        
    /// Show elapsed time
    #[arg(short, long)]
    showElapsedTime: bool,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let args = Args::parse();

    print!("Difficulty: {}\n", args.difficulty);
    print!("showElapsedTime: {}\n", args.showElapsedTime);

    let app_result = run(terminal, args.difficulty, args.showElapsedTime);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal, difficulty: u8, showElapsedTime: bool) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greetingText = Text::from(vec![
                Line::from(vec![ format!("Difficulty: {}", difficulty).into() ]),
                Line::from(vec![ format!("Show Elapsed Time: {}", showElapsedTime).yellow() ]),
            ]);
            let greeting = Paragraph::new(greetingText)
                .white()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
