use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::{self, Write},
};

use nom::{
    IResult, Parser,
    bytes::complete::take,
    character::complete::line_ending,
    combinator::{all_consuming, opt},
    multi::{count, separated_list1},
};

use z3::{
    Solver,
    ast::{Ast, Int},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{self, ClearType},
};

struct SudokuInput {
    grid: [[Option<u8>; 9]; 9],
    cursor_row: usize,
    cursor_col: usize,
}

impl SudokuInput {
    fn new() -> Self {
        Self {
            grid: [[None; 9]; 9],
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    fn display(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        queue!(
            stdout,
            Print("╔═══════════════════════════╗\r\n"),
            Print("║   SUDOKU PUZZLE INPUT     ║\r\n"),
            Print("╚═══════════════════════════╝\r\n\r\n"),
        )?;

        queue!(stdout, Print("  A B C   D E F   G H I\r\n"))?;
        queue!(stdout, Print("┌───────┬───────┬───────┐\r\n"))?;

        for row in 0..9 {
            if row == 3 || row == 6 {
                queue!(stdout, Print("├───────┼───────┼───────┤\r\n"))?;
            }

            queue!(stdout, Print("│ "))?;

            for col in 0..9 {
                if col == 3 || col == 6 {
                    queue!(stdout, Print("│ "))?;
                }

                // Highlight cursor position
                if row == self.cursor_row && col == self.cursor_col {
                    queue!(
                        stdout,
                        SetBackgroundColor(Color::White),
                        SetForegroundColor(Color::Black)
                    )?;
                }

                match self.grid[row][col] {
                    Some(n) => queue!(stdout, Print(n))?,
                    None => queue!(stdout, Print('.'))?,
                }

                if row == self.cursor_row && col == self.cursor_col {
                    queue!(stdout, ResetColor)?;
                }

                queue!(stdout, Print(' '))?;
            }

            queue!(stdout, Print(format!("│ {}\r\n", row)))?;
        }

        queue!(stdout, Print("└───────┴───────┴───────┘\r\n\r\n"))?;

        queue!(
            stdout,
            Print("Controls:\r\n"),
            Print("  Arrow Keys / WASD: Move cursor\r\n"),
            Print("  1-9: Enter number\r\n"),
            Print("  0 / Space / Backspace: Clear cell\r\n"),
            Print("  Q / Esc: Quit and show result\r\n"),
            Print("  R: Reset grid\r\n"),
            Print(format!(
                "\r\nCursor: Row {}, Col {}\r\n",
                (b'A' + self.cursor_row as u8) as char,
                self.cursor_col + 1
            )),
        )?;

        stdout.flush()?;
        Ok(())
    }

    fn move_cursor(&mut self, dr: i32, dc: i32) {
        let new_row = (self.cursor_row as i32 + dr).rem_euclid(9) as usize;
        let new_col = (self.cursor_col as i32 + dc).rem_euclid(9) as usize;
        self.cursor_row = new_row;
        self.cursor_col = new_col;
    }

    fn set_value(&mut self, val: Option<u8>) {
        self.grid[self.cursor_row][self.cursor_col] = val;
    }

    fn reset(&mut self) {
        self.grid = [[None; 9]; 9];
    }

    fn to_array(&self) -> [[u8; 9]; 9] {
        let mut result = [[0u8; 9]; 9];
        for (row, result_row) in result.iter_mut().enumerate() {
            for (col, result_cell) in result_row.iter_mut().enumerate() {
                *result_cell = self.grid[row][col].unwrap_or(0);
            }
        }
        result
    }
}

fn run() -> io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut sudoku = SudokuInput::new();
    let mut quit = false;

    while !quit {
        sudoku.display()?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    quit = true;
                }
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                    sudoku.move_cursor(-1, 0);
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                    sudoku.move_cursor(1, 0);
                }
                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                    sudoku.move_cursor(0, -1);
                }
                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                    sudoku.move_cursor(0, 1);
                }
                KeyCode::Char('1') => sudoku.set_value(Some(1)),
                KeyCode::Char('2') => sudoku.set_value(Some(2)),
                KeyCode::Char('3') => sudoku.set_value(Some(3)),
                KeyCode::Char('4') => sudoku.set_value(Some(4)),
                KeyCode::Char('5') => sudoku.set_value(Some(5)),
                KeyCode::Char('6') => sudoku.set_value(Some(6)),
                KeyCode::Char('7') => sudoku.set_value(Some(7)),
                KeyCode::Char('8') => sudoku.set_value(Some(8)),
                KeyCode::Char('9') => sudoku.set_value(Some(9)),
                KeyCode::Char('0') | KeyCode::Char(' ') | KeyCode::Backspace | KeyCode::Delete => {
                    sudoku.set_value(None);
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    sudoku.reset();
                }
                _ => {}
            }
        }
    }

    // Cleanup
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // // Show final grid
    // println!("\nFinal Sudoku Grid (0 = empty):");
    // let grid = sudoku.to_array();
    // for row in grid.iter() {
    //     println!("{:?}", row);
    // }

    let mut puzzle = Puzzle::from_array(&sudoku.to_array());

    println!("{puzzle}");

    puzzle.solve();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

