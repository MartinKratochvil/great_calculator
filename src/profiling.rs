use std::{io};
use std::io::BufRead;

fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line)= lines.next(){

        for number in line.unwrap().split_whitespace(){
            println!("{}",number);
        }

    }

}