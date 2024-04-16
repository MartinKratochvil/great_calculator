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
enum BinaryFunctions {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
}

#[derive(Clone, Copy)]
enum UnaryFunctions {
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

impl Tree {
    pub fn parse(tokens: Vec<Token>) -> Tree {
        fn parse_addition(mut tokens_or_trees: Vec<TokenOrTree>) -> Tree {
            #[macro_export]
            macro_rules! parentheses {
                ($stack: expr, $tok: expr) => {
                    if let Some(popped) = $stack.pop() {
                        $stack.push(TokenOrTree::Tree {
                            tree: Tree::UnaryFunction {
                                kind: $tok.to_unary(),
                                x: Box::new(match popped {
                                    TokenOrTree::Tree { tree } => tree,
                                    _ => panic!(),
                                }),
                            },
                        })
                    } else {
                        panic!();
                    }
                };
            }
            fn parentheses(
                stack: &mut Vec<TokenOrTree>,
                tokens_or_trees: &mut Vec<TokenOrTree>,
            ) -> TokenOrTree {
                let mut r_par_count = 1;
                while let Some(token_or_tree) = tokens_or_trees.pop() {
                    if r_par_count <=0 {
                        break;
                    }
                    match &token_or_tree {
                        TokenOrTree::Token { tok } => {
                            match tok {
                                Token::RightParentheses => {
                                    r_par_count += 1;
                                    stack.push(token_or_tree);
                                }
                                Token::LeftParentheses => {
                                    let mut argument_for_recursive_calculation_of_parantheses = Vec::new();
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
                                    stack.push(TokenOrTree::Tree{tree: parse_addition(argument_for_recursive_calculation_of_parantheses)});
                                    
                                }
                                _=> stack.push(token_or_tree),
                            }
                        }
                        TokenOrTree::Tree { .. } => {stack.push(token_or_tree)}
                    };
                }
                if let Some(result_tree) = stack.pop() {
                    result_tree
                }
                else {
                    panic!()
                }
            }
            enum States {
                QLoop,
                Q1,
                Q2,
                Q3,
                Q4,
            }
            let mut state = States::QLoop;
            let mut stack = Vec::new();
            while let Some(token_or_tree) = tokens_or_trees.pop() {
                match state {
                    States::QLoop => {
                        match &token_or_tree {
                            //If tree push tree onto stack
                            TokenOrTree::Tree { ..} => stack.push(token_or_tree),
                            //If token call ParsePower on the rest of imput and on the stack
                            TokenOrTree::Token { tok } => {
                                match tok {
                                    //+, ^$ -> E / +(self(input.reverse), self(\1))
                                    Token::Plus => {
                                        stack.reverse();
                                        return Tree::BinaryFunction {
                                            kind: BinaryFunctions::Add,
                                            x: Box::new(parse_addition(tokens_or_trees)),
                                            y: Box::new(parse_addition(stack)),
                                        };
                                    }
                                    Token::Minus => match tokens_or_trees.last() {
                                        Some(next) => match next {
                                            TokenOrTree::Tree { .. } => {
                                                stack.reverse();
                                                return Tree::BinaryFunction {
                                                    kind: BinaryFunctions::Sub,
                                                    x: Box::new(parse_addition(tokens_or_trees)),
                                                    y: Box::new(parse_addition(stack)),
                                                };
                                            }
                                            TokenOrTree::Token { tok } => match tok {
                                                Token::Value(..)
                                                | Token::Exclamation
                                                | Token::RightParentheses => {
                                                    stack.reverse();
                                                    return Tree::BinaryFunction {
                                                        kind: BinaryFunctions::Sub,
                                                        x: Box::new(parse_addition(
                                                            tokens_or_trees,
                                                        )),
                                                        y: Box::new(parse_addition(stack)),
                                                    };
                                                }
                                                _ => parentheses!(stack,tok),
                                            },
                                        },
                                        None => panic!(),
                                    },
                                    //>+, E -> \1
                                    Token::Star | Token::Slash | Token::Pow | Token::Sqrt => {
                                        stack.push(token_or_tree)
                                    }
                                    //v, E->T(v)
                                    Token::Value(i) => stack.push(TokenOrTree::Tree {
                                        tree: Tree::Value(*i),
                                    }),
                                    //sin, T->sin(T)
                                    Token::Sin | Token::Cos => parentheses!(stack, tok),
                                    Token::RightParentheses => {
                                        stack.push(token_or_tree);
                                        let tree_from_parentheses = parentheses(&mut stack, &mut tokens_or_trees);
                                        stack.push(tree_from_parentheses);
                                    }
                                    Token::Exclamation => {
                                        state = States::Q2;
                                    }
                                    Token::LeftParentheses => panic!(),
                                }
                            }
                        }
                    }
                    States::Q1 => {}
                    States::Q2 => {
                        match &token_or_tree {
                            TokenOrTree::Tree { .. } => {
                                stack.push(token_or_tree);
                                //state = States::Q4;
                            },
                            TokenOrTree::Token { tok } => match tok {
                                Token::Value(i) => {
                                    stack.push(
                                        TokenOrTree::Tree {
                                            tree: Tree::Value(*i),
                                        }
                                    );
                                    //state = States::Q4;
                                }
                                Token::RightParentheses => {
                                    stack.push(token_or_tree);
                                    let tree_from_parentheses = parentheses(&mut stack, &mut tokens_or_trees);
                                    stack.push(tree_from_parentheses);
                                    //state = States::Q4;
                                }
                                _=> panic!(),
                            },

                        
                        }
                        

                        //this should be in Q4 but it requires not reading so
                        //it is right behind Q2 since Q3 is in function and therefore it is reduces to transfer
                        if let Some(tree_from_stack) = stack.pop(){
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
                                TokenOrTree::Token {..} => panic!()
                            }
                        }
                        else {
                            panic!();
                        }
                        state = States::QLoop;
                    }
                    States::Q3 => {}
                    States::Q4 => {
                        if let Some(tree_from_stack) = stack.pop(){
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
                                TokenOrTree::Token {..} => panic!()
                            }
                        }
                        else {
                            panic!();
                        }
                    }
                }
            }
            stack.reverse();
            parse_multiplication(stack)
        }

