//! # parser
//! 
//! Program made for parsing math expression from tokens and converting them to parse tree.

use crate::our_math_lib::{add, cos, div, fact, mul, pwr, sin, sqrt, sub};

/// # Token
/// 
/// Token that is returned by GUI.
#[derive(Clone)]
pub enum Token {
    Value(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Pow,
    Sqrt,
    Sin,
    Cos,
    Exclamation,
    RightParentheses,
    LeftParentheses,
}

impl Token {
    /// # Token::to_binary
    /// Converts `Token` to `BinaryFunctions`.
    /// 
    /// `returns` `BinaryFunctions`
    fn to_binary(&self) -> BinaryFunctions {
        match self {
            Token::Plus => BinaryFunctions::Add,
            Token::Minus => BinaryFunctions::Sub,
            Token::Star => BinaryFunctions::Mul,
            Token::Slash => BinaryFunctions::Div,
            Token::Pow => BinaryFunctions::Pow,
            Token::Sqrt => BinaryFunctions::Root,
            _ => panic!(),
        }
    }

    /// # Token::to_unary
    /// 
    /// Converts `Token` to `UnaryFunctions`.
    /// `returns` `UnaryFunctions`
    fn to_unary(&self) -> UnaryFunctions {
        match self {
            Token::Minus => UnaryFunctions::Negate,
            Token::Sin => UnaryFunctions::Sin,
            Token::Cos => UnaryFunctions::Cos,
            Token::Exclamation => UnaryFunctions::Factorial,
            _ => panic!(),
        }
    }
}


/// # BinaryFunctinos
/// 
/// Enum defining binary functions.
/// used in enum `Tree`.
#[derive(Clone, Copy)]
pub enum BinaryFunctions {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
}

/// # UnaryFunctinos
/// 
/// Enum defining unary functions.
/// Used in enum `Tree`.
#[derive(Clone, Copy)]
pub enum UnaryFunctions {
    Sin,
    Cos,
    Factorial,
    Negate,
}

//definování funkcí pro enum BinaryFuncion
impl BinaryFunctions {
    /// # BinaryFunctions::execute
    /// Takes in two arguments type f64 and calls operation based on enum `BinaryFunctions` it was called on.
    /// 
    /// `returns` `f64`
    fn execute(&self, x: f64, y: f64) -> f64 {
        match self {
            BinaryFunctions::Add => add(x, y),
            BinaryFunctions::Sub => sub(x, y),
            BinaryFunctions::Mul => mul(x, y),
            BinaryFunctions::Div => div(x, y),
            BinaryFunctions::Pow => pwr(x, y),
            BinaryFunctions::Root => sqrt(y, x),
        }
    }
}

//definování funkcí pro enum UnaryFuncion
impl UnaryFunctions {
    /// # UnaryFunctions::execute
    /// Takes in two arguments type f64 and calls operation based on enum `UnaryFunctions` it was called on.
    /// 
    /// `returns` `f64`
    fn execute(&self, x: f64) -> f64 {
        match self {
            UnaryFunctions::Sin => sin(x),
            UnaryFunctions::Cos => cos(x),
            UnaryFunctions::Factorial => fact(x),
            UnaryFunctions::Negate => -x,
        }
    }
}

/// # ErrorCalls
/// 
/// Enum representing possible errors.
#[derive(Clone, Copy)]
pub enum ErrorCalls{
    UnclosedParentheses,
    ExtraClosingParentheses,
    UnaryFunctionWithoutArgument,
    FactorialWithoutArgument,
    BinaryFuncionWithoutArgument,
    UnconectedValues,
    MathError,
    UnexpectedError, //This should never occur
}

impl std::fmt::Display for ErrorCalls {
    /// # ErrorCalls::fmt
    /// 
    /// Implements trait display for the enum ErrorCalls.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnclosedParentheses => write!(f, "Error: Unclosed parentheses"),
            Self::ExtraClosingParentheses => write!(f, "Error: Extra closing parentheses"),
            Self::UnaryFunctionWithoutArgument => write!(f, "Error: Unary function without argument"),
            Self::FactorialWithoutArgument => write!(f, "Error: Factorial without argument"),
            Self::BinaryFuncionWithoutArgument => write!(f, "Error: Operator without argument"),
            Self::UnconectedValues => write!(f, "Error: Operands without operator"),
            Self::MathError => write!(f, "Error: Math Error"),
            Self::UnexpectedError => write!(f, "Error: Something unexpected happened"),
        }
    }
}

