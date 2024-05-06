// use relm4::gtk::Root;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use gtk::prelude::*;
use gtk::glib::clone;
use relm4::gtk::Box;

//这代码失败了。打算继续学习relm4。

struct AppModel{
    number:i32,
    operator:char,
    // widgets:AppWidgets,
}

#[derive(Debug)]
enum AppInput{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    Clear,
    Decimal,
    Negative,
    Positive,
    Percent,
    Backspace,
}

struct AppWidgets{
    label:gtk::Label,
}

// impl AppWidgets {
//     fn new(_windows:gtk::Window)->Self{
//         Self{label:gtk::Label::new(Some("0"))}
//     }
// }

impl SimpleComponent for AppModel {

    type Input = AppInput;    
    type Output = ();
    type Init=i32;
    type Root=gtk::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
        .title("Simple Calculator")
        .default_width(300)
        .default_height(200)
        .build()
    }

    fn init(
        number:Self::Init,
        window:Self::Root,
        sender:ComponentSender<Self>,
    )-> relm4::ComponentParts<Self>{
        let model=AppModel{ number, operator: ' ' };
        let vbox=gtk::Box::builder()
           .orientation(gtk::Orientation::Vertical)
           .spacing(5)
           .build();

        let row1_vbox=gtk::Box::builder()
           .orientation(gtk::Orientation::Horizontal)
           .homogeneous(true)
           .spacing(5)
           .build();
        let one_button=gtk::Button::with_label("1");
        let two_button=gtk::Button::with_label("2");
        let three_button=gtk::Button::with_label("3");
        row1_vbox.append(&one_button);
        row1_vbox.append(&two_button);
        row1_vbox.append(&three_button);

        let row2_vbox=gtk::Box::builder()
           .orientation(gtk::Orientation::Horizontal)
           .homogeneous(true)
           .spacing(5)
           .build();
        let four_button=gtk::Button::with_label("4");
        let five_button=gtk::Button::with_label("5");
        let six_button=gtk::Button::with_label("6");
        // row2_vbox.pack_start(&four_button, true, true, 0);
        // row2_vbox.pack_start(&five_button, true, true, 0);
        // row2_vbox.pack_start(&six_button, true, true, 0);
        row2_vbox.append(&four_button);
        row2_vbox.append(&five_button);
        row2_vbox.append(&six_button);


        let row3_vbox=gtk::Box::builder()
           .orientation(gtk::Orientation::Horizontal)
           .homogeneous(true)
           .spacing(5)
           .build();
        let seven_button=gtk::Button::with_label("7");
        let eight_button=gtk::Button::with_label("8");
        let nine_button=gtk::Button::with_label("9");
        let zero_button=gtk::Button::with_label("0");
        row3_vbox.append(&seven_button);
        row3_vbox.append(&eight_button);
        row3_vbox.append(&nine_button);
        row3_vbox.append(&zero_button);

        let add_button=gtk::Button::with_label("+");
        let subtract_button=gtk::Button::with_label("-");
        let multiply_button=gtk::Button::with_label("*");
        let divide_button=gtk::Button::with_label("/");
        let equals_button=gtk::Button::with_label("=");
        let clear_button=gtk::Button::with_label("C");
        let decimal_button=gtk::Button::with_label(".");
        let negative_button=gtk::Button::with_label("+/-");
        let positive_button=gtk::Button::with_label("+");
        let percent_button=gtk::Button::with_label("%");
        let backspace_button=gtk::Button::with_label("←");

        let label=gtk::Label::new(Some(&model.number.to_string()));
        label.set_margin_all(5);

        window.set_child(Some(&vbox));

        vbox.append(&label);
        // vbox.append(&one_button);
        // vbox.append(&two_button);        
        // vbox.append(&three_button);
        vbox.append(&row1_vbox);
        // vbox.append(&four_button);
        // vbox.append(&five_button);
        // vbox.append(&six_button);
        vbox.append(&row2_vbox);
        // vbox.append(&seven_button);
        // vbox.append(&eight_button);
        // vbox.append(&nine_button);
        // vbox.append(&zero_button);
        vbox.append(&row3_vbox);

        vbox.append(&add_button);
        vbox.append(&subtract_button);
        vbox.append(&multiply_button);
        vbox.append(&divide_button);
        vbox.append(&equals_button);
        vbox.append(&clear_button);
        vbox.append(&decimal_button);
        vbox.append(&negative_button);
        vbox.append(&percent_button);
        vbox.append(&backspace_button);

        one_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::One)));
        two_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Two)));
        three_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Three)));
        four_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Four)));
        five_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Five)));
        six_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Six)));
        seven_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Seven)));
        eight_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Eight)));
        nine_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Nine)));
        zero_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Zero)));
        add_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Add)));
        subtract_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Subtract)));
        multiply_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Multiply)));
        divide_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Divide)));
        equals_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Equals)));
        clear_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Clear)));
        decimal_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Decimal)));
        negative_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Negative)));
        positive_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Positive)));
        percent_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Percent)));
        backspace_button.connect_clicked(clone!(@strong sender => move |_| sender.input(AppInput::Backspace)));



        let widgets=AppWidgets{
            label,
        };
        ComponentParts{ model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        let mut number1:i32=0;
        let mut number2:i32=0;
        match message {
            AppInput::One => {
                self.number=self.number*10+1;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Two => {
                self.number=self.number*10+2;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Three => {
                self.number=self.number*10+3;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Four => {
                self.number=self.number*10+4;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Five => {
                self.number=self.number*10+5;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Six => {
                self.number=self.number*10+6;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Seven => {
                self.number=self.number*10+7;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Eight => {
                self.number=self.number*10+8;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Nine => {
                self.number=self.number*10+9;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Zero => {
                self.number=self.number*10;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Add => {
                self.operator='+';
                number1=self.number;
                self.number=0;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Subtract => {
                self.operator='-';
                number1=self.number;
                self.number=0;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Multiply => {
                self.operator='*';
                number1=self.number;
                self.number=0;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Divide => {
                self.operator='/';
                number1=self.number;
                self.number=0;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Equals => {       //这个代码有问题，卡住了。
                number2=self.number;
                self.number=0;
                if number2==0 && self.operator=='/' {
                    // self.widgets.label.set_text("Error: Divide by zero error");
                    return;
                } else {
                    self.number=match self.operator {
                        '+' => number1+number2,
                        '-' => number1-number2,
                        '*' => number1*number2,
                        '/' => number1/number2,
                        _ => 0,
                    };
                }
                // match self.operator {
                //     '+' => self.number=number1+number2,
                //     '-' => self.number=number1-number2,
                //     '*' => self.number=number1*number2,
                //     '/' => self.number=number1/number2,
                //     _ => todo!("Handle invalid operator"),
                    // {
                    //     if number2!=0 {
                    //         self.number=number1/number2;
                    //     } else {
                    //         Err::<i32, std::string::String>("Divide by zero error".to_string());
                    //     }
                    //     // _ => {
                    //     //     Err("Invalid operator".to_string());
                    //     // }
                    // },
                    // // _=>todo!(),
                    // // _ => {
                    // //     Err("Invalid operator".to_string());
                    // // }
                // }
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Clear => {
                self.number=0;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Decimal => {
                // self.number=self.number*10+0.1;
                if !self.number.to_string().contains('.') {
                    self.number=self.number*10+0;
                    // self.number=self.number*10+0.1;
                }
                self.number=self.number*10+0;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Negative => {
                self.number=-self.number;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Positive => {
                self.number=self.number;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Percent => {
                self.number=self.number/100;
                //self.update_view(&mut self.widgets, sender);
            }
            AppInput::Backspace => {
                self.number=self.number/10;
                //self.update_view(&mut self.widgets, sender);
            }
        }
    }
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {        
        widgets.label.set_text(&self.number.to_string());
    }
}


fn main() {
    let app=RelmApp::new("Simple Calculator");
    app.run::<AppModel>(0);
}

// fn update_cmd(&mut self, input: &relm4::Sender<Self::Input>, output: relm4::Sender<Self::Output>) {}

// fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}

// fn shutdown(&mut self, widgets: &mut Self::Widgets, output: relm4::Sender<Self::Output>) {}
