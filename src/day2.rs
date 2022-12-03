// Copyright 2022 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

// Item			Symbols		Score
// Rock: 		A, X			1
// Paper: 		B, Y			2
// Scissors: 	C, Z			3

// Outcome		Score	Symbol (part 2)
// Lost			0		X
// Draw			3		Y
// Won			6		Z

use std::{
    fs::File,
    io::{BufRead as _, BufReader},
};

#[repr(u8)]
#[derive(Clone, Copy)]
enum Item {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[allow(unused)]
pub fn day2(file: File) -> anyhow::Result<()> {
    println!("-- Problem 2 --");
    let lines = BufReader::new(file).lines();

    let mut score: usize = 0;
    let mut score2: usize = 0;
    for l in lines {
        if let Ok(line) = l {
            let line = line.split(' ').collect::<Vec<_>>();

            // Part 1
            score += match_item(line[1]).unwrap() as usize;

            match (match_item(line[0]).unwrap(), match_item(line[1]).unwrap()) {
                (Item::Rock, Item::Rock)
                | (Item::Paper, Item::Paper)
                | (Item::Scissors, Item::Scissors) => score += Outcome::Draw as usize,
                (Item::Rock, Item::Scissors)
                | (Item::Scissors, Item::Paper)
                | (Item::Paper, Item::Rock) => score += Outcome::Lose as usize,
                (Item::Scissors, Item::Rock)
                | (Item::Paper, Item::Scissors)
                | (Item::Rock, Item::Paper) => score += Outcome::Win as usize,
            }

            // Part 2
            let outcome = match_outcome(line[1]).unwrap();
            let item = match_item(line[0]).unwrap();
            score2 += outcome as usize;
            match outcome {
                Outcome::Draw => score2 += item as usize,
                Outcome::Lose => match item {
                    Item::Rock => score2 += Item::Scissors as usize,
                    Item::Paper => score2 += Item::Rock as usize,
                    Item::Scissors => score2 += Item::Paper as usize,
                },
                Outcome::Win => match item {
                    Item::Rock => score2 += Item::Paper as usize,
                    Item::Paper => score2 += Item::Scissors as usize,
                    Item::Scissors => score2 += Item::Rock as usize,
                },
            }
        }
    }

    println!("Score (Part 1): {}", score);
    println!("Corrected Score (Part 2): {}", score2);

    Ok(())
}

fn match_item(val: &str) -> Option<Item> {
    match val {
        "A" | "X" => Some(Item::Rock),
        "B" | "Y" => Some(Item::Paper),
        "C" | "Z" => Some(Item::Scissors),
        _ => None,
    }
}

fn match_outcome(val: &str) -> Option<Outcome> {
    match val {
        "X" => Some(Outcome::Lose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}