// fn main() {
//     let mut input = String::new();

//     io::stdin()
//         .read_to_string(&mut input)
//         .expect("Failed to read input");

//     let (_, mut puzzle) = all_consuming(parse)
//         .parse(input.as_str())
//         .expect("should parse");

//     println!("{puzzle}");

//     puzzle.solve();
// }

fn _parse(input: &str) -> IResult<&str, Puzzle> {
    let (input, (rows, _)) = all_consuming((
        separated_list1(line_ending, count(take(1usize), 9)),
        opt(line_ending),
    ))
    .parse(input)?;

    assert!(rows.len() == 9);

    rows.iter().for_each(|row| {
        assert!(row.len() == 9);
        row.iter().for_each(|s| assert!(s.len() == 1));
    });

    let data: BTreeMap<String, Option<u8>> = rows
        .iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter().enumerate().map(move |(i, s)| {
                (
                    format!(
                        "{}{j}",
                        match i {
                            0 => 'a',
                            1 => 'b',
                            2 => 'c',
                            3 => 'd',
                            4 => 'e',
                            5 => 'f',
                            6 => 'g',
                            7 => 'h',
                            8 => 'i',
                            _ => {
                                panic!("more than 9 columns")
                            }
                        }
                    ),
                    {
                        let ch = s.chars().next().unwrap();

                        match ch {
                            '1'..='9' => Some(ch.to_digit(10).unwrap() as u8),
                            _ => None,
                        }
                    },
                )
            })
        })
        .collect();

    Ok((
        input,
        Puzzle {
            data: data.clone(),
            initial_cells: data.keys().cloned().collect(),
        },
    ))
}

#[derive(Debug)]
struct Puzzle {
    data: BTreeMap<String, Option<u8>>,
    initial_cells: HashSet<String>,
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+---+")?;
        for row in 0..9 {
            write!(f, "|")?;
            for col in 0..9 {
                let key = format!(
                    "{}{}",
                    match col {
                        0 => 'a',
                        1 => 'b',
                        2 => 'c',
                        3 => 'd',
                        4 => 'e',
                        5 => 'f',
                        6 => 'g',
                        7 => 'h',
                        8 => 'i',
                        _ => panic!("more than 9 columns"),
                    },
                    row
                );
                write!(
                    f,
                    " {} |",
                    if let Some(v) = self.data.get(&key).unwrap() {
                        let num_string = String::from((v + b'0') as char);

                        if self.initial_cells.contains(&key) {
                            num_string.stylize()
                        } else {
                            num_string.blue().bold()
                        }
                    } else {
                        " ".to_string().stylize()
                    }
                )?;
            }
            writeln!(f)?;
            writeln!(f, "+---+---+---+---+---+---+---+---+---+")?;
        }

        Ok(())
    }
}

