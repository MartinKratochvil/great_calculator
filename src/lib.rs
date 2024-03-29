

///takes two float values and returns their sum
///
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

/// returns left - right
///
/// ```
/// let x = 36;
/// let y = 16;
/// let sum = great_calculator::sub(x as f64,y as f64);
/// assert_eq!(sum,20f64);
/// ```
/// ```
/// assert_eq!(great_calculator::sub(20f64,13.0), 7.0);
/// ```
pub fn sub(left: f64, right: f64) -> f64{
    left - right
}

/// Function requires two arguments of type f64
/// returns arg1 * arg2
/// ```
/// let y = 15.0;
/// let x = 10.0;
/// let result = great_calculator::mul(x,y);
/// assert_eq!(result,150.0);
/// ```
pub fn mul(left: f64, right:f64) -> f64{
    left*right
}
/// Function requires two arguments of type f64
/// returns arg1 / arg2
/// ```
/// let y = 40.0;
/// let x = 10.0;
/// let result = great_calculator::div(x,y);
/// assert_eq!(result,4.0);
/// let result = great_calculator::div(y,x);
/// assert_eq!(result,0.25);///
/// let result = great_calculator::div(x,x);
/// assert_eq!(result,1.0);
/// ```
pub fn div(left: f64, right:f64) -> f64{
    left/right
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