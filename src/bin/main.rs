use std::cell::RefCell;
use indexmap::IndexMap;
use std::collections::{HashMap};
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Env, EventCtx, Widget, WidgetExt, WindowDesc};
use great_calculator::parser::{Token, Tree};


type InputHandler = fn(&mut [Token], String);
type Swag = fn(&mut Token);
static COLUMNS: i32 = 5;


fn main() {
    /*let mut s = Token::Sin;
    fn swag_impl(token: &mut Token) {

    }
    swag_impl(&mut s);*/
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

    let tokens = RefCell::new(Vec::new());

    //enjoy this abomination :)
    let mut dynamic_buttons = Flex::column();
    for (key, func) in event_map {
        let tokens_ref = tokens.clone();
        let button = Button::new(key.clone())
            .on_click(move |_, _, _| {
                func(&mut tokens_ref.borrow_mut(), key.clone());
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


    fn handle_number(tokens: &mut [Token], label: String) {
        println!("Function handle_number called with label: {}", label);
    }

    fn handle_point(tokens: &mut [Token], label: String) {
        println!("Function handle_point called with label: {}", label);
    }

    fn handle_binary_func(tokens: &mut [Token], label: String) {
        println!("Function handle_binary_func called with label: {}", label);
    }

    fn handle_minus(tokens: &mut [Token], label: String) {
        println!("Function handle_minus called with label: {}", label);
    }

    fn handle_gon_func(tokens: &mut [Token], label: String) {
        // Since we can't push to an array slice, this functionality is disabled
        println!("Function handle_gon_func called with label: {}", label);
    }

    fn handle_factorial(tokens: &mut [Token], label: String) {
        println!("Function handle_factorial called with label: {}", label);
    }

    fn handle_open_bracket(tokens: &mut [Token], label: String) {
        println!("Function handle_open_bracket called with label: {}", label);
    }

    fn handle_close_bracket(tokens: &mut [Token], label: String) {
        println!("Function handle_close_bracket called with label: {}", label);
    }

    fn handle_delete(tokens: &mut [Token], label: String) {
        println!("Function handle_delete called with label: {}", label);
    }

    fn handle_clear(tokens: &mut [Token], label: String) {
        println!("Function handle_clear called with label: {}", label);
    }

    fn handle_calculate(tokens: &mut [Token], label: String) {
        println!("Function handle_calculate called with label: {}", label);
    }

}



