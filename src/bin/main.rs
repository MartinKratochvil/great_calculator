use indexmap::IndexMap;
use std::sync::{Arc, Mutex};
use druid::widget::{Button, Flex, Label, CrossAxisAlignment, Container};
use druid::{AppLauncher, WidgetExt, WindowDesc, Color, Data, Lens};
use great_calculator::parser::{Token, Tree};


type InputHandler = fn(&mut Vec<Token>, &mut String, String);
static COLUMNS: i32 = 5;

#[derive(Clone, Data, Lens)]
struct Calculator {
    display: String,
}

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
        ("-".to_string(), handle_minus as InputHandler),

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
                reload_display(&mut tokens_lock, data);
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

#[allow(unused_variables)]
fn handle_number(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if let Some(digit) = label.chars().last() {
        match tokens.last() {
            Some(Token::RightParentheses) => return,
            Some(Token::Exclamation) => return,
            _ => {}
        }
        num_buff.push(digit);
        println!("add: {}", label);
        //TODO: print digit
    }

}

#[allow(unused_variables)]
fn handle_point(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if !num_buff.chars().last().is_none() {
        if let Some(dec_point) = label.chars().last() {
            if !num_buff.contains(dec_point) {
                num_buff.push(dec_point);
                println!("add: {}", label);
                //TODO: print dot
            }

        }
    }
}

fn handle_binary_func(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if !push_num_buff(tokens, num_buff) && tokens.last().is_none() {
        return;
    }

    println!("push: {}", label.as_str());
    tokens.push(get_token_from_str(label));
    //TODO: print operation
}

fn handle_minus(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if !push_num_buff(tokens, num_buff) && tokens.last().is_none() {
        return;
    }

    println!("push: {}", label.as_str());
    tokens.push(get_token_from_str(label));
    //TODO: print operation
}

fn handle_gon_func(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
    println!("push: {}", label);
    tokens.push(get_token_from_str(label));
    //TODO: print gon func
}

fn handle_factorial(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
    println!("push: {}", label);
    tokens.push(get_token_from_str(label));
    //TODO: print factorial
}

fn handle_open_bracket(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if !push_num_buff(tokens, num_buff) {
        match tokens.last() {
            Some(Token::RightParentheses) => return,
            Some(Token::Exclamation) => return,
            _ => {}
        }
        println!("push: {}", label);
        tokens.push(get_token_from_str(label));
        //TODO: print bracket

    }
}

fn handle_close_bracket(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if push_num_buff(tokens, num_buff) {
        println!("push: {}", label);
        tokens.push(get_token_from_str(label));
        //TODO: print bracket
    }
}

#[allow(unused_variables)]
fn handle_delete(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if !num_buff.is_empty() {
        num_buff.clear();

        return;
    }

    tokens.pop();
    //Todo: remove last item
}

#[allow(unused_variables)]
fn handle_clear(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    tokens.clear();
    //Todo: clear list
}

#[allow(unused_variables)]
fn handle_calculate(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    println!("numbuff: {} - tokens: {}", num_buff, tokens.len());
    push_num_buff(tokens, num_buff);
    match Tree::parse(tokens.clone()) {
        Ok(tree) => {
            let ans = tree.calculate();
            println!("ANS: {}", ans);
            //Todo: print answer
        }
        Err(er) => {
            println!("ERROR!!!{}", er.to_string());
            //Todo: print error
        }
    }
    tokens.clear();
}

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
        Err(err) => {
            println!("Failed to parse: {}", err);
            num_buff.clear();
            false
        }
    }
}

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


fn reload_display(tokens: &mut Vec<Token>, display: &mut Calculator) {

}