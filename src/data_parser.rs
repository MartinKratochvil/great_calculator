
#[derive(PartialOrd, PartialEq, Copy, Clone)]
enum InfixToken{
    Value(f64),
    RightParethesies,
    LeftParenthesies,
    Minus,
    Plus,
    Slash,
    Star,
    Caret,
    Root,
    Sin,
    Cos,
    ExclamMark,
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

#[derive(PartialEq)]
enum TokenOrExpression{
    Expression(Expression),
    Token(PrefixToken),
}


