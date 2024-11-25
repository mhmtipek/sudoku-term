use std::io;
use clap::Parser;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
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

fn main() { // -> io::Result<()> {
//    let mut terminal = ratatui::init();
//    terminal.clear()?;

    let args = Args::parse();

    print!("Difficulty: {}\n", args.difficulty);
    print!("showElapsedTime: {}\n", args.showElapsedTime);

//    let app_result = run(terminal);
    ratatui::restore();
//    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
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
