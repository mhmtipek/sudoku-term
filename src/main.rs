use std::{
    io,
    time::{Duration, Instant},
};
//use std::fmt;
use clap::{Parser, ValueEnum};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::Rect,
    style::{Color, Stylize},
    text::Text,
    widgets::{Cell, Row, Table},
    DefaultTerminal,
};
use std::thread;

pub mod sudoku;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Difficulty {
    // Easy
    Easy,
    // Medium
    Medium,
    // Hard
    Hard,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Difficulty
    #[arg(value_enum, default_value_t = Difficulty::Medium)]
    difficulty: Difficulty,

    /// Hide elapsed time
    #[arg(long, default_value_t = false)]
    hide_elapsed_time: bool,
}

#[derive(Copy, Clone)]
struct CellData {
    editable: bool,
    conflict: bool,
    highlight: bool,
}

struct Board {
    rows: [[u8; 9]; 9],
    cell_data: [[CellData; 9]; 9],
    current_cell: (u8, u8),
    difficulty: Difficulty,
}

impl<'a> Board {
    fn new(difficulty: Difficulty) -> Self {
        let current_cell = (0u8, 0u8);
        let rows: [[u8; 9]; 9] = [[2; 9]; 9];
        let cell_data: [[CellData; 9]; 9] = [([CellData {
            conflict: false,
            editable: false,
            highlight: false,
        }; 9]); 9];
        Self {
            rows,
            cell_data,
            current_cell,
            difficulty,
        }
    }

    fn create_table(&self) -> Table<'a> {
        let mut rows: Vec<Row> = Vec::with_capacity(9);
        let finished = sudoku::sudoku::is_finished(&self.rows);
        for row in 0..9 {
            let mut cells: Vec<Cell> = Vec::with_capacity(9);
            for col in 0..9 {
                let bg_color = {
                    if row == self.current_cell.0 && col == self.current_cell.1 {
                        Color::Indexed(180)
                    } else if self.cell_data[row as usize][col as usize].conflict {
                        Color::Indexed(162)
                    } else {
                        let is_cell_darker = (row % 2) ^ (col % 2) == 0;
                        let is_rect_darker = ((row / 3) % 2) ^ ((col / 3) % 2) == 0;
                        if is_cell_darker {
                            if is_rect_darker {
                                if self.current_cell.0 == row || self.current_cell.1 == col {
                                    Color::Indexed(241)
                                } else {
                                    Color::Indexed(240)
                                }
                            } else {
                                if self.current_cell.0 == row || self.current_cell.1 == col {
                                    Color::Indexed(245)
                                } else {
                                    Color::Indexed(244)
                                }
                            }
                        } else {
                            if is_rect_darker {
                                if self.current_cell.0 == row || self.current_cell.1 == col {
                                    Color::Indexed(243)
                                } else {
                                    Color::Indexed(242)
                                }
                            } else {
                                if self.current_cell.0 == row || self.current_cell.1 == col {
                                    Color::Indexed(247)
                                } else {
                                    Color::Indexed(246)
                                }
                            }
                        }
                    }
                };
                let fg_color = {
                    if finished {
                        Color::Indexed(155)
                    } else if row == self.current_cell.0 && col == self.current_cell.1 {
                        if self.cell_data[row as usize][col as usize].editable {
                            Color::Indexed(123)
                        } else {
                            Color::Black
                        }
                    } else if self.cell_data[row as usize][col as usize].highlight {
                        Color::Indexed(230)
                    } else {
                        Color::Black
                    }
                };
                let mut char = String::from(" ");
                if self.rows[row as usize][col as usize] > 0 {
                    char = format!("{}", self.rows[row as usize][col as usize]);
                } else if row as u8 == self.current_cell.0 && col as u8 == self.current_cell.1 {
                    char = String::from("_");
                }
                cells.insert(
                    col as usize,
                    Cell::from(Text::from(char).centered())
                        .bg(bg_color)
                        .fg(fg_color),
                );
            }
            rows.insert(row as usize, Row::new(cells));
        }
        let widths = [3; 9];
        Table::new(rows, widths)
            .column_spacing(0)
            .bg(Color::Indexed(0))
    }

    fn set_current(&mut self, row: u8, col: u8) {
        self.current_cell = (row, col);
        self.update_cell_data();
    }

    fn update_cell_data(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                self.cell_data[row as usize][col as usize].conflict =
                    !sudoku::sudoku::is_valid(&self.rows, row, col);
                self.cell_data[row as usize][col as usize].highlight = sudoku::sudoku::are_related(
                    (self.current_cell.0, self.current_cell.1),
                    (row, col),
                );
            }
        }
    }

    fn set_value(&mut self, val: u8) {
        self.rows[self.current_cell.0 as usize][self.current_cell.1 as usize] = val;
        self.update_cell_data();
    }

    fn set_initial_rows(&mut self, rows: [[u8; 9]; 9]) {
        self.rows = rows;

        // Init cell data
        for row in 0..9 {
            for col in 0..9 {
                self.cell_data[row][col].editable = self.rows[row][col] == 0;
            }
        }
        self.update_cell_data();
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = run(terminal, args.difficulty, args.hide_elapsed_time);
    ratatui::restore();
    app_result
}

