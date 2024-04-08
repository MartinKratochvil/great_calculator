use crate::data_parser::InfixToken::LeftParenthesies;

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

#[derive(PartialEq, Debug)]
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

//vektory se dají porovnávat pomocí ==
//potom to definuj pro Vec<InfixToken> ať se to dá jen zavolat na outputu
impl PrefixToken{
    fn make_prefix(mut infix: Vec<InfixToken>) -> Vec<Self>{

        //Přehození faktoriálu

        for mut i in infix.len() - 1 .. 0 {
            let mut r_params_count = 0;
            if infix[i] == InfixToken::ExclamMark{
                if infix[i - 1] == InfixToken::RightParethesies {
                    r_params_count += 1;
                    let mut j = i - 2;
                    while r_params_count != 0 {
                        if infix[j] == InfixToken::RightParethesies{
                            r_params_count += 1;
                        }
                        else if infix[j] == InfixToken::LeftParenthesies {
                            r_params_count -= 1;
                        }
                        j -= 1;
                    }
                    j += 1;
                    for k in i .. j {
                        infix[k] = infix[k-1];
                    }
                    infix[j] = InfixToken::ExclamMark;
                }
                else {
                    let x = infix[i];
                    infix[i] = infix[i - 1];
                    infix[i - 1] = x;
                }
            }
        }



        let mut stack: Vec<InfixToken> = Vec::new();
        let mut reversePrefix: Vec<Self> = Vec::new();
        let mut count = 0;
        while let Some(infix_token) = infix.last() {
            count +=1;
            match infix_token{
                InfixToken::Value(..) |
                InfixToken::Sin |
                InfixToken::Cos |
                InfixToken::ExclamMark  => {
                    reversePrefix.push(infix_token.ToPrefix());
                    infix.pop();
                },
                InfixToken::Plus |
                InfixToken::Minus |
                InfixToken::Star |
                InfixToken::Slash |
                InfixToken::Caret |
                InfixToken::Root => {
                    while let Some(stack_last) = stack.last() {
                        if stack_last >= infix_token {
                            reversePrefix.push(stack_last.ToPrefix());
                            stack.pop();
                        }
                        else {
                            break;
                        }
                    }
                    stack.push(*infix_token);
                    infix.pop();
                },

                InfixToken::LeftParenthesies => {
                    stack.push(*infix_token);
                    infix.pop();
                },
                InfixToken::RightParethesies => {
                    while let Some(stack_token) = stack.pop() {
                        if stack_token == InfixToken::LeftParenthesies{
                            stack.pop();
                            break;
                        }
                        reversePrefix.push(stack_token.ToPrefix());
                    }
                    infix.pop();
                },
            };
        }
        while let Some(left_in_stack) = stack.last(){
            reversePrefix.push(left_in_stack.ToPrefix());
            stack.pop();
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


