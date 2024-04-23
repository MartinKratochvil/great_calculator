use great_calculator::*;

#[test]
#[cfg(test)]
pub fn test_parser(){
    // ((2-/((5^3-3^4)+5)*2)-10)! / 4
    use parser::*;
    let x = vec![Token::LeftParentheses, Token::LeftParentheses, Token::Value(2.0),
                 Token::Sqrt, Token::LeftParentheses,Token::LeftParentheses,Token::Value(5.0), Token::Pow, Token::Value(3.0),
                 Token::Minus, Token::Value(3.0), Token::Pow, Token::Value(4.0), Token::RightParentheses, Token::Plus,
                 Token::Value(5.0), Token::RightParentheses, Token::Star, Token::Value(2.0), Token::RightParentheses,
                 Token::Minus, Token::Value(10.0), Token::RightParentheses, Token::Exclamation, Token::Slash, Token::Value(4.0) ];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }

    //println!("((2-/((5^3-3^4)+5)*2)-10)! / 4 = 6 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 6.0);

    let x = vec![Token::LeftParentheses, Token::Value(2.0),
                 Token::Sqrt, Token::LeftParentheses,Token::LeftParentheses,Token::Value(5.0), Token::Pow, Token::Value(3.0),
                 Token::Minus, Token::Value(3.0), Token::Pow, Token::Value(4.0), Token::RightParentheses, Token::Plus,
                 Token::Value(5.0), Token::RightParentheses, Token::Star, Token::Value(2.0), Token::RightParentheses,
                 Token::Minus, Token::Value(10.0)];

    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("(2-/((5^3-3^4)+5)*2)-10 = 4 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 4.0);

    let x = vec![Token::Value(5.0), Token::Plus, Token::Value(5.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("5 + 5 = 10 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 10.0);

    let x = vec![Token::Value(5.0), Token::Minus, Token::Value(5.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("5 - 5 = 0 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 0.0);

    let x = vec![Token::Value(5.0), Token::Star, Token::Value(5.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("5 * 5 = 25 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 25.0);

    let x = vec![Token::Value(5.0), Token::Slash, Token::Value(5.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("5 / 5 = 1 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 1.0);

    let x = vec![Token::Value(5.0), Token::Pow, Token::Value(2.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("5 ^ 2 = 25 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 25.0);

    let x = vec![Token::Value(2.0), Token::Sqrt, Token::Value(4.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("2-/4 = 2 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 2.0);

    let x = vec![Token::Value(5.0), Token::Star, Token::Value(5.0), Token::Minus, Token::Value(5.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("5 * 5 - 5 = 20 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 20.0);

    let x = vec![Token::Value(5.0), Token::Minus, Token::Value(5.0), Token::Star, Token::Value(5.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("5 - 5 * 5 = -20 : {}",tree.calculate());
    assert_eq!(tree.calculate(), -20.0);

    let x = vec![Token::LeftParentheses, Token::Value(5.0), Token::Minus, 
    Token::Value(5.0), Token::RightParentheses, Token::Star, Token::Value(5.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("(5 - 5) * 5 = 0 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 0.0);

    let x = vec![Token::LeftParentheses, Token::Value(5.0), Token::Plus, 
    Token::Value(5.0), Token::RightParentheses, Token::Star, Token::Value(5.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("(5 + 5) * 5 = 50 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 50.0);

    let x = vec![Token::Value(5.0), Token::Exclamation];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("5! = 120 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 120.0);

    let x = vec![ Token::Sin, Token::Value(90.0), Token::Plus, Token::Value(2.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("sin(90) + 2 = 3 : {}",tree.calculate());
    assert_eq!(tree.calculate(), 3.0);

    let x = vec![ Token::Cos, Token::Value(90.0), Token::Minus, Token::Value(2.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("cos(90) - 2 = -2 : {}",tree.calculate());
    assert_eq!(tree.calculate(), -2.0);

    let x = vec![Token::Minus, Token::Value(5.0)];
    let tree;
    if let Ok(a) = Tree::parse(x){
        tree = a;
    }
    else {
        panic!();
    }
    //println!("-5 = -5 : {}",tree.calculate());
    assert_eq!(tree.calculate(), -5.0);

    let x = vec![Token::Exclamation, Token::Value(5.0)];
    let tree = Tree::parse(x);
    //println!("!5 != 5 : {}",tree.calculate());
    match tree {
        Ok(..) => assert!(false),
        Err(i) =>{
            match i {
                ErrorCalls::FactorialWithoutArgument => (),
                _=> assert!(false),
            }
        }
    }
    if let Err(error) = tree {
        println!("{}", error);
    }
}



