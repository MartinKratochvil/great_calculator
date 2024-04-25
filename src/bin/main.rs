use indexmap::IndexMap;
use std::sync::{Arc, Mutex};
use druid::widget::{Button, Flex, Label, CrossAxisAlignment, Container};
use druid::{AppLauncher, WidgetExt, WindowDesc, Color, Data, Lens};
use great_calculator::parser::{Token, Tree};


/// `InputHandler` :type - type for handers
/// #
type InputHandler = fn(&mut Vec<Token>, &mut String, String);
/// `COLUMNS` :i32 - number of columns of buttons
/// #
static COLUMNS: i32 = 5;

/// `Calculator` :struct
/// #
/// `display`: String - text of inputted data
#[derive(Clone, Data, Lens)]
struct Calculator {
    display: String,
}


///`main` function called after running program
///#
/// generates components and load app window
/// creates vector of inputs
fn main() {
    let event_map: IndexMap<String, InputHandler> = IndexMap::from([
        ("sin".to_string(), handle_gon_func as InputHandler),
        ("cos".to_string(), handle_gon_func as InputHandler),
        ("!".to_string(), handle_factorial as InputHandler),
        ("C".to_string(), handle_clear as InputHandler),
        ("⌫".to_string(), handle_delete as InputHandler),

        ("7".to_string(), handle_number as InputHandler),
        ("8".to_string(), handle_number as InputHandler),
        ("9".to_string(), handle_number as InputHandler),
        ("^".to_string(), handle_binary_func as InputHandler),
        ("√".to_string(), handle_binary_func as InputHandler),

        ("4".to_string(), handle_number as InputHandler),
        ("5".to_string(), handle_number as InputHandler),
        ("6".to_string(), handle_number as InputHandler),
        ("*".to_string(), handle_binary_func as InputHandler),
        ("/".to_string(), handle_binary_func as InputHandler),

        ("1".to_string(), handle_number as InputHandler),
        ("2".to_string(), handle_number as InputHandler),
        ("3".to_string(), handle_number as InputHandler),
        ("+".to_string(), handle_binary_func as InputHandler),
        ("-".to_string(), handle_binary_func as InputHandler),

        (".".to_string(), handle_point as InputHandler),
        ("0".to_string(), handle_number as InputHandler),
        ("(".to_string(), handle_open_bracket as InputHandler),
        (")".to_string(), handle_close_bracket as InputHandler),
        ("=".to_string(), handle_calculate as InputHandler)
    ]);

    let display = Label::new(|data: &Calculator, _env: &_| format!("{}", data.display))
        .with_text_size(64.0)
        .align_right()
        .expand_width();

    let display_container = Container::new(display)
        .fix_height(90.0)
        .padding(20.0)
        .background(Color::rgb8(24,119,242));

    let mut col = Flex::column();
    col.add_child(display_container);

    let mut button_grid = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    let tokens = Arc::new(Mutex::new(Vec::new()));
    let num_buff = Arc::new(Mutex::new(String::new()));
    let mut index = 1;

    for (key, func) in event_map {
        let tokens_clone = Arc::clone(&tokens);
        let num_buff_clone = Arc::clone(&num_buff);
        let button = Button::new(key.clone())
            .on_click(move |_ctx, data: &mut Calculator, _env| {
                let mut tokens_lock = tokens_clone.lock().unwrap();
                let mut num_lock = num_buff_clone.lock().unwrap();
                func(&mut tokens_lock, &mut num_lock, key.clone());
                reload_display(&mut tokens_lock, &mut num_lock, data);
            })
            .border(Color::rgb8(24,119,242), 1.0)
            .expand();

        button_grid.add_flex_child(button.padding(2.0).background(Color::BLACK), 1.0);

        if index == COLUMNS {
            index = 0;
            col.add_flex_child(button_grid, 1.0);
            button_grid = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);
        }

        index += 1;
    }

    let window = WindowDesc::new(col)
        .window_size((400.0, 600.0))
        .title("GGC - Great calculator");

    let calculator = Calculator {
        display: String::from(""),
    };

    AppLauncher::with_window(window)
        .launch(calculator)
        .expect("Failed to launch application");

}

/// handle_number add number to number stack
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `label`: String - label of the pressed button
#[allow(unused_variables)]
fn handle_number(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if let Some(digit) = label.chars().last() {
        match tokens.last() {
            Some(Token::RightParentheses) => return,
            Some(Token::Exclamation) => return,
            _ => {}
        }
        num_buff.push(digit);
    }

}

/// handle_point add decimal point to number stack
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `label`: String - label of the pressed button
#[allow(unused_variables)]
fn handle_point(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if !num_buff.chars().last().is_none() {
        if let Some(dec_point) = label.chars().last() {
            if !num_buff.contains(dec_point) {
                num_buff.push(dec_point);
            }
        }
    }
}

