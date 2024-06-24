use std::process::*; 

mod table;
use table::{Table, Column};
use std::io;

//Two main ways for storing database systems on disk (and in memory):
// 1. Page structured
// 2. Log structured

pub struct DBRepl {
    selectPredicate: Vec<Column>,
    tableSelector: Vec<Table>,
}



fn main() {
    loop {
        print!("habtdb> ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer)?;
    }

}
