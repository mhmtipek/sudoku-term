pub mod sudoku {
    use rand::prelude::*;
    use std::boxed::Box;
    use std::collections::HashSet;
    use std::collections::LinkedList;
    use std::thread;

    #[derive(Debug)]
    struct Node {
        row: u8,
        col: u8,
        board: [[u8; 9]; 9],
    }

    impl Node {
        fn new(row: u8, col: u8, board: [[u8; 9]; 9]) -> Self {
            Self { row, col, board }
        }
    }

    impl Clone for Node {
        fn clone(&self) -> Self {
            let row = self.row;
            let col = self.col;
            let board = self.board.clone();
            Self { row, col, board }
        }
    }

    // TODO: write unit test
    pub fn available_values(board: &[[u8; 9]; 9], row: u8, col: u8) -> Vec<u8> {
        let val = board[row as usize][col as usize];
        // Return current value in case it's non zero
        if val > 0 {
            return Vec::from([val]);
        }

        // Note that value zero means empty
        let mut values = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        // Check same row
        for i in 0..9 {
            values.remove(&board[row as usize][i]);
        }
        // Check same column
        for i in 0..9 {
            values.remove(&board[i][col as usize]);
        }
        // Check same rect
        for r in ((row / 3) * 3)..((row / 3) * 3 + 3) {
            for c in ((col / 3) * 3)..((col / 3) * 3 + 3) {
                values.remove(&board[r as usize][c as usize]);
            }
        }

        values.into_iter().collect()
    }

    // TODO: write unit test
    pub fn is_valid(board: &[[u8; 9]; 9], row: u8, col: u8) -> bool {
        let value = board[row as usize][col as usize];

        // Check same col
        for row_i in 0..9 {
            if row_i == row {
                continue;
            }
            if board[row_i as usize][col as usize] == value {
                return false;
            }
        }

        // Check same row
        for col_i in 0..9 {
            if col_i == col {
                continue;
            }
            if board[row as usize][col_i as usize] == value {
                return false;
            }
        }

        // Check same rect
        for row_i in ((row / 3) * 3)..((row / 3) * 3 + 3) {
            for col_i in ((col / 3) * 3)..((col / 3) * 3 + 3) {
                if board[row_i as usize][col_i as usize] == value {
                    return false;
                }
            }
        }

        true
    }

    // TODO: write unit test
    pub fn is_finished(board: &[[u8; 9]; 9]) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if !is_valid(board, row, col) {
                    return false;
                }
            }
        }

        true
    }

    // Performs depth first search on node
    // @return solutions
    fn search(node: &mut Box<Node>) -> Vec<[[u8; 9]; 9]> {
        let mut all_nodes: Vec<[[u8; 9]; 9]> = Vec::new();

        // Use linked list as stack because we'll be worknig with last element all the time
        let mut nodes_stack: LinkedList<Box<Node>> = LinkedList::new();
        for child_node in create_children(node) {
            nodes_stack.push_back(child_node);
        }

        while !nodes_stack.is_empty() {
            let node = nodes_stack.pop_back().unwrap();

            if node.row == 8 {
                if node.col == 8 {
                    all_nodes.push(node.board);
                    continue;
                }
            }

            for child_node in create_children(&node) {
                nodes_stack.push_back(child_node);
            }
        }

        all_nodes
    }

    fn create_children(node: &Box<Node>) -> Vec<Box<Node>> {
        let child_row;
        let child_col;

        if node.col == 8 {
            if node.row == 8 {
                return Vec::new(); // We reached end of the board
            }
            child_row = node.row + 1;
            child_col = 0;
        } else {
            child_row = node.row;
            child_col = node.col + 1;
        }

        let mut all_nodes: Vec<Box<Node>> = Vec::new();

        for val in available_values(&node.board, child_row, child_col) {
            let mut child_board = node.board.clone();
            child_board[child_row as usize][child_col as usize] = val;
            all_nodes.push(Box::new(Node::new(child_row, child_col, child_board)));
        }

        all_nodes
    }

    fn all_solutions(board: &[[u8; 9]; 9]) -> Vec<[[u8; 9]; 9]> {
        let mut solutions: Vec<[[u8; 9]; 9]> = Vec::new();

        // Create initial nodes. Ideally it'll be one for each thread but
        // it can be a little more, depending on available values for a cell
        let ideal_thread_count = thread::available_parallelism().unwrap().get();

        let mut head_nodes: Vec<Box<Node>> = Vec::new();

        for val in available_values(&board, 0, 0) {
            let mut copy_board = board.clone();
            copy_board[0][0] = val;
            head_nodes.push(Box::new(Node::new(0, 0, copy_board)));
        }

        let mut c = 0;
        while head_nodes.len() < ideal_thread_count {
            if c == 30 {
                // If it takes too long to reach, just break
                break;
            }
            let node;
            match head_nodes.pop() {
                Some(_node) => node = _node,
                None => {
                    break;
                }
            }
            if node.row == 8 && node.col == 8 {
                solutions.push(node.board.clone());
            }
            let mut child_nodes = create_children(&node);
            head_nodes.append(&mut child_nodes);
            c = c + 1;
        }

        let mut join_handles: Vec<thread::JoinHandle<Vec<[[u8; 9]; 9]>>> = Vec::new();
        while !head_nodes.is_empty() {
            let mut node = head_nodes.pop().unwrap();
            join_handles.push(thread::spawn(move || search(&mut node)));
        }

        for handle in join_handles {
            match handle.join() {
                Ok(boards) => {
                    let mut mut_boards = boards.clone();
                    solutions.append(&mut mut_boards);
                }
                Err(_) => {
                    eprintln!("Thread failed")
                }
            }
        }

        solutions
    }

    // TODO: write unit test
    // difficulty is in between 0-255
    pub fn generate_initial_board(difficulty: u8) -> [[u8; 9]; 9] {
        let mut solutions: Vec<[[u8; 9]; 9]> = Vec::new();

        let mut try_count = 1;
        let mut rng = rand::rng();
        while solutions.is_empty() {
            // Value 0 (zero) means cell is empty
            let mut board: [[u8; 9]; 9] = [[0; 9]; 9];

            // Assign random but valid initial values
            let mut all_indexes = [0; 81];
            for i in 1..81 {
                all_indexes[i] = i;
            }
            all_indexes.shuffle(&mut rng);
            // 30 is magic number, an optimized value
            for i in 0..30 {
                let row = all_indexes[i] / 9;
                let col = all_indexes[i] % 9;
                let available_values = available_values(&board, row as u8, col as u8);
                if available_values.is_empty() {
                    continue;
                }
                let index = rng.random::<u8>() % available_values.len() as u8;
                board[row][col] = available_values[index as usize];
            }

            solutions = all_solutions(&board);
            try_count = try_count + 1;
        }

        *solutions.first().unwrap()

        //
        // let mut join_handles: Vec<thread::JoinHandle<[[u8; 9]; 9]>> = Vec::new();
        // for solved_board in solutions {
        //     join_handles.push(thread::spawn(move || {
        //         adjust_difficulty(solved_board, difficulty)
        //     }));
        // }

        // let mut game_boards: Vec<[[u8; 9]; 9]> = Vec::new();
        // for handle in join_handles {
        //     match handle.join() {
        //         Ok(board) => {
        //             game_boards.push(board);
        //         }
        //         Err(_) => {
        //             eprintln!("Thread failed");
        //         }
        //     }
        // }

        // std::thread::sleep(Duration::from_millis(3000));

        // board
    }
}
