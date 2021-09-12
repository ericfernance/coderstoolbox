use gtk::{glib, Widget, Application, BoxBuilder, NONE_BUILDABLE};
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, MessageDialog, Label};
use gio::prelude::*;
use std::collections::HashMap;

mod helpers;

mod modules;


static mut APP_BUILDER:Option<gtk::Builder>=None;
static mut COMPONENTS: Option<HashMap<String,String>> = None;

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
    // Load the compiled resource bundle
    let resources_bytes = include_bytes!("./views/resources.gresource");
    let resource_data = glib::Bytes::from(&resources_bytes[..]);
    let res = gio::Resource::from_data(&resource_data).unwrap();
    gio::resources_register(&res);

    unsafe {
        //APP_BUILDER = Some(Builder::from_file("src/ui/main.ui"));
        APP_BUILDER = Some(Builder::from_resource("/com/thisisericrobert/coderstoolbox/main.ui"));
    }
    let builder = get_app_builder();
    //builder.add_from_file("ui/main.ui");
    let window: ApplicationWindow = builder.object("window1").expect("Couldn't get window1");

    // connect the buttons.
    let md5_button: Button = builder.object("button_md5").expect("no button");
    let button_json: Button = builder.object("button_json").unwrap();
    let button_stringlength: Button = builder.object("button_stringlength").unwrap();
    md5_button.connect_clicked( |_| {modules::md5::update_ui(get_app_builder())});
    button_json.connect_clicked( |_| {modules::jsonphp::update_ui(get_app_builder())});
    button_stringlength.connect_clicked(|_| {modules::strlength::update_ui(get_app_builder())});

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