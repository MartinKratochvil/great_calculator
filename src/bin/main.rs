use great_calculator::{parser::Tree, *};

use std::collections::HashMap;
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Env, EventCtx, Widget, WidgetExt, WindowDesc};


type MyFunction = fn(String);

fn main() {
    fn function1(label: String) {
        println!("Function 1 called with label: {}", label);
    }

    fn function2(label: String) {
        println!("Function 2 called with label: {}", label);
    }

    let event_map: HashMap<String, MyFunction> = vec![
        ("event1".to_string(), function1 as MyFunction),
        ("event2".to_string(), function2 as MyFunction),
    ]
        .into_iter()
        .collect();

    let mut dynamic_buttons = Flex::column();
    for (key, func) in &event_map {
        let label = Label::new(key.clone());
        let button = Button::new(label.clone())
            .on_click(move |_, _, _| {
                // Call the associated function with the button label
                func(label.to_string());
            });
        dynamic_buttons.add_child(button);
    }

    let window = WindowDesc::new(dynamic_buttons)
        .title("Dynamic Buttons Example")
        .window_size((300.0, 200.0));

    AppLauncher::with_window(window)
        .launch(())
        .expect("Failed to launch application");
}
