/// Function requires two arguments of type f64
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
    left + right
}

/// Function requires two arguments of type f64
/// returns arg2-th Sqrt of arg1
/// ```
/// let n = 2.0;
/// let x = 16.0;
/// let result = great_calculator::sqrt(x,n);
/// assert_eq!(result,4.0);
/// let x = 1024.0;
/// let result = great_calculator::pwr(x,n);
/// assert_eq!(result,32.0);
/// let x = 4096.0;
/// let n = 4.0;
/// let result = great_calculator::pwr(x,n);
/// assert_eq!(result,8.0);
/// let x = 27.0;
/// let n = 3.0;
/// let result = great_calculator::pwr(x,n);
///  assert_eq!(result,3.0);
/// ```
pub fn sqrt(left: f64, right:f64) -> f64{
    left + right
}

/// Function requires one argument of type f64
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
pub fn fact(x: f64) -> f64{
    x
}

/// Function requires one argument of type f64
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
/// let refresult = f64::sin(x.to_radians());
/// assert_eq!(result,refresult);
/// let x = 60.0;
/// let refresult = f64::sin(x.to_radians());
/// let result = great_calculator::sin(x);
///  assert_eq!(result,refresult);
/// ```
pub fn sin(x: f64) -> f64{
    x
}

/// Function requires one argument of type f64
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
/// let refresult = f64::cos(x.to_radians());
/// assert_eq!(result,refresult);
/// let x = 60.0;
/// let refresult = f64::cos(x.to_radians());
/// let result = great_calculator::cos(x);
///  assert_eq!(result,refresult);
/// ```
pub fn cos(x: f64) -> f64{
    x
}





/*
#[cfg(test)]
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