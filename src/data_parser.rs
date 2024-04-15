enum Token{
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

enum BinaryFunctions {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root
}

enum UnaryFunctions{
    Sin,
    Cos,
    Factorial,
    Negate
}

impl BinaryFunctions{
    fn execute(&self, x:f64, y:f64) -> f64{
        match self {
            BinaryFunctions::Add => lib::add(x,y),
            BinaryFunctions::Sub => lib::sub(x,y),
            BinaryFunctions::Mul => lib::mul(x,y),
            BinaryFunctions::Div => lib:div(x,y),
            BinaryFunctions::Pow => lib::pwr(x,y),
            BinaryFunctions::Root => lib::sqrt(y,x),
        }
    }
}

impl UnaryFunctions{
    fn execute(&self, x:f64) -> f64{
        match self {
            UnaryFunctions::Sin => lib::sin(x),
            UnaryFunctions::Cos => lib::cos(x),
            UnaryFunctions::Factorial => lib::fact(x),
            UnaryFunctions::Negate => -x,
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


