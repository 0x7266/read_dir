#![allow(unused)]

use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}");
    }
    Ok(())
}
