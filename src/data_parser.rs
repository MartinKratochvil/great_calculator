enum InfixToken{
    Value(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Sin,
    Cos,
    RightParethesies,
    LeftParenthesies,
}

enum PrefixToken{
    Value(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Sin,
    Cos,
}

enum BinaryFunctions {
    Add,
    Sub,
    Mul,
    Div,
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