impl Puzzle {
    fn from_array(data: &[[u8; 9]; 9]) -> Self {
        let initial_cells = data
            .iter()
            .enumerate()
            .flat_map(|(j, row)| {
                row.iter().enumerate().filter_map(move |(i, value)| {
                    if matches!(value, 1..=9) {
                        Some(format!(
                            "{}{}",
                            match i {
                                0 => 'a',
                                1 => 'b',
                                2 => 'c',
                                3 => 'd',
                                4 => 'e',
                                5 => 'f',
                                6 => 'g',
                                7 => 'h',
                                8 => 'i',
                                _ => panic!("more than 9 columns"),
                            },
                            (b'0' + j as u8) as char,
                        ))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self {
            data: data
                .iter()
                .enumerate()
                .flat_map(|(j, row)| {
                    row.iter().enumerate().map(move |(i, value)| {
                        let key = format!(
                            "{}{}",
                            match i {
                                0 => 'a',
                                1 => 'b',
                                2 => 'c',
                                3 => 'd',
                                4 => 'e',
                                5 => 'f',
                                6 => 'g',
                                7 => 'h',
                                8 => 'i',
                                _ => panic!("more than 9 columns"),
                            },
                            (b'0' + j as u8) as char,
                        );

                        (
                            key,
                            match value {
                                1..=9 => Some(*value),
                                _ => None,
                            },
                        )
                    })
                })
                .collect(),
            initial_cells,
        }
    }

    fn solve(&mut self) {
        let solver = Solver::new();

        let mut int_vars = HashMap::new();

        int_vars.insert("a0", Int::fresh_const("a0"));
        int_vars.insert("a1", Int::fresh_const("a1"));
        int_vars.insert("a2", Int::fresh_const("a2"));
        int_vars.insert("a3", Int::fresh_const("a3"));
        int_vars.insert("a4", Int::fresh_const("a4"));
        int_vars.insert("a5", Int::fresh_const("a5"));
        int_vars.insert("a6", Int::fresh_const("a6"));
        int_vars.insert("a7", Int::fresh_const("a7"));
        int_vars.insert("a8", Int::fresh_const("a8"));
        int_vars.insert("b0", Int::fresh_const("b0"));
        int_vars.insert("b1", Int::fresh_const("b1"));
        int_vars.insert("b2", Int::fresh_const("b2"));
        int_vars.insert("b3", Int::fresh_const("b3"));
        int_vars.insert("b4", Int::fresh_const("b4"));
        int_vars.insert("b5", Int::fresh_const("b5"));
        int_vars.insert("b6", Int::fresh_const("b6"));
        int_vars.insert("b7", Int::fresh_const("b7"));
        int_vars.insert("b8", Int::fresh_const("b8"));
        int_vars.insert("c0", Int::fresh_const("c0"));
        int_vars.insert("c1", Int::fresh_const("c1"));
        int_vars.insert("c2", Int::fresh_const("c2"));
        int_vars.insert("c3", Int::fresh_const("c3"));
        int_vars.insert("c4", Int::fresh_const("c4"));
        int_vars.insert("c5", Int::fresh_const("c5"));
        int_vars.insert("c6", Int::fresh_const("c6"));
        int_vars.insert("c7", Int::fresh_const("c7"));
        int_vars.insert("c8", Int::fresh_const("c8"));
        int_vars.insert("d0", Int::fresh_const("d0"));
        int_vars.insert("d1", Int::fresh_const("d1"));
        int_vars.insert("d2", Int::fresh_const("d2"));
        int_vars.insert("d3", Int::fresh_const("d3"));
        int_vars.insert("d4", Int::fresh_const("d4"));
        int_vars.insert("d5", Int::fresh_const("d5"));
        int_vars.insert("d6", Int::fresh_const("d6"));
        int_vars.insert("d7", Int::fresh_const("d7"));
        int_vars.insert("d8", Int::fresh_const("d8"));
        int_vars.insert("e0", Int::fresh_const("e0"));
        int_vars.insert("e1", Int::fresh_const("e1"));
        int_vars.insert("e2", Int::fresh_const("e2"));
        int_vars.insert("e3", Int::fresh_const("e3"));
        int_vars.insert("e4", Int::fresh_const("e4"));
        int_vars.insert("e5", Int::fresh_const("e5"));
        int_vars.insert("e6", Int::fresh_const("e6"));
        int_vars.insert("e7", Int::fresh_const("e7"));
        int_vars.insert("e8", Int::fresh_const("e8"));
        int_vars.insert("f0", Int::fresh_const("f0"));
        int_vars.insert("f1", Int::fresh_const("f1"));
        int_vars.insert("f2", Int::fresh_const("f2"));
        int_vars.insert("f3", Int::fresh_const("f3"));
        int_vars.insert("f4", Int::fresh_const("f4"));
        int_vars.insert("f5", Int::fresh_const("f5"));
        int_vars.insert("f6", Int::fresh_const("f6"));
        int_vars.insert("f7", Int::fresh_const("f7"));
        int_vars.insert("f8", Int::fresh_const("f8"));
        int_vars.insert("g0", Int::fresh_const("g0"));
        int_vars.insert("g1", Int::fresh_const("g1"));
        int_vars.insert("g2", Int::fresh_const("g2"));
        int_vars.insert("g3", Int::fresh_const("g3"));
        int_vars.insert("g4", Int::fresh_const("g4"));
        int_vars.insert("g5", Int::fresh_const("g5"));
        int_vars.insert("g6", Int::fresh_const("g6"));
        int_vars.insert("g7", Int::fresh_const("g7"));
        int_vars.insert("g8", Int::fresh_const("g8"));
        int_vars.insert("h0", Int::fresh_const("h0"));
        int_vars.insert("h1", Int::fresh_const("h1"));
        int_vars.insert("h2", Int::fresh_const("h2"));
        int_vars.insert("h3", Int::fresh_const("h3"));
        int_vars.insert("h4", Int::fresh_const("h4"));
        int_vars.insert("h5", Int::fresh_const("h5"));
        int_vars.insert("h6", Int::fresh_const("h6"));
        int_vars.insert("h7", Int::fresh_const("h7"));
        int_vars.insert("h8", Int::fresh_const("h8"));
        int_vars.insert("i0", Int::fresh_const("i0"));
        int_vars.insert("i1", Int::fresh_const("i1"));
        int_vars.insert("i2", Int::fresh_const("i2"));
        int_vars.insert("i3", Int::fresh_const("i3"));
        int_vars.insert("i4", Int::fresh_const("i4"));
        int_vars.insert("i5", Int::fresh_const("i5"));
        int_vars.insert("i6", Int::fresh_const("i6"));
        int_vars.insert("i7", Int::fresh_const("i7"));
        int_vars.insert("i8", Int::fresh_const("i8"));

        // Assert that all integers are in the range 1..=9
        for int_var in int_vars.values() {
            solver.assert(int_var.ge(Int::from_u64(1)));
            solver.assert(int_var.le(Int::from_u64(9)));
        }

        // Assert that all rows have distinct values
        for row in 0..9 {
            let mut row_vars = Vec::new();
            for col in 0..9 {
                let key = format!(
                    "{}{}",
                    match col {
                        0 => 'a',
                        1 => 'b',
                        2 => 'c',
                        3 => 'd',
                        4 => 'e',
                        5 => 'f',
                        6 => 'g',
                        7 => 'h',
                        8 => 'i',
                        _ => panic!("more than 9 columns"),
                    },
                    row
                );
                row_vars.push(int_vars.get(&key.as_str()).unwrap().clone());
            }
            solver.assert(Int::distinct(&row_vars));
        }

        // Assert that all columns have distinct values
        for col in 0..9 {
            let mut col_vars = Vec::new();
            for row in 0..9 {
                let key = format!(
                    "{}{}",
                    match col {
                        0 => 'a',
                        1 => 'b',
                        2 => 'c',
                        3 => 'd',
                        4 => 'e',
                        5 => 'f',
                        6 => 'g',
                        7 => 'h',
                        8 => 'i',
                        _ => panic!("more than 9 columns"),
                    },
                    row
                );
                col_vars.push(int_vars.get(&key.as_str()).unwrap().clone());
            }
            solver.assert(Int::distinct(&col_vars));
        }

        // Assert that all 3x3 boxes have distinct values
        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut box_vars = Vec::new();
                for row in 0..3 {
                    for col in 0..3 {
                        let key = format!(
                            "{}{}",
                            match box_col * 3 + col {
                                0 => 'a',
                                1 => 'b',
                                2 => 'c',
                                3 => 'd',
                                4 => 'e',
                                5 => 'f',
                                6 => 'g',
                                7 => 'h',
                                8 => 'i',
                                _ => panic!("more than 9 columns"),
                            },
                            box_row * 3 + row
                        );
                        box_vars.push(int_vars.get(&key.as_str()).unwrap().clone());
                    }
                }
                solver.assert(Int::distinct(&box_vars));
            }
        }

        // Assign values to the integers that are known from the initial puzzle data

        for (key, value) in &self.data {
            if let Some(v) = value
                && let Some(int_var) = int_vars.get(key.as_str())
            {
                solver.assert(int_var.eq(Int::from_u64(*v as u64)));
            }
        }

        match solver.check() {
            z3::SatResult::Sat => {
                let model = solver.get_model().unwrap();

                for (key, int_var) in &int_vars {
                    let value = model.eval(int_var, true).unwrap().as_i64().unwrap() as u8;

                    self.data
                        .entry(key.to_string())
                        .and_modify(|e| *e = Some(value));
                }
                println!("{self}");
            }
            z3::SatResult::Unsat => {
                println!("No solution found");
            }
            z3::SatResult::Unknown => {
                println!("Solver returned unknown");
            }
        }
    }
}
