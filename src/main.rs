use gtk::{glib, Widget, Application, BoxBuilder, NONE_BUILDABLE};
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, MessageDialog, Label};

mod md5;

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

    let builder = Builder::from_file("src/ui/main.ui");
    //builder.add_from_file("ui/main.ui");
    let window: ApplicationWindow = builder.object("window1").expect("Couldn't get window1");


    builder.connect_signals(|_,handler|{
        println!("handler");
        println!("{}" , handler);
        Box::new(|_| {None})
    });


    // connect the buttons.
    let md5_button: Button = builder.object("button_md5").expect("no button");
    md5_button.connect_clicked(move |_| {md5::update_ui(&builder);});





    window.set_application(Some(application));
    window.show_all();
}