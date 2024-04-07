#[cfg(test)]
/// Function requires two arguments of type f64
///
/// returns arg1 + arg2
/// ```
/// // Some documentation.
/// let x = 23.0;
/// let y = 32.0;
/// let sum = great_calculator::add(x,y);
/// assert_eq!(sum, 55.0);
/// ```
///
pub fn add(left: f64, right: f64) -> f64{
    left + right
}

/// Function requires two arguments of type f64
///
/// returns arg1 - arg2
/// ```
/// let x = 36;
/// let y = 16;
/// let sum = great_calculator::sub(x as f64,y as f64);
/// assert_eq!(sum,20f64);
/// assert_eq!(great_calculator::sub(20f64,13.0), 7.0);
/// ```
pub fn sub(left: f64, right: f64) -> f64{
    left - right
}

/// Function requires two arguments of type f64
///
/// returns arg1 * arg2
/// ```
/// let x = 15.0;
/// let y = 10.0;
/// let result = great_calculator::mul(x,y);
/// assert_eq!(result,150.0);
/// let result = great_calculator::mul(y,x);
/// assert_eq!(result,150.0);
/// ```
pub fn mul(left: f64, right:f64) -> f64{
    left*right
}

/// Function requires two arguments of type f64
///
/// returns arg1 / arg2
/// ```
/// let x = 40.0;
/// let y = 10.0;
/// let result = great_calculator::div(x,y);
/// assert_eq!(result,4.0);
/// let result = great_calculator::div(y,x);
/// assert_eq!(result,0.25);
/// let result = great_calculator::div(x,x);
/// assert_eq!(result,1.0);
/// ```
pub fn div(left: f64, right:f64) -> f64{
    left/right
}

/// Function requires two arguments of type f64
///
/// returns arg1^{arg2}
/// ```
/// let x = 2.0;
/// let y = 5.0;
/// let result = great_calculator::pwr(x,y);
/// assert_eq!(result,32.0);
/// let result = great_calculator::pwr(y,x);
/// assert_eq!(result,25.0);
/// let x = 10.0;
/// let y = 5.0;
/// let result = great_calculator::pwr(y,x);
/// assert_eq!(result,9765625.0);
/// let result = great_calculator::pwr(x,y);
///  assert_eq!(result,100000.0);
/// ```
pub fn pwr(left: f64, right:f64) -> f64{
    left.powf(right)
}

/// Function requires two arguments of type f64
///
/// returns arg2-th Sqrt of arg1
/// ```
/// let n = 2.0;
/// let x = 16.0;
/// let result = great_calculator::sqrt(x,n);
/// assert_eq!(result,4.0);
/// let x = 256.0;
/// let result = great_calculator::sqrt(x,n);
/// assert_eq!(result,16.0);
/// let x = 4096.0;
/// let n = 4.0;
/// let result = great_calculator::sqrt(x,n);
/// assert_eq!(result,8.0);
/// let x = 27.0;
/// let n = 3.0;
/// let result = great_calculator::sqrt(x,n);
///  assert_eq!(result,3.0);
/// ```
pub fn sqrt(x: f64, n:f64) -> f64{
    x.powf(1.0/n)
}

/// Function requires one argument of type f64
///
/// returns factorial of arg1...arg1!
/// ```
/// let x = 3.0;
/// let result = great_calculator::fact(x);
/// assert_eq!(result,6.0);
/// let x = 5.0;
/// let result = great_calculator::fact(x);
/// assert_eq!(result,120.0);
/// let x = 10.0;
/// let result = great_calculator::fact(x);
/// assert_eq!(result,3628800.0);
/// let x = 1.0;
/// let result = great_calculator::fact(x);
///  assert_eq!(result,1.0);
/// ```
pub fn fact(x: f64) -> f64{  //Counts factorial using recursion
    if x == 1.0{
        return 1.0;
    }else {
        fact(x - 1.0) * x //function calls itself with argument x-1 until it reduces to 1, then it multiplies by x in every level
    }
}

/// Function requires one argument of type f64
///
/// returns sine of arg1...sin(arg1)
/// ```
/// use great_calculator::sin;
/// let x = 0.0;
/// let result = great_calculator::sin(x);
/// assert_eq!(result,0.0);
/// let x = 90.0;
/// let result = great_calculator::sin(x);
/// assert_eq!(result,1.0);
/// let x = 45.0;
/// let result = great_calculator::sin(x);
/// let refresult = f64::sin(f64::to_radians(x));
/// assert_eq!(result,refresult);
/// let x = 60.0;
/// let refresult = f64::sin(f64::to_radians(x));
/// let result = great_calculator::sin(x);
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

/// Function requires one argument of type f64
///
/// returns cosine of arg1...cos(arg1)
/// ```
/// use great_calculator::sin;
/// let x = 0.0;
/// let result = great_calculator::cos(x);
/// assert_eq!(result,1.0);
/// let x = 90.0;
/// let result = great_calculator::cos(x);
/// assert_eq!(result,0.0);
/// let x = 45.0;
/// let result = great_calculator::cos(x);
/// let refresult = f64::cos(f64::to_radians(x));
/// assert_eq!(result,refresult);
/// let x = 60.0;
/// let refresult = f64::cos(f64::to_radians(x));
/// let result = great_calculator::cos(x);
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


/*
mod test{
    use super::*;
    #[test]
    fn test1(){
        let x = 23.0;
        let y = 32.0;
        let sum = add(x,y);
        assert_eq!(sum, 55.0);
    }
}
*/