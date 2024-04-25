//! # Our_math_lib
//!
//! A math library for our calculator.

use std::{f64::{INFINITY, NAN}};

/// Addition of two numbers
///#
/// Function requires two arguments of type f64
///
/// `returns` arg1 + arg2
///
/// # Examples
/// ```
/// // Some documentation.
/// use great_calculator::our_math_lib::add;
/// let x = 23.0;
/// let y = 32.0;
/// let sum = add(x,y);
/// assert_eq!(sum, 55.0);
/// ```
pub fn add(left: f64, right: f64) -> f64{
    left + right
}

/// Subtraction of two numbers
///#
/// Function requires two arguments of type f64
///
/// `returns` arg1 - arg2
///
/// # Examples
/// ```
/// use great_calculator::our_math_lib::sub;
/// let x = 36.0;
/// let y = 16.0;
/// let result = sub(x,y);
/// assert_eq!(result,20.0);
///
/// assert_eq!(sub(20.0,13.0), 7.0);
/// ```
pub fn sub(left: f64, right: f64) -> f64{
    left - right
}

/// Multiplication of two numbers
///#
/// Function requires two arguments of type f64
///
/// `returns` arg1 * arg2
///
/// # Examples
/// ```
/// use great_calculator::our_math_lib::mul;
/// let x = 15.0;
/// let y = 10.0;
/// let result = mul(x,y);
/// assert_eq!(result,150.0);
///
/// let result = mul(y,x);
/// assert_eq!(result,150.0);
/// ```
pub fn mul(left: f64, right:f64) -> f64{
    left*right
}

/// Division of two numbers
///#
/// Function requires two arguments of type f64
///
/// `returns` arg1 / arg2
///
/// # Examples
/// ```
/// use great_calculator::our_math_lib::div;
/// let x = 40.0;
/// let y = 10.0;
/// let result = div(x,y);
/// assert_eq!(result,4.0);
///
/// let result = div(y,x);
/// assert_eq!(result,0.25);
///
/// let result = div(x,x);
/// assert_eq!(result,1.0);
///
/// let x = 40.0;
/// let y = 0.0;
/// let result = div(x,y);
/// assert_eq!(result,f64::NAN);
/// ```
pub fn div(left: f64, right:f64) -> f64{
    if right == 0.0{
        return f64::NAN;
    }
    else {
        left / right
    }
}

/// N-th power of a number
///#
/// Function requires two arguments of type f64
///
/// `returns` x^{n}
///
/// # Examples
/// ```
/// use great_calculator::our_math_lib::pwr;
/// let x = 2.0;
/// let y = 5.0;
/// let result = pwr(x,y);
/// assert_eq!(result,32.0);
///
/// let result = pwr(y,x);
/// assert_eq!(result,25.0);
///
/// let x = 10.0;
/// let y = 5.0;
/// let result = pwr(y,x);
/// assert_eq!(result,9765625.0);
///
/// let result = pwr(x,y);
/// assert_eq!(result,100000.0);
///
/// let x = 5.0;
/// let y = 0.0;
/// let result = pwr(x,y);
/// assert_eq!(result,1.0);
/// ```
pub fn pwr(x: f64, n:f64) -> f64{
    x.powf(n)
}

/// N-th root of a number
///#
/// Function requires two arguments of type f64
///
/// `returns` n-th root of x
///
/// # Examples
/// ```
/// use great_calculator::our_math_lib::sqrt;
/// let n = 2.0;
/// let x = 16.0;
/// let result = sqrt(x,n);
/// assert_eq!(result,4.0);
///
/// let x = 256.0;
/// let n = 2.0;
/// let result = sqrt(x,n);
/// assert_eq!(result,16.0);
///
/// let x = 4096.0;
/// let n = 4.0;
/// let result = sqrt(x,n);
/// assert_eq!(result,8.0);
///
/// let x = 27.0;
/// let n = 3.0;
/// let result = sqrt(x,n);
///  assert_eq!(result,3.0);
/// ```
pub fn sqrt(x: f64, n:f64) -> f64{
    x.powf(1.0/n)
}

/// Factorial of a number
///#
/// Function requires one argument of type f64
///
/// `returns` x!
///
/// # Examples
/// ```
/// use great_calculator::our_math_lib::fact;
/// let x = 3.0;
/// let result = fact(x);
/// assert_eq!(result,6.0);
///
/// let x = 5.0;
/// let result = fact(x);
/// assert_eq!(result,120.0);
///
/// let x = 10.0;
/// let result = fact(x);
/// assert_eq!(result,3628800.0);
///
/// let x = 1.0;
/// let result = fact(x);
///  assert_eq!(result,1.0);
/// ```
pub fn fact(x: f64) -> f64{  //Counts factorial using recursion
    if x == INFINITY.into(){
        return INFINITY.into();
    }
    if x == NAN.into(){
        return NAN.into();
    }
    if x != x.trunc() {
        return NAN.into();
    }
    if x < 0.0 {
        return NAN.into();
    }
    if x > 170.0 {
        return INFINITY.into();
    }
    let x = x as i64;
    let mut result = 1.0;
    for i in 1..x+1 {
        result *= i as f64;
    }
    result
}

/// Sine of a number
///#
/// Function requires one argument of type f64
///
/// `returns` sin(x)
///
/// # Examples
/// ```
/// use great_calculator::our_math_lib::sin;
/// let x = 0.0;
/// let result = sin(x);
/// assert_eq!(result,0.0);
///
/// let x = 90.0;
/// let result = sin(x);
/// assert_eq!(result,1.0);
///
/// let x = 45.0;
/// let result = sin(x);
/// let refresult = f64::sin(f64::to_radians(x));
/// assert_eq!(result,refresult);
///
/// let x = 60.0;
/// let refresult = f64::sin(f64::to_radians(x));
/// let result = sin(x);
///  assert_eq!(result,refresult);
/// ```
pub fn sin(x: f64) -> f64{
    let y = x as i64;
    match (y%360+360)%360 { // if inputed with negative value, transfers it into positive equivalent
        0 | 180 => 0.0,
        90 => 1.0,
        270 => -1.0,
        _=> f64::sin(x*std::f64::consts::PI/180.0),
    }
}

/// Cosine of a number
///#
/// Function requires one argument of type f64
///
/// `returns` cos(x)
///
/// # Examples
/// ```
/// use great_calculator::our_math_lib::cos;
/// let x = 0.0;
/// let result = cos(x);
/// assert_eq!(result,1.0);
/// let x = 90.0;
/// let result = cos(x);
/// assert_eq!(result,0.0);
/// let x = 45.0;
/// let result = cos(x);
/// let refresult = f64::cos(f64::to_radians(x));
/// assert_eq!(result,refresult);
/// let x = 60.0;
/// let refresult = f64::cos(f64::to_radians(x));
/// let result = cos(x);
///  assert_eq!(result,refresult);
/// ```
pub fn cos(x: f64) -> f64{
    let y = x as i64;
    match (y%360+360)%360 { // if inputed with negative value, transfers it into positive equivalent
        0  => 1.0,
        90 | 270 => 0.0,
        180 => -1.0,
        _=> f64::cos(x*std::f64::consts::PI/180.0),
    }
}