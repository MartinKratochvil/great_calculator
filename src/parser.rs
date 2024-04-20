use crate::{add, cos, div, fact, mul, pwr, sin, sqrt, sub};

//Token které příjmá kalkulačka
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

#[derive(Clone, Copy)]
pub enum BinaryFunctions {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
}

#[derive(Clone, Copy)]
pub enum UnaryFunctions {
    Sin,
    Cos,
    Factorial,
    Negate,
}

//definování funkcí pro enum BinaryFuncion
impl BinaryFunctions {
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
    fn execute(&self, x: f64) -> f64 {
        match self {
            UnaryFunctions::Sin => sin(x),
            UnaryFunctions::Cos => cos(x),
            UnaryFunctions::Factorial => fact(x),
            UnaryFunctions::Negate => -x,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ErrorCalls{
    UnclosedParentheses,
    ExtraClosingParentheses,
    UnaryFunctionWithoutArgument,
    FactorialWithoutArgument,
    BinaryFuncionWithoutArgument,
    UnconectedValues,
    MathError,
    WTF, //This should never occur
}

impl std::fmt::Display for ErrorCalls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnclosedParentheses => write!(f, "Error: Unclosed parentheses"),
            Self::ExtraClosingParentheses => write!(f, "Error: Extra closing parentheses"),
            Self::UnaryFunctionWithoutArgument => write!(f, "Error: Unary function without argument"),
            Self::FactorialWithoutArgument => write!(f, "Error: Factorial without argument"),
            Self::BinaryFuncionWithoutArgument => write!(f, "Error: Operator without argument"),
            Self::UnconectedValues => write!(f, "Error: Operands without operator"),
            Self::MathError => write!(f, "Error: Math Error"),
            Self::WTF => write!(f, "Error: Something unexpected happened"),
        }
    }
}

impl Tree {
    pub fn parse(tokens: Vec<Token>) -> Result<Tree, ErrorCalls> {
        fn parse_addition(mut tokens_or_trees: Vec<TokenOrTree>) -> Result<Tree, ErrorCalls> {
            #[macro_export]
            macro_rules! put_unary_function {
                ($stack: expr, $tok: expr) => {
                    if let Some(popped) = $stack.pop() {
                        $stack.push(TokenOrTree::Tree {
                            tree: Tree::UnaryFunction {
                                kind: $tok.to_unary(),
                                x: Box::new(match popped {
                                    TokenOrTree::Tree { tree } => tree,
                                    _ => return Err(ErrorCalls::WTF),
                                }),
                            },
                        })
                    } else {
                        return Err(ErrorCalls::UnaryFunctionWithoutArgument); //UnaryFunctionWithout Argument
                    }
                };
            }
            fn parentheses( //This function represents state Q1 and Q3
                stack: &mut Vec<TokenOrTree>,
                tokens_or_trees: &mut Vec<TokenOrTree>,
            ) -> Result<TokenOrTree, ErrorCalls> {
                let mut r_par_count = 1;
                while let Some(token_or_tree) = tokens_or_trees.pop() {
                    // n==0 => state = QLoop
                    if r_par_count <=0 {
                        break;
                    }
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
                                    stack.push(TokenOrTree::Tree{tree: parse_addition(argument_for_recursive_calculation_of_parantheses)?});
                                    
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
                    Err(ErrorCalls::WTF) //wtf
                }
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
                                            x: Box::new(parse_addition(tokens_or_trees)?),
                                            y: Box::new(parse_addition(stack)?),
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
                                                    x: Box::new(parse_addition(tokens_or_trees)?),
                                                    y: Box::new(parse_addition(stack)?),
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
                                                        x: Box::new(parse_addition(
                                                            tokens_or_trees
                                                        )?),
                                                        y: Box::new(parse_addition(stack)?),
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
                                        let tree_from_parentheses = parentheses(&mut stack, &mut tokens_or_trees);
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
                                    let tree_from_parentheses = parentheses(&mut stack, &mut tokens_or_trees);
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
                                TokenOrTree::Token {..} => return Err(ErrorCalls::WTF) //wtf
                            }
                        }
                        else { //this thing should never occur
                            return Err(ErrorCalls::WTF);//wtf
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
            parse_multiplication(stack)
        }

        fn parse_multiplication(mut tokens_or_trees: Vec<TokenOrTree>) -> Result<Tree, ErrorCalls> {
            let mut stack: Vec<TokenOrTree> = Vec::new();
            while let Some(token_or_tree) = tokens_or_trees.pop() {
                match &token_or_tree {
                    //If tree push tree onto stack
                    TokenOrTree::Tree { .. } => stack.push(token_or_tree),
                    //If token call ParsePower on the rest of imput and on the stack
                    TokenOrTree::Token { tok } => match tok {
                        Token::Star | Token::Slash => {
                            stack.reverse();
                            return Ok(Tree::BinaryFunction {
                                kind: tok.to_binary(),
                                x: Box::new(parse_multiplication(tokens_or_trees)?),
                                y: Box::new(parse_multiplication(stack)?),
                            });
                        }
                        //if its stronger operarion than */, then push them onto stack
                        Token::Pow | Token::Sqrt => stack.push(token_or_tree),
                        //Everything else was proccesed during parse_addition phase
                        _ => return Err(ErrorCalls::WTF), //wtf
                    },
                }
            }
            // if we havent found any division or multiplication we call parse power on the imput
            stack.reverse();
            parse_power(stack)
        }

        fn parse_power(mut tokens_or_trees: Vec<TokenOrTree>) -> Result<Tree, ErrorCalls> {
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
                                x: Box::new(parse_power(tokens_or_trees)?),
                                y: Box::new(parse_power(stack)?),
                            });
                        }
                        //everithing besides power and root has already been processed in prior phases
                        _ => return Err(ErrorCalls::WTF),//wtf
                    },
                }
            }
            //if we havent found anything then there should be only 1 thing on stack
            //That being the final tree
            if stack.len() == 1 {
                match stack.pop().unwrap() {
                    TokenOrTree::Tree { tree } => Ok(tree),
                    _ => return Err(ErrorCalls::WTF), //wtf
                }
            } else {
                return Err(ErrorCalls::UnconectedValues); //Values without functions
            }
        }

        parse_addition(TokenOrTree::create(tokens))
    }

    //Funkce pro vypočtení stromu Tree
    pub fn calculate(&self) -> f64 {
        match self {
            Tree::BinaryFunction { kind, x, y } => kind.execute(x.calculate(), y.calculate()),
            Tree::UnaryFunction { kind, x } => kind.execute(x.calculate()),
            Tree::Value(i) => *i,
        }
    }
}

//Rekurzivní enum pro celý výraz
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

#[derive(Clone)]
enum TokenOrTree {
    Tree { tree: Tree },
    Token { tok: Token },
}

impl TokenOrTree {
    pub fn create(mut tokens: Vec<Token>) -> Vec<TokenOrTree> {
        tokens.reverse();
        let mut tokens_or_trees: Vec<TokenOrTree> = Vec::new();
        while let Some(token) = tokens.pop() {
            tokens_or_trees.push(TokenOrTree::Token { tok: token });
        }
        tokens_or_trees
    }
}



