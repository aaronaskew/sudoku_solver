use std::{
    collections::{BTreeMap, HashMap},
    io::{self, Read},
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

/// +--------------------------+
/// |a0|b0|c0|d0|e0|f0|g0|h0|i0|
/// |a1|b1|c1|d1|e1|f1|g1|h1|i1|
/// |a2|b2|c2|d2|e2|f2|g2|h2|i2|
/// |a3|b3|c3|d3|e3|f3|g3|h3|i3|
/// |a4|b4|c4|d4|e4|f4|g4|h4|i4|
/// |a5|b5|c5|d5|e5|f5|g5|h5|i5|
/// |a6|b6|c6|d6|e6|f6|g6|h6|i6|
/// |a7|b7|c7|d7|e7|f7|g7|h7|i7|
/// |a8|b8|c8|d8|e8|f8|g8|h8|i8|
/// +--------------------------+
fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let (_, mut puzzle) = all_consuming(parse)
        .parse(input.as_str())
        .expect("should parse");

    println!("{puzzle}");

    puzzle.solve();
}

fn parse(input: &str) -> IResult<&str, Puzzle> {
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

    let data = rows
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

    Ok((input, Puzzle { data }))
}

#[derive(Debug)]
struct Puzzle {
    data: BTreeMap<String, Option<u8>>,
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
                        (v + b'0') as char
                    } else {
                        ' '
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
