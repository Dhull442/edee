use std::io::{self,Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

fn main() {
    // let mut command = false;
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        if c=='Q' {
            println!("Quitting");
            disable_raw_mode().unwrap();
        } else {
            print("{}",c);
        }
    }
}