fn run(
    mut terminal: DefaultTerminal,
    difficulty: Difficulty,
    hide_elapsed_time: bool,
) -> io::Result<()> {
    let mut board: Board = Board::new(difficulty);

    let difficulty_val = match difficulty {
        Difficulty::Easy => 100,
        Difficulty::Medium => 140,
        Difficulty::Hard => 160,
    };

    // Start the thread which creates the initial board here.
    let init_thread_handle =
        thread::spawn(move || sudoku::sudoku::generate_initial_board(difficulty_val));

    // The loop until initial board is created
    let mut counter = 0;
    let board_generation_start_time = Instant::now();
    loop {
        if init_thread_handle.is_finished() {
            break;
        }

        // Animate text
        let text = "Generating board ... ";
        let shift = counter % text.len();
        let print_text = format!("{}{}", &text[shift..], &text[0..shift]);

        terminal.draw(|frame| {
            frame.render_widget(
                Text::from(print_text).centered(),
                Rect::new(0, frame.area().height / 2, frame.area().width, 1),
            );
            if board_generation_start_time.elapsed() > Duration::from_secs(10) {
                frame.render_widget(
                    Text::from("It may take time depending on difficulty").centered(),
                    Rect::new(0, frame.area().height - 3, frame.area().width, 1),
                );
            }
        })?;

        thread::sleep(Duration::from_millis(100));
        counter = counter + 1;
    }
    board.set_initial_rows(init_thread_handle.join().unwrap());

    // The game loop
    let start_time = Instant::now();
    let mut finish_time = start_time;
    let mut finished = false;
    let mut undo_data: Option<(u8, u8, u8)> = Option::None; // row, col, val
    loop {
        terminal.draw(|frame| {
            let board_rect = Rect::new(
                (frame.area().width as u16 - 27) / 2,
                (frame.area().height as u16 - 9) / 2,
                27,
                9,
            );
            frame.render_widget(board.create_table(), board_rect);
            if !hide_elapsed_time {
                let secs = {
                    if start_time == finish_time {
                        start_time.elapsed().as_secs()
                    } else {
                        finish_time.duration_since(start_time).as_secs()
                    }
                };
                let mut time_label = Text::from(format!("{} secs", secs)).right_aligned();
                if secs >= 60 {
                    if secs >= 120 {
                        time_label = Text::from(format!("{} mins {} secs", secs / 60, secs % 60))
                            .right_aligned();
                    } else {
                        time_label =
                            Text::from(format!("1 min {} secs", secs % 60)).right_aligned();
                    }
                }
                frame.render_widget(
                    time_label,
                    Rect::new(
                        frame.area().width / 2,
                        frame.area().height - 1,
                        frame.area().width / 2,
                        1,
                    ),
                );
            }
            let difficulty_label = Text::from(format!("{:?}", board.difficulty)).left_aligned();
            frame.render_widget(
                difficulty_label,
                Rect::new(0, frame.area().height - 1, frame.area().width / 2, 1),
            );
            frame.render_widget(
                Text::from("d: delete, u: undo, q: quit").centered(),
                Rect::new(0, frame.area().height - 1, frame.area().width, 1),
            );
        })?;

        match event::poll(Duration::from_millis(200)) {
            Ok(true) => {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if key.code == KeyCode::Char('q') {
                            return Ok(());
                        } else if finished {
                            continue;
                        } else if key.code == KeyCode::Char('u') {
                            match undo_data {
                                Some(data) => {
                                    board.set_current(data.0, data.1);
                                    board.set_value(data.2);
                                }
                                None => {}
                            }
                            undo_data = Option::None;
                        } else if key.code == KeyCode::Char('d') {
                            if board.cell_data[board.current_cell.0 as usize]
                                [board.current_cell.1 as usize]
                                .editable
                            {
                                board.set_value(0);
                            }
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
                        } else if key.code >= KeyCode::Char('1') && key.code <= KeyCode::Char('9') {
                            if board.cell_data[board.current_cell.0 as usize]
                                [board.current_cell.1 as usize]
                                .editable
                            {
                                undo_data = Some((
                                    board.current_cell.0,
                                    board.current_cell.1,
                                    board.rows[board.current_cell.0 as usize]
                                        [board.current_cell.1 as usize],
                                ));
                                board.set_value(key.code.to_string().parse().unwrap());
                                finished = sudoku::sudoku::is_finished(&board.rows);
                                if finished {
                                    finish_time = Instant::now();
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