impl Tree {
    /// # Tree::parentheses
    /// 
    /// Aguments: `stack`, `tokens_or_trees`
    /// `tokens_or_trees` represents the current imput of Tree::parse_addition
    /// `tokens_or_trees` represents the current stack of Tree::parse_addition
    /// 
    /// Takes input that is in parentheses and once it reaches the end of parentheses
    /// recursively calls Tree::parse_addition with the insed of them.
    /// 
    /// `returns` Result<`TokenOrTree`, `ErrorCalls`>
    fn parentheses( //This function represents state Q1 and Q3
        stack: &mut Vec<TokenOrTree>,
        tokens_or_trees: &mut Vec<TokenOrTree>,
    ) -> Result<TokenOrTree, ErrorCalls> {
        let mut r_par_count = 1;
        while let Some(token_or_tree) = tokens_or_trees.pop() {
            match &token_or_tree {
                TokenOrTree::Token { tok } => {
                    match tok {
                        // ),E->), n++
                        Token::RightParentheses => {
                            r_par_count += 1;
                            stack.push(token_or_tree);
                        }
                        //(, ...) -> self(...), n--
                        Token::LeftParentheses => {
                            let mut argument_for_recursive_calculation_of_parantheses = Vec::new();
                            //We pop everything from stack until we find leftParenthesies token then we call parse_addtion
                            //with everithing we popped
                            while let Some(token_or_tree_from_stack) = stack.pop()  {
                                match &token_or_tree_from_stack {
                                    TokenOrTree::Tree {..} => argument_for_recursive_calculation_of_parantheses.push(token_or_tree_from_stack),
                                    TokenOrTree::Token { tok } =>{
                                        match tok {
                                            Token::RightParentheses => {
                                                r_par_count -= 1;
                                                break;
                                            }
                                            _=> argument_for_recursive_calculation_of_parantheses.push(token_or_tree_from_stack),
                                        }
                                    }
                                }
                            }
                            stack.push(TokenOrTree::Tree{tree: Tree::parse_addition(argument_for_recursive_calculation_of_parantheses)?});
                            // n==0 => state = QLoop
                            if r_par_count <=0 {
                                break;
                            }
                            
                        }
                        _=> stack.push(token_or_tree), //_,E->push(_)
                    }
                }
                TokenOrTree::Tree { .. } => {stack.push(token_or_tree)} //_,E->push(_)
            };
        }
        if let Some(result_tree) = stack.pop() {
            Ok(result_tree)
        }
        else {
            Err(ErrorCalls::UnexpectedError) //UnexpectedError
        }
    }

