use gtk::{glib, Widget, Application, BoxBuilder, NONE_BUILDABLE};
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, MessageDialog, Label};

mod md5;
mod jsonphp;

static mut APP_BUILDER:Option<gtk::Builder>=None;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );

    // build the ui
    application.connect_activate(build_ui);

    // run the application.
    application.run();
}

fn build_ui(application: &gtk::Application) {
    println!("{}", application);
    let glade_src = include_str!("ui/main.ui");
    //let builder: gtk::Builder = Builder::new();

    unsafe {
        APP_BUILDER = Some(Builder::from_file("src/ui/main.ui"));
    }
    let builder = get_app_builder();
    //builder.add_from_file("ui/main.ui");
    let window: ApplicationWindow = builder.object("window1").expect("Couldn't get window1");

    // connect the buttons.
    let md5_button: Button = builder.object("button_md5").expect("no button");
    let button_json: Button = builder.object("button_json").unwrap();
    md5_button.connect_clicked( |_| {md5::update_ui(get_app_builder())});
    button_json.connect_clicked( |_| {jsonphp::update_ui(get_app_builder())});

    window.set_application(Some(application));
    window.show_all();
}

pub fn get_app_builder() -> &'static gtk::Builder {
    unsafe {
        APP_BUILDER
            .as_ref()
            .expect("APP_BUILDER not initialised")
    }
}