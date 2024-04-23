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


    fn handle_number(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        match label.parse::<i32>() {
            Ok(num) => {
                println!("pressed: {}", num.to_string());

                if let Some(Token::Value(mut value)) = tokens.last() {
                    tokens.pop();
                    println!("Last value is: {}", value);

                    value *= 10f64;
                    value += num as f64;
                    tokens.push(Token::Value(value));

                    println!("New value is: {}", value);

                }
                else {
                    tokens.push(Token::Value(num as f64));
                    println!("Not a Value variant");
                }
            }
            Err(err) => {
                panic!("{}", err.to_string());
            }
        }
        println!("size: {}", tokens.len());
    }

    fn handle_point(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_point called with label: {}", label);
    }

    fn handle_binary_func(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_binary_func called with label: {}", label);
    }

    fn handle_minus(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_minus called with label: {}", label);
    }

    fn handle_gon_func(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_gon_func called with label: {}", label);
    }

    fn handle_factorial(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_factorial called with label: {}", label);
    }

    fn handle_open_bracket(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_open_bracket called with label: {}", label);
    }

    fn handle_close_bracket(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_close_bracket called with label: {}", label);
    }

    fn handle_delete(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_delete called with label: {}", label);
    }

    fn handle_clear(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_clear called with label: {}", label);
    }

    fn handle_calculate(tokens: &mut Vec<Token>, num_buff: &mut String, label: String) {
        println!("Function handle_calculate called with label: {}", label);
    }

}



