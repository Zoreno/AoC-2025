extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;
extern crate crypto;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn transpose_ref<T>(matrix: &[Vec<T>]) -> Vec<Vec<&T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut out = vec![Vec::with_capacity(rows); cols];

    for row in matrix {
        for (j, value) in row.iter().enumerate() {
            out[j].push(value);
        }
    }

    out
}

aoc_lib! { year = 2025 }