        fn parse_multiplication(mut tokens_or_trees: Vec<TokenOrTree>) -> Tree {
            let mut stack: Vec<TokenOrTree> = Vec::new();
            while let Some(token_or_tree) = tokens_or_trees.pop() {
                match &token_or_tree {
                    //If tree push tree onto stack
                    TokenOrTree::Tree { .. } => stack.push(token_or_tree),
                    //If token call ParsePower on the rest of imput and on the stack
                    TokenOrTree::Token { tok } => match tok {
                        Token::Star | Token::Slash => {
                            stack.reverse();
                            return Tree::BinaryFunction {
                                kind: tok.to_binary(),
                                x: Box::new(parse_multiplication(tokens_or_trees)),
                                y: Box::new(parse_multiplication(stack)),
                            };
                        }
                        Token::Pow | Token::Sqrt => stack.push(token_or_tree),
                        _ => panic!(),
                    },
                }
            }
            stack.reverse();
            parse_power(stack)
        }

        fn parse_power(mut tokens_or_trees: Vec<TokenOrTree>) -> Tree {
            let mut stack: Vec<TokenOrTree> = Vec::new();
            while let Some(token_or_tree) = tokens_or_trees.pop() {
                match &token_or_tree {
                    //If tree push tree onto stack
                    TokenOrTree::Tree { .. } => stack.push(token_or_tree),
                    //If token call ParsePower on the rest of imput and on the stack
                    TokenOrTree::Token { tok } => match tok {
                        Token::Pow | Token::Sqrt => {
                            stack.reverse();
                            return Tree::BinaryFunction {
                                kind: tok.to_binary(),
                                x: Box::new(parse_power(tokens_or_trees)),
                                y: Box::new(parse_power(stack)),
                            };
                        }
                        _ => panic!(),
                    },
                }
            }
            if stack.len() == 1 {
                match stack.pop().unwrap() {
                    TokenOrTree::Tree { tree } => tree,
                    _ => panic!(),
                }
            } else {
                panic!();
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



