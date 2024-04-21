use std::{io};
use std::io::BufRead;
use crate ::{add, cos, div, fact, mul, pwr, sin, sqrt, sub};

//use great_calculator::*;
fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut sum = 0.0;
    let mut sum_of_powers:f64 = 0.0;
    let mut count = 0;

    while let Some(line)= lines.next() {
        for str_number in line.unwrap().split_whitespace() {
            let number: f64 = str_number.parse().unwrap();
            count += 1;
            sum += number;
            sum_of_powers += pwr(number,2 as f64);               //           !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! needs change

            //println!("{}",number+1 as f64);
        }
    }
        let average:f64 = sum/count as f64;
        let mut intermediate:f64 = sum_of_powers-(count as f64 * average.powf(2.0));
        intermediate = intermediate/(count-1) as f64;
        let result:f64 = intermediate.powf(0.5);

        println!("{}",result);

}