// Copyright 2022 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use std::fs::File;

mod day1;
mod day2;
mod day3;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(anyhow::anyhow!("Usage: {} path/to/input", args[0]));
    }

    let file = File::open(&args[1])?;

    day3::day3(file);
    Ok(())
}