    /// # Tree::parse_addition
    /// 
    /// Arguments: token_or_trees: Vec<TokenOrTree>
    /// Parses the imput and converts it into enum Tree.
    /// 
    /// `returns` Result<`Tree`, `ErrorCalls`>
    fn parse_addition(mut tokens_or_trees: Vec<TokenOrTree>) -> Result<Tree, ErrorCalls> {
        /// # put_unary_function
        /// Arguments: `stack`, `tok`
        /// 
        /// Takes in two arguments stack and tok
        /// Pops the topmost item on stack and creates the tree with the binary function pointing to it
        /// Pushes the tree back to stack
        /// 
        #[macro_export]
        macro_rules! put_unary_function {
            ($stack: expr, $tok: expr) => {
                if let Some(popped) = $stack.pop() {
                    $stack.push(TokenOrTree::Tree {
                        tree: Tree::UnaryFunction {
                            kind: $tok.to_unary(),
                            x: Box::new(match popped {
                                TokenOrTree::Tree { tree } => tree,
                                _ => return Err(ErrorCalls::UnexpectedError),
                            }),
                        },
                    })
                } else {
                    return Err(ErrorCalls::UnaryFunctionWithoutArgument); //UnaryFunctionWithout Argument
                }
            };
        }

        if tokens_or_trees.len() == 0 {
            return Err(ErrorCalls::BinaryFuncionWithoutArgument);
        }

        #[derive(PartialEq)]
        enum States {
            QLoop, //Main state
            //Q1 is represented by parenthesies()
            Q2, //State taking care of factorial
            //Q3 is represented by parenthesies()
            //Q4 is directly after Q2 
        }
        let mut state = States::QLoop;
        let mut stack = Vec::new();
        while let Some(token_or_tree) = tokens_or_trees.pop() {
            match state {
                States::QLoop => { //Main state
                    match &token_or_tree {
                        //If tree push tree onto stack
                        TokenOrTree::Tree { ..} => stack.push(token_or_tree),
                        //If token call ParsePower on the rest of imput and on the stack
                        TokenOrTree::Token { tok } => {
                            match tok {
                                //+, ^$ -> E / +(self(input.reverse), self(\1))
                                Token::Plus => {
                                    stack.reverse();
                                    return Ok(Tree::BinaryFunction {
                                        kind: BinaryFunctions::Add,
                                        x: Box::new(Tree::parse_addition(tokens_or_trees)?),
                                        y: Box::new(Tree::parse_addition(stack)?),
                                    });
                                }
                                //We find out if - isnt unary if it's not we do the same as for +
                                // if it is we negate the topmost tree from stack and push it back into stack
                                Token::Minus => match tokens_or_trees.last()/* token or tree before - */ {
                                    Some(next) => match next {
                                        //if tree call parse addition recursively and link it with -
                                        TokenOrTree::Tree { .. } => {
                                            stack.reverse();
                                            return Ok(Tree::BinaryFunction {
                                                kind: BinaryFunctions::Sub,
                                                x: Box::new(Tree::parse_addition(tokens_or_trees)?),
                                                y: Box::new(Tree::parse_addition(stack)?),
                                            });
                                        }
                                        //if not tree check what token comes before it
                                        TokenOrTree::Token { tok } => match tok {
                                            //if its value, RightParenth. or factorial, then - is binary
                                            Token::Value(..)
                                            | Token::Exclamation
                                            | Token::RightParentheses => {
                                                stack.reverse();
                                                return Ok(Tree::BinaryFunction {
                                                    kind: BinaryFunctions::Sub,
                                                    x: Box::new(Tree::parse_addition(
                                                        tokens_or_trees
                                                    )?),
                                                    y: Box::new(Tree::parse_addition(stack)?),
                                                });
                                            }
                                            //else is unary
                                            _ => put_unary_function!(stack,tok),
                                        },
                                    },
                                    None => put_unary_function!(stack,tok),
                                },
                                //(>+), E -> (..)
                                Token::Star | Token::Slash | Token::Pow | Token::Sqrt => {
                                    stack.push(token_or_tree)
                                }
                                //v, E->T(v)
                                Token::Value(i) => stack.push(TokenOrTree::Tree {
                                    tree: Tree::Value(*i),
                                }),
                                //sin, T->sin(T)    *sin is any unary function except -,!
                                Token::Sin | Token::Cos => put_unary_function!(stack, tok),
                                // if RightParen. enter state Q1 represented by parentheses()
                                Token::RightParentheses => {
                                    stack.push(token_or_tree);
                                    let tree_from_parentheses = Tree::parentheses(&mut stack, &mut tokens_or_trees);
                                    stack.push(tree_from_parentheses?);
                                }
                                // if Exclamation enter state Q2 to proces factorial
                                Token::Exclamation => {
                                    state = States::Q2;
                                }
                                //LeftParentheses should never appear in this state
                                Token::LeftParentheses => return Err(ErrorCalls::UnclosedParentheses), //Left Parantheses without context
                            }
                        }
                    }
                }
                States::Q2 => { //State for factorial
                    //before factorial there can be only value/tree or right parentheses
                    //State Q4 comes right after this match
                    match &token_or_tree {
                        //tree
                        TokenOrTree::Tree { .. } => {
                            stack.push(token_or_tree);
                            //Q4
                        },
                        TokenOrTree::Token { tok } => match tok {
                            //Value
                            Token::Value(i) => {
                                stack.push(
                                    TokenOrTree::Tree {
                                        tree: Tree::Value(*i),
                                    }
                                );
                                //Q4
                            }
                            //RightParentheses
                            Token::RightParentheses => {
                                stack.push(token_or_tree);
                                let tree_from_parentheses = Tree::parentheses(&mut stack, &mut tokens_or_trees);
                                stack.push(tree_from_parentheses?);
                                //Q4
                            }
                            //anything else is not supported by syntax
                            _=> return Err(ErrorCalls::FactorialWithoutArgument), //Factorial without argument
                        },

                    
                    }
                    

                    // State Q4
                    //E,T! -> !(T)
                    //We are not poping Exclamation token from stack since we didnt put it there
                    //but Exclamation token is only thing that leads to this state
                    if let Some(tree_from_stack) = stack.pop(){
                        //We processed what was before ! so it can only be tree of some sort
                        match tree_from_stack {
                            TokenOrTree::Tree {tree} => {
                                let arg = tree;
                                stack.push(
                                    TokenOrTree::Tree {
                                        tree : Tree::UnaryFunction {
                                            kind: UnaryFunctions::Factorial, x : Box::new(arg)
                                        }
                                    }
                                )
                            },
                            TokenOrTree::Token {..} => return Err(ErrorCalls::UnexpectedError) //UnexpectedError
                        }
                    }
                    else { //this thing should never occur
                        return Err(ErrorCalls::UnexpectedError);//UnexpectedError
                    }
                    state = States::QLoop;
                }
            }
        }
        // if we didnt find anything we call parse_multiplication on the partially parsed imput
        // E,(...)-> parse_multiplication(...)
        if state != States::QLoop {
            return Err(ErrorCalls::FactorialWithoutArgument); //factorial without argument
        }
        stack.reverse();
        Tree::parse_multiplication(stack)
    }

