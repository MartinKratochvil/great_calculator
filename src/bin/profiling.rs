use std::{io};
use std::io::BufRead;
use great_calculator::our_math_lib::{div, mul, pwr, sqrt, sub};


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
            sum_of_powers += pwr(number,2.0);

            //println!("{}",number);
        }
    }
        let average:f64 = div(sum,count as f64);
        let mut intermediate:f64 = sub(sum_of_powers,mul(count as f64, pwr(average,2.0)));
        intermediate = div(intermediate,sub(count as f64, 1.0));
        let result:f64 = sqrt(intermediate,2.0);

        println!("{}",result);
}