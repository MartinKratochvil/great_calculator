

///takes two float values and returns their sum
///
/// ```
///
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