     /// # Tree::parse_multiplication
    /// 
    /// Arguments: token_or_trees: Vec<TokenOrTree>
    /// Parses the imput and converts it into enum Tree.
    /// 
    /// `returns` Result<`Tree`, `ErrorCalls`>
    fn parse_multiplication(mut tokens_or_trees: Vec<TokenOrTree>) -> Result<Tree, ErrorCalls> {
        if tokens_or_trees.len() == 0 {
            return Err(ErrorCalls::BinaryFuncionWithoutArgument);
        }
        let mut stack: Vec<TokenOrTree> = Vec::new();
        while let Some(token_or_tree) = tokens_or_trees.pop() {
            match &token_or_tree {
                //If tree push tree onto stack
                TokenOrTree::Tree { .. } => stack.push(token_or_tree),
                //If token call ParsePower on the rest of imput and on the stack
                TokenOrTree::Token { tok } => match tok {
                    Token::Star => {  //..............................................................................
                        stack.reverse();
                        return Ok(Tree::BinaryFunction {
                            kind: tok.to_binary(),
                            x: Box::new(Tree::parse_multiplication(tokens_or_trees)?),
                            y: Box::new(Tree::parse_multiplication(stack)?),
                        });
                    }
                    Token::Slash => {  //..............................................................................
                        stack.reverse();
                        return Ok(Tree::BinaryFunction {
                            kind: tok.to_binary(),
                            x: Box::new(Tree::parse_multiplication(tokens_or_trees)?),
                            y: Box::new(Tree::parse_multiplication(stack)?),
                        });
                    }
                    //if its stronger operarion than */, then push them onto stack
                    Token::Pow | Token::Sqrt => stack.push(token_or_tree),
                    //Everything else was proccesed during parse_addition phase
                    _ => return Err(ErrorCalls::UnexpectedError), //UnexpectedError
                },
            }
        }
        // if we havent found any division or multiplication we call parse power on the imput
        stack.reverse();
        Tree::parse_power(stack)
    }

