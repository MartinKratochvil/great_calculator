//! # Program made for profiling tests.
//!
//! Counts standard deviation of numbers from stdin
//! Program reads until `EOF`, if stdin in not redirected to file,
//! result is given after writing EOF to command line manually (using ctrl+D)

use std::{io};
use std::io::BufRead;
use great_calculator::our_math_lib::{div, mul, pwr, sqrt, sub};


fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();  //store all the lines inputted

    let mut sum = 0.0;
    let mut sum_of_powers:f64 = 0.0;
    let mut count = 0;            //initialization of variables

    while let Some(line)= lines.next() { //while there are any lines left
        for str_number in line.unwrap().split_whitespace() {  //for every number on line
            let number: f64 = str_number.parse().unwrap();  //cast the string into float
            count += 1;
            sum += number;
            sum_of_powers += pwr(number,2.0);

            //println!("{}",number);
        }
    }
        let average:f64 = div(sum,count as f64);             //average
        let mut intermediate:f64 = sub(sum_of_powers,mul(count as f64, pwr(average,2.0)));  //intermediate = (sum_of_powers - count * average^2
        intermediate = div(intermediate,sub(count as f64, 1.0));   // intermediate = intermediate/(count -1)
        let result:f64 = sqrt(intermediate,2.0);                     // sqrt of intermediate

        println!("{}",result);
}