//! # Program made for profiling tests.
//!
//! Counts standard deviation of numbers from stdin
//! Program reads until `EOF`, if stdin in not redirected to file,
//! result is given after writing EOF to command line manually (using ctrl+D)

use std::{io};
use std::io::BufRead;
use great_calculator::our_math_lib::{add, div, mul, pwr, sqrt, sub};

///Main function of program
///#
/// Reads data from `stdin`
/// Counts sum, count and sum of second powers of the numbers on input
/// Then calls function `stddev` and prints result on `stdout`
///
fn main() {
    let mut lines = io::stdin().lock().lines();  //store all the lines inputted

    let mut sum = 0.0;
    let mut sum_of_powers: f64 = 0.0;
    let mut count = 0.0;            //initialization of variables

    while let Some(line) = lines.next() { //while there are any lines left
        for str_number in line.unwrap().split_whitespace() {  //for every number on line
            let number: f64 = str_number.parse().unwrap();  //cast the string into float

            count = add(count, 1.0);
            sum = add(sum, number);
            sum_of_powers = add(sum_of_powers, pwr(number, 2.0));
        }
    }
    let result = stddev(count, sum, sum_of_powers);
    println!("{}",result);
}

///Function for calculation of standard deviation.
///#
/// Function requires 3 parameters:
///
/// `count` :f64 - count of numbers,
///
/// `sum` :f64 - sum of all the numbers,
///
/// `sum_of_powers` :f64 - sum of second powers of all the number.
/// #
/// `return` :f64 - standard deviation
fn stddev(count:f64,sum:f64,sum_of_powers:f64) -> f64{

        let average:f64 = div(sum,count);             //average
        let mut intermediate:f64 = sub(sum_of_powers,mul(count, pwr(average,2.0)));  //intermediate = (sum_of_powers - count * average^2
        intermediate = div(intermediate,sub(count, 1.0));   // intermediate = intermediate/(count -1)
        let result:f64 = sqrt(intermediate,2.0);                     // sqrt of intermediate
        result
}