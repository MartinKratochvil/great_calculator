enum InfixToken{
    Value(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Sin,
    Cos,
    Caret,
    Root,
    ExclamMark,
    RightParethesies,
    LeftParenthesies,
}

impl InfixToken{
    fn ToPrefix(&self) -> PrefixToken{
        match self {
            InfixToken::Value(i) => PrefixToken::Value(*i),
            InfixToken::Plus => PrefixToken::Plus,
            InfixToken::Minus => PrefixToken::Minus,
            InfixToken::Star => PrefixToken::Star,
            InfixToken::Slash => PrefixToken::Slash,
            InfixToken::Sin => PrefixToken::Sin,
            InfixToken::Cos => PrefixToken::Cos,
            InfixToken::Caret => PrefixToken::Caret,
            InfixToken::Root => PrefixToken::Root,
            InfixToken::ExclamMark => PrefixToken::ExclamMark,
            _ => panic!(),
        }
    }
    fn Unary(&self) -> bool{
        match self {
            InfixToken::Sin => true,
            InfixToken::Cos => true,
            _ => false,
        }
    }
    fn Binary(&self) -> bool{
        match self{
            InfixToken::Plus => true,
            InfixToken::Minus => true,
            InfixToken::Star => true,
            InfixToken::Slash => true,
            InfixToken::Caret => true,
            InfixToken::Root => true,
            _ => false,
        }
    }
    fn Value(&self) -> bool{
        match self {
            InfixToken::Value(i) => true,
            _ => false,
        }
    }
}

enum PrefixToken{
    Value(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Sin,
    Cos,
    Caret,
    Root,
    ExclamMark,
}

impl PrefixToken{
    fn make_prefix(mut infix: Vec<InfixToken>) -> Vec<PrefixToken>{
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut reversePrefix: Vec<PrefixToken> = Vec::new();
        infix.reverse();
        while !infix.is_empty() {
            match infix.last(){
                Some(infixToken) => {
                    match infixToken {
                        InfixToken::Value(i) => reversePrefix.push(infixToken.ToPrefix()),
                        InfixToken::Plus => {
                            match stack.last() {
                                Some(prefixToken) => {
                                    match prefixToken {
                                        PrefixToken::Caret => reversePrefix.push(stack.pop()),
                                        PrefixToken::Root => reversePrefix.push(stack.pop()),
                                        PrefixToken::Slash => reversePrefix.push(stack.pop()), // tady se to musí převést ze Some<T> na T
                                        PrefixToken::Star => reversePrefix.push(stack.pop()),
                                        _ => reversePrefix.push(infixToken.ToPrefix()),
                                    }
                                }
                                None => stack.push(infixToken.ToPrefix()),
                            }
                        },
                        InfixToken::Minus => {},
                        InfixToken::Star => {},
                        InfixToken::Slash => {},
                        InfixToken::Sin => {},
                        InfixToken::Cos => {},
                        InfixToken::Caret => {},
                        InfixToken::Root => {},
                        InfixToken::ExclamMark => {},
                        InfixToken::RightParethesies => {},
                        InfixToken::LeftParenthesies => {},
                    }
                }
                None => panic!(),
            };
            infix.pop();
        }
        reversePrefix
    }
}

enum BinaryFunctions {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
}

enum UnaryFunctions{
    Sin,
    Cos,
}

impl BinaryFunctions{
    fn execute(&self, x:f64, y:f64) -> f64{
        match self {
            BinaryFunctions::Add => x + y,
            _ => todo!()
        }
    }
}

impl UnaryFunctions{
    fn execute(&self, x:f64) -> f64{
        match self {
            _ => todo!()
        }
    }
}

impl Expression {
    fn calculate(&self) -> f64{
        match self {
            Expression::BinaryFunction {kind, x, y} => kind.execute(x.calculate(), y.calculate()),
            Expression::UnaryFunction {kind, x} => kind.execute(x.calculate()),
            Expression::Value(i) => *i,
        }
    }
}

enum Expression {
    BinaryFunction{kind:BinaryFunctions, x:Box<Expression>, y:Box<Expression>},
    UnaryFunction{kind:UnaryFunctions, x:Box<Expression>},
    Value(f64),
}

fn TestF(){

}
