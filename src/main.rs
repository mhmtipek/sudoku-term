#![allow(non_snake_case)]

use std::io;
//use std::fmt;
use clap::Parser;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Cell, Row, Table, TableState},
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

struct Board {
    rows: [[u8; 9]; 9],
    table_state: TableState,
    current_cell: (u8, u8),
    difficulty: u8,
}

impl<'a> Board {
    fn new(difficulty: u8) -> Self {
        let current_cell = (0u8, 0u8);
        let mut table_state = TableState::default();
        table_state.select_cell(Some((0, 0)));
        table_state.select(Some(0));
        table_state.select_column(Some(0));
        let rows: [[u8; 9]; 9] = [[2; 9]; 9];
        Self {
            rows,
            current_cell,
            table_state,
            difficulty,
        }
    }

    fn create_table(&self) -> Table<'a> {
        let mut rows: Vec<Row> = Vec::with_capacity(9);
        for row in 0..9 {
            let mut cells: Vec<Cell> = Vec::with_capacity(9);
            for col in 0..9 {
                let bg_color = {
                    let is_cell_darker = (row % 2) ^ (col % 2) == 0;
                    let is_rect_darker = ((row / 3) % 2) ^ ((col / 3) % 2) == 0;
                    if is_cell_darker {
                        if is_rect_darker {
                            Color::Indexed(244)
                        } else {
                            Color::Indexed(250)
                        }
                    } else {
                        if is_rect_darker {
                            Color::Indexed(246)
                        } else {
                            Color::Indexed(252)
                        }
                    }
                };
                cells.insert(
                    col,
                    Cell::from(Text::from(format!("{}", self.rows[row][col])).centered())
                        .bg(bg_color)
                        .fg(Color::Black),
                );
            }
            rows.insert(row, Row::new(cells));
        }
        let widths = (0..9).map(|_| 3);
        Table::new(rows, widths)
            .column_spacing(0)
            .bg(Color::Indexed(0))
            .row_highlight_style(Style::new().fg(Color::Indexed(210)))
            .column_highlight_style(Style::new().fg(Color::Indexed(210)))
            .cell_highlight_style(Style::new().fg(Color::Indexed(120)))
    }

    fn set_current(&mut self, row: u8, col: u8) {
        self.current_cell = (row, col);
        self.table_state
            .select_cell(Some((row as usize, col as usize)));
        self.table_state.select(Some(row as usize));
        self.table_state.select_column(Some(col as usize));
    }

    fn set_value(&mut self, val: u8) {
        self.rows[self.current_cell.0 as usize][self.current_cell.1 as usize] = val;
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    print!("Difficulty: {}\n", args.difficulty);
    print!("showElapsedTime: {}\n", args.showElapsedTime);

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = run(terminal, args.difficulty, args.showElapsedTime);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal, difficulty: u8, showElapsedTime: bool) -> io::Result<()> {
    let mut board: Board = Board::new(difficulty);
    loop {
        terminal.draw(|frame| {
            frame.render_stateful_widget(
                board.create_table(),
                frame.area(),
                &mut board.table_state,
            );
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                } else if key.code == KeyCode::Right {
                    board.set_current(board.current_cell.0, (board.current_cell.1 + 1) % 9);
                } else if key.code == KeyCode::Left {
                    if board.current_cell.1 == 0 {
                        board.set_current(board.current_cell.0, 8);
                    } else {
                        board.set_current(board.current_cell.0, board.current_cell.1 - 1);
                    }
                } else if key.code == KeyCode::Up {
                    if board.current_cell.0 == 0 {
                        board.set_current(8, board.current_cell.1);
                    } else {
                        board.set_current(board.current_cell.0 - 1, board.current_cell.1);
                    }
                } else if key.code == KeyCode::Down {
                    board.set_current((board.current_cell.0 + 1) % 9, board.current_cell.1);
                } else if key.code >= KeyCode::Char('0') && key.code <= KeyCode::Char('9') {
                    board.set_value(key.code.to_string().parse().unwrap())
                }
            }
        }
    }
}