/// handle_binary_func add token of binary operations to tokens
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `label`: String - label of the pressed button
fn handle_binary_func(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if !push_num_buff(tokens, num_buff) && tokens.last().is_none() {
        return;
    }
    match tokens.last() {
        Some(Token::Value(..)) => {}
        Some(Token::Exclamation) => {}
        Some(Token::RightParentheses) => {}
        _ => return
    }

    tokens.push(get_token_from_str(label));
}

/// handle_gon_func add token of goniometric function to tokens
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `label`: String - label of the pressed button
fn handle_gon_func(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
    match tokens.last() {
        Some(Token::Value(..)) => return,
        Some(Token::RightParentheses) => return,
        Some(Token::Exclamation) => return,
        _ => {}
    }

    tokens.push(get_token_from_str(label));
}

/// handle_factorial add token of factorial to tokens
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `label`: String - label of the pressed button
fn handle_factorial(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
    match tokens.last() {
        Some(Token::RightParentheses) => {}
        Some(Token::Value(..)) => {}
        _ => return
    }

    tokens.push(get_token_from_str(label));
}

/// handle_open_bracket add token of opened bracket to tokens
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `label`: String - label of the pressed button
fn handle_open_bracket(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
    match tokens.last() {
        Some(Token::Value(..)) => return,
        Some(Token::Exclamation) => return,
        _ => {}
    }

    tokens.push(get_token_from_str(label));
}

/// handle_close_bracket add token of closed bracket to tokens
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `label`: String - label of the pressed button
fn handle_close_bracket(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
    if let Some(Token::LeftParentheses) = tokens.last() {
        return;
    }

    tokens.push(get_token_from_str(label));
}

/// handle_delete remove last token from tokens
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `label`: String - label of the pressed button
#[allow(unused_variables)]
fn handle_delete(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if !num_buff.is_empty() {
        num_buff.clear();

        return;
    }

    tokens.pop();
}

/// handle_clear clears all tokens
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `label`: String - label of the pressed button
#[allow(unused_variables)]
fn handle_clear(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    tokens.clear();
    num_buff.clear();
}

#[allow(unused_variables)]
fn handle_calculate(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);

    if tokens.len() == 0 {
        return;
    }

    match Tree::parse(tokens.clone()) {
        Ok(tree) => {
            num_buff.push_str(tree.calculate().to_string().as_str())
        }
        Err(er) => {
            num_buff.push_str("ERROR");
        }
    }
    tokens.clear();
}


/// push_num_buff create f64 number from num_buff and pushes it to tokens
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
fn push_num_buff(tokens: &mut Vec<Token>, num_buff: &mut String) -> bool {
    if num_buff.is_empty() {
        return false;
    }

    if !num_buff.contains('.') {
        num_buff.push_str(".0");
    }
    if let Some(last_char) = num_buff.chars().last() {
        if last_char == '.' {
            num_buff.push('0');
        }
    }

    return match num_buff.parse::<f64>() {
        Ok(number) => {
            tokens.push(Token::Value(number));
            num_buff.clear();
            true
        }
        Err(..) => {
            num_buff.clear();
            false
        }
    }
}


/// get_token_from_str
///#
/// param
/// `str`: String - string to match
/// #
/// `return`: Token - matched token
fn get_token_from_str(str: String) -> Token {
    match str.as_str() {
        "sin" => Token::Sin,
        "cos" => Token::Cos,
        "!" => Token::Exclamation,
        "^" => Token::Pow,
        "√" => Token::Sqrt,
        "*" => Token::Star,
        "/" => Token::Slash,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "(" => Token::LeftParentheses,
        ")" => Token::RightParentheses,
        _ => Token::Value(f64::NAN)
    }
}


/// get_str_from_token
///#
/// param
/// `token`: Token - token to match
/// #
/// `return`: String - matched string
fn get_str_from_token(token: Token) -> String {
    match token {
        Token::Sin=> "sin".to_string(),
        Token::Cos => "cos".to_string(),
        Token::Exclamation => "!".to_string(),
        Token::Pow => "^".to_string(),
        Token::Sqrt => "√".to_string(),
        Token::Star => "*".to_string(),
        Token::Slash => "/".to_string(),
        Token::Plus => "+".to_string(),
        Token::Minus => "-".to_string(),
        Token::LeftParentheses => "(".to_string(),
        Token::RightParentheses => ")".to_string(),
        Token::Value(value) => value.to_string()
    }
}


/// reload_display reload the label text from tokens and number buffer
///#
/// params
/// `tokens`: Vec<Token> - vector of inputted values
///
/// `num_buff`: String - number buffer
///
/// `display`: Calculator - label of inputted data

fn reload_display(tokens: &mut Vec<Token>, num_buff: &mut String, display: &mut Calculator) {
    display.display.clear();
    for token in tokens {
        display.display.push_str(get_str_from_token(token.clone()).as_str());
        display.display.push(' ');
    }

    display.display.push_str(num_buff);

    if num_buff == "ERROR" {
        num_buff.clear();
    }
}