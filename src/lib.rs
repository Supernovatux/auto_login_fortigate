use std::io;
use std::io::prelude::*;
pub mod get_pass;

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout,"Press any key to continue.....").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [1u8]).unwrap();
}