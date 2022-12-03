// Copyright 2022 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(unused)]
pub fn day3(file: File) /* -> anyhow::Result<()> */
{
    println!("-- Problem 3 --");
    let lines = BufReader::new(file).lines();

    let mut sum: u32 = 0;
    let mut sum_groups: u32 = 0;

    let mut group: Vec<String> = Vec::new();

    for l in lines {
        if let Ok(line) = l {
            let (comp1, comp2) = line.split_at(line.len() / 2);

            group.push(line.clone());
            if group.len() == 3 {
                let mut set0 = group[0].chars().collect::<HashSet<_>>();
                let mut set1 = group[1].chars().collect::<HashSet<_>>();

                let mut common_letters: Vec<char> = Vec::new();
                for letter in group[2].chars().into_iter() {
                    if set0.contains(&letter) && set1.contains(&letter) {
                        common_letters.push(letter);
                        set0.remove(&letter); // No need to remove from both
                    }
                }

                sum_groups += get_score(&common_letters);

                group.clear();
            }

            // TODO: A HashSet Cannot hold duplicates
            let mut set = comp1.chars().collect::<HashSet<_>>();

            let mut common_letters: Vec<char> = Vec::new();
            for letter in comp2.chars().into_iter() {
                if set.contains(&letter) {
                    common_letters.push(letter);
                    set.remove(&letter);
                }
            }

            sum += get_score(&common_letters);
        }
    }

    println!("Sum (Part 1): {}", sum);
    println!("Sum groups (Part 2): {}", sum_groups);
}

fn get_score(val: &Vec<char>) -> u32 {
    let mut sum: u32 = 0;

    for letter in val {
        let x = if letter.is_uppercase() { 26 } else { 0 };
        sum += x + match letter {
            'a' | 'A' => 1,
            'b' | 'B' => 2,
            'c' | 'C' => 3,
            'd' | 'D' => 4,
            'e' | 'E' => 5,
            'f' | 'F' => 6,
            'g' | 'G' => 7,
            'h' | 'H' => 8,
            'i' | 'I' => 9,
            'j' | 'J' => 10,
            'k' | 'K' => 11,
            'l' | 'L' => 12,
            'm' | 'M' => 13,
            'n' | 'N' => 14,
            'o' | 'O' => 15,
            'p' | 'P' => 16,
            'q' | 'Q' => 17,
            'r' | 'R' => 18,
            's' | 'S' => 19,
            't' | 'T' => 20,
            'u' | 'U' => 21,
            'v' | 'V' => 22,
            'w' | 'W' => 23,
            'x' | 'X' => 24,
            'y' | 'Y' => 25,
            'z' | 'Z' => 26,
            _ => unreachable!("Bad input"),
        }
    }
    sum
}
