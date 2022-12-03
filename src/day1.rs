// Copyright 2022 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use std::{
    fs::File,
    io::{BufRead as _, BufReader},
};

#[allow(unused)]
pub fn day1(file: File) -> anyhow::Result<()> {
    println!("-- Problem 1 --");
    let lines = BufReader::new(file).lines();

    let mut calories: usize = 0;
    let mut elves_calories: Vec<usize> = Vec::new();
    for l in lines.into_iter() {
        if let Ok(line) = l {
            if line != "" {
                calories += line.parse::<usize>()?;
            } else {
                elves_calories.push(calories);
                calories = 0;
            }
        }
    }

    elves_calories.sort();

    let mut backup_calories: usize = 0;
    let num_elves_calories = elves_calories.len();
    for i in 0..3 {
        backup_calories += elves_calories[num_elves_calories - 1 - i];
    }

    println!(
        "Highest calories (Part 1): {}",
        *elves_calories.last().unwrap()
    );
    println!("Backup calories (Part 2): {}", backup_calories);

    Ok(())
}
