use gtk4::{prelude::*, Orientation};
use gtk4::{glib, Application,ApplicationWindow, Button,Label,Box};
use std::cell::RefCell;
use std::rc::Rc;



const APP_ID: &str = "org.douxiaobo.HelloWorld";

fn main() -> glib::ExitCode {
    // 使用Rc<RefCell<bool>>来包装flag  
    // let flag = Rc::new(RefCell::new(true));
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    // app.connect_activate(move |app| build_ui(&app, flag.clone()));
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

// fn build_ui(app: &Application,flag:Rc<RefCell<bool>>) {
fn build_ui(app: &Application){
    let vbox=Box::builder()
    .orientation(Orientation::Vertical)
    .build();

    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let label=Label::builder()
        .label(&format!("Hello, Rust! {}", APP_ID))
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    vbox.append(&label);
    vbox.append(&button);
    

    // Connect to "clicked" signal of `button`
    // button.connect_clicked(move|_| {
    //     // Set the label to "Hello World!" after the button has been clicked on
    //     let mut borrow_flag = flag.borrow_mut();
    //     *borrow_flag = !*borrow_flag;
    //     let text=if *borrow_flag { "Hello, World!".to_string() } else { "Hello, Rust!".to_string() };
    //     label.set_label(&format!("{} {}", text, APP_ID));
    //     label.queue_draw();        
    // });

    // let is_hello_world=Rc::new(RefCell::new(true));
    // button.connect_clicked(move |_|{
    //     let mut is_hello_world_ref=is_hello_world.borrow_mut();
    //     *is_hello_world_ref=!*is_hello_world_ref;
    //     let text=if *is_hello_world_ref { "Hello, World!".to_string() } else { "Hello, Rust!".to_string() };
    //     label.set_label(&format!("{} {}", text, APP_ID));
    //     label.queue_draw();        
    // });

    

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&vbox)
        .build();

    // Present window
    window.present();
        // 强制执行一次事件循环迭代


    let is_hello_world = Rc::new(RefCell::new(false));

    button.connect_clicked(move |_| {
        let mut is_hello_world_ref = is_hello_world.borrow_mut();
        *is_hello_world_ref = !*is_hello_world_ref;
        let text = if *is_hello_world_ref {
            "Hello, World!".to_string()
        } else {
            "Hello, Rust!".to_string()
        };
        label.set_label(&format!("{} {}", text, APP_ID));
        label.queue_draw();
    });
}