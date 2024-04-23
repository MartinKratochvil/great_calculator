use indexmap::IndexMap;
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Env, EventCtx, Widget, WidgetExt, WindowDesc};
use great_calculator::parser::{Token, Tree};


type InputHandler = fn(&mut Vec<Token>, &mut String, String);
static COLUMNS: i32 = 5;


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

    let token_map: HashMap<String, Token> = HashMap::from([
        ("sin".to_string(), Token::Sin),
        ("cos".to_string(), Token::Cos),
        ("!".to_string(), Token::Exclamation),
        ("^".to_string(), Token::Pow),
        ("√".to_string(), Token::Sqrt),
        ("*".to_string(), Token::Star),
        ("/".to_string(), Token::Slash),
        ("+".to_string(), Token::Plus),
        ("-".to_string(), Token::Minus),
        ("(".to_string(), Token::LeftParentheses),
        (")".to_string(), Token::RightParentheses)
    ]);

    let tokens = Arc::new(Mutex::new(Vec::new()));
    let num_buff = Arc::new(Mutex::new(String::new()));

    //enjoy this abomination :)
    let mut dynamic_buttons = Flex::column();
    for (key, func) in event_map {
        let tokens_clone = Arc::clone(&tokens);
        let num_buff_clone = Arc::clone(&num_buff);
        let button = Button::new(key.clone())
            .on_click(move |_, _, _| {
                let mut tokens_lock = tokens_clone.lock().unwrap();
                let mut num_lock = num_buff_clone.lock().unwrap();
                func(&mut tokens_lock, &mut num_lock, key.clone());
            });
        dynamic_buttons.add_child(button);
    }

    let window = WindowDesc::new(dynamic_buttons)
        .title("Dynamic Buttons Example")
        .window_size((300.0, 200.0));

    AppLauncher::with_window(window)
        .launch(())
        .expect("Failed to launch application");
    //finally end of this hell
}


fn handle_number(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if let Some(digit) = label.chars().last() {
        num_buff.push(digit);
        //TODO: print digit
    }
}

fn handle_point(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    if let Some(dec_point) = label.chars().last() {
        if !num_buff.contains(dec_point) {
            num_buff.push(dec_point);
            //TODO: print dot
        }
    }
}

fn handle_binary_func(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
}

fn handle_minus(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
}

fn handle_gon_func(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
}

fn handle_factorial(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
}

fn handle_open_bracket(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
}

fn handle_close_bracket(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
}

fn handle_delete(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
}

fn handle_clear(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    push_num_buff(tokens, num_buff);
}

fn handle_calculate(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
    println!("numbuff: {} - tokens: {}", num_buff, tokens.len());
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

    match num_buff.parse::<f64>() {
        Ok(number) => {
            tokens.push(Token::Value(number));
            num_buff.clear();
            return true;
        }
        Err(err) => {
            println!("Failed to parse: {}", err);
            num_buff.clear();
            return false;
        }
    }
}



