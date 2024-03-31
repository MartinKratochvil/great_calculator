#[derive(PartialOrd, PartialEq)]
enum InfixToken{
    Value(f64),
    Minus,
    Plus,
    Slash,
    Star,
    Caret,
    Root,
    Sin,
    Cos,
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
            InfixToken::Value(_i) => true,
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

//potom to definuj pro Vec<InfixToken> ať se to dá jen zavolat na outputu
impl PrefixToken{
    fn make_prefix(mut infix: Vec<InfixToken>) -> Vec<PrefixToken>{
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut reversePrefix: Vec<PrefixToken> = Vec::new();
        infix.reverse();
        while let Some(infix_token) = infix.last() {
            match infix_token{
                InfixToken::Value(_i) => {
                    reversePrefix.push(infix_token.ToPrefix());
                    infix.pop();
                },
                InfixToken::Sin |
                InfixToken::Cos |
                InfixToken::ExclamMark => {
                    reversePrefix.push(infix_token.ToPrefix());
                    infix.pop();
                },
                InfixToken::Plus |
                InfixToken::Minus |
                InfixToken::Star |
                InfixToken::Slash |
                InfixToken::Caret |
                InfixToken::Root => {
                    if stack.is_empty() {
                        stack.push(*infix_token)
                    }
                    while stack.last() >= Some(infix_token) {
                        match stack.pop() {
                            Some(stack_token) => reversePrefix.push(stack_token.ToPrefix()),
                            None => panic!(),
                        };
                    }
                    stack.push(*infix_token);
                },

                InfixToken::RightParethesies => {
                    stack.push(*infix_token);
                },
                InfixToken::LeftParenthesies => {
                    while let Some(stack_token) = stack.pop() {
                        if stack_token == InfixToken::RightParethesies{
                            stack.pop();
                            break;
                        }
                        reversePrefix.push(stack_token.ToPrefix());
                    }
                },
            };
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
