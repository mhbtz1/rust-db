use std::process::*; 
use log::info;
mod table;
use table::{Table, Column};
use std::io;
use env_logger::Builder;

//Two main ways for storing database systems on disk (and in memory):
// 1. Page structured
// 2. Log structured
pub struct DBRepl<'a, 'b>{
    selectPredicate: Vec<Column<'a, usize>>,
    tableSelector: Vec<Table<'a, 'b, usize>>,
}

fn main() {
    loop {
        print!("habtdb> ");
        let mut buffer = String::new();
        let res = io::stdin().read_line(&mut buffer);
        match res {
            Ok(_) =>  { info!("Processed input!") }; }
            Err(_) => { info!("No input processed!"); }
        }
    }
}
