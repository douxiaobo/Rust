use druid::theme::TEXT_COLOR;
use druid::widget::{Align,  Flex, Label,  TextBox};
use druid::{AppLauncher, Lens, Data, Widget, WidgetExt, WindowDesc};
use druid::Color;

#[derive(Clone, Default, PartialEq, Debug,Data,Lens)]
struct Calculator{
    num1: String,
    num2: String,
    result: String,
}

fn build_ui() -> impl Widget< Calculator > {
    let label_num1 = Label::new("Enter the first number:")
    .with_text_color(Color::RED).with_text_size(20.0);

    let textbox_num1 = TextBox::new()
    .with_placeholder("num1").lens(Calculator::num1).background(Color::GREEN);

    let label_num2 = Label::new("Enter the second number:")
    .with_text_color(Color::RED).with_text_size(20.0);

    let textbox_num2 = TextBox::new()
    .with_placeholder("num2").lens(Calculator::num2).background(Color::GREEN);

    let label_result = Label::new(|data: &Calculator, _env:&_| format!("Result: {}", add(data.num1.clone(), data.num2.clone())))
    .with_text_color(Color::BLUE).with_text_size(40.0);

    let layout1 = Flex::row()
        .with_child(label_num1)
        .with_child(textbox_num1);

    let layout2 = Flex::row()
        .with_child(label_num2)
        .with_child(textbox_num2);

    let layout=Flex::column().with_child(layout1).with_child(layout2).with_child(label_result);

    Align::centered(layout).background(Color::WHITE)


}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((400.0, 300.0))
        .title("My Simple Calculator with Rust and Druid App");
    let initial_data = Calculator{
        num1: "0".into(),
        num2: "0".into(),
        result: "0".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}

fn add(num1_str: String, num2_str: String) -> String {
    let num1_i32:i32=match num1_str.parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    let num2_i32:i32=match num2_str.parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    (num1_i32+num2_i32).to_string()
}