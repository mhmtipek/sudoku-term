use std::{
    io,
    time::{Duration, Instant},
};
//use std::fmt;
use clap::{Parser, ValueEnum};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Cell, Row, Table, TableState},
    DefaultTerminal,
};
use std::thread;

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

    /// Show elapsed time
    #[arg(short, long, default_value_t = true)]
    show_elapsed_time: bool,
}

struct Board {
    rows: [[u8; 9]; 9],
    table_state: TableState,
    current_cell: (u8, u8),
    difficulty: Difficulty,
}

impl<'a> Board {
    fn new(difficulty: Difficulty) -> Self {
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
            .row_highlight_style(Style::new().fg(Color::Indexed(230)))
            .column_highlight_style(Style::new().fg(Color::Indexed(230)))
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

fn generate_initial_board(difficulty: Difficulty) -> [[u8; 9]; 9] {
    // Assign initial values. Maybe create available_values function
    // Solve board. Create solve function.
    // Remove cells until desired difficulty is reached.
    thread::sleep(Duration::from_millis(3000));
    let mut board: [[u8; 9]; 9] = [[3; 9]; 9];
    board
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = run(terminal, args.difficulty, args.show_elapsed_time);
    ratatui::restore();
    app_result
}

fn run(
    mut terminal: DefaultTerminal,
    difficulty: Difficulty,
    show_elapsed_time: bool,
) -> io::Result<()> {
    let mut board: Board = Board::new(difficulty);

    // Start the thread which creates the initial board here.
    let init_thread_handle = thread::spawn(move || generate_initial_board(difficulty));

    // The loop until initial board is created
    let mut counter = 0;
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
        })?;

        thread::sleep(Duration::from_millis(100));
        counter = counter + 1;
    }
    board.rows = init_thread_handle.join().unwrap();

    // The game loop
    let start_time = Instant::now();
    loop {
        terminal.draw(|frame| {
            let board_rect = Rect::new(
                (frame.area().width as u16 - 27) / 2,
                (frame.area().height as u16 - 9) / 2,
                27,
                9,
            );
            frame.render_stateful_widget(board.create_table(), board_rect, &mut board.table_state);
            if show_elapsed_time {
                let time_label =
                    Text::from(format!("{} secs", start_time.elapsed().as_secs())).right_aligned();
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
        })?;

        match event::poll(Duration::from_millis(200)) {
            Ok(true) => {
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
                        } else if key.code >= KeyCode::Char('1') && key.code <= KeyCode::Char('9') {
                            board.set_value(key.code.to_string().parse().unwrap())
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