     /// # Tree::parse_power
    /// 
    /// Arguments: token_or_trees: Vec<TokenOrTree>
    /// Parses the imput and converts it into enum Tree.
    /// 
    /// `returns` Result<`Tree`, `ErrorCalls`>
    fn parse_power(mut tokens_or_trees: Vec<TokenOrTree>) -> Result<Tree, ErrorCalls> {
        if tokens_or_trees.len() == 0 {
            return Err(ErrorCalls::BinaryFuncionWithoutArgument);
        }
        let mut stack: Vec<TokenOrTree> = Vec::new();
        while let Some(token_or_tree) = tokens_or_trees.pop() {
            match &token_or_tree {
                //If tree push tree onto stack
                TokenOrTree::Tree { .. } => stack.push(token_or_tree),
                //If token call ParsePower on the rest of imput and on the stack
                TokenOrTree::Token { tok } => match tok {
                    Token::Pow | Token::Sqrt => {
                        stack.reverse();
                        return Ok(Tree::BinaryFunction {
                            kind: tok.to_binary(),
                            x: Box::new(Tree::parse_power(tokens_or_trees)?),
                            y: Box::new(Tree::parse_power(stack)?),
                        });
                    }
                    //everithing besides power and root has already been processed in prior phases
                    _ => return Err(ErrorCalls::UnexpectedError),//UnexpectedError
                },
            }
        }
        //if we havent found anything then there should be only 1 thing on stack
        //That being the final tree
        
        if stack.len() == 1 {
            match stack.pop().unwrap() {
                TokenOrTree::Tree { tree } => Ok(tree),
                _ => Err(ErrorCalls::UnexpectedError), //UnexpectedError
            }
        } else {
            Err(ErrorCalls::UnconectedValues) //Values without functions
        }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Tree, ErrorCalls> {
        Tree::parse_addition(TokenOrTree::create(tokens))
    }

    /// # Tree::calculate
    /// 
    /// Takes tree and calculates the value of the expession within it.
    pub fn calculate(&self) -> f64 {
        match self {
            Tree::BinaryFunction { kind, x, y } => kind.execute(x.calculate(), y.calculate()),
            Tree::UnaryFunction { kind, x } => kind.execute(x.calculate()),
            Tree::Value(i) => *i,
        }
    }
}

/// # Tree
/// Enum representing the expression in calculator.
#[derive(Clone)]
pub enum Tree {
    BinaryFunction {
        kind: BinaryFunctions,
        x: Box<Tree>,
        y: Box<Tree>,
    },
    UnaryFunction {
        kind: UnaryFunctions,
        x: Box<Tree>,
    },
    Value(f64),
}

/// # TokenOrTree
/// Enum capable of hoding both enum Token and enum Tree.
/// Used as intermediate value in parsing.
#[derive(Clone)]
enum TokenOrTree {
    Tree { tree: Tree },
    Token { tok: Token },
}

impl TokenOrTree {
    /// # TokenOrTree::create
    /// 
    /// Arguments: tokens: Vec<Token>
    /// 
    /// Takes in vector of enum Token and parses it to vector of enum TokenOrTree.
    /// `returns` Vec<`TokenOrTree`>
    pub fn create(mut tokens: Vec<Token>) -> Vec<TokenOrTree> {
        tokens.reverse();
        let mut tokens_or_trees: Vec<TokenOrTree> = Vec::new();
        while let Some(token) = tokens.pop() {
            tokens_or_trees.push(TokenOrTree::Token { tok: token });
        }
        tokens_or_trees
    }
}



