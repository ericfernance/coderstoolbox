use gtk::prelude::*;
use gtk::{Builder, ApplicationWindow, Application};
use gtk::Window;
use glib::prelude::*;
use std::fmt::Debug;


static mut MOD_BUILDER:Option<gtk::Builder>=None;

pub fn calculate_md5(param: &[glib::Value])->Option<glib::Value>{
    println!("calculate md5");
    println!("{:?}",param);
    let mod_builder = get_mod_builder();
    println!("{:?}", mod_builder);

    let txt_source_value: gtk::TextView = mod_builder.object("txtSourceValue").unwrap();
    println!("{:?}", txt_source_value);
    let buffer_source_value: gtk::TextBuffer = txt_source_value.buffer().unwrap();
    println!("{:?}", buffer_source_value);

    let s = buffer_source_value.text(&buffer_source_value.start_iter(), &buffer_source_value.end_iter(), false).unwrap();
    println!("{}",s);
    None
}

pub fn update_ui(builder: &gtk::Builder){
    println!("update ui in md5 - add the controls.");
    println!("see if I can open a new window!");
    let box_inner_right: gtk::Box = builder.object("box_right_inner").expect("Couldn't get box");

    unsafe {
        MOD_BUILDER = Some(Builder::from_file("src/ui/md5.ui"));
    }

    let mod_builder = get_mod_builder();
    let md5_box: gtk::Box = mod_builder.object("box_junk").expect("not here");
    box_inner_right.add_child(mod_builder,&md5_box, None);

    mod_builder.connect_signals(|_,handler|{
        println!("handler");
        println!("{}" , handler);
        match handler {
            "signal_calculate_md5"=>Box::new(calculate_md5),
            _ => Box::new(|_| {None})
        }
    });

    // connect the new buttons
    println!("connect the new buttons.");

}

pub fn get_mod_builder() -> &'static gtk::Builder {
    unsafe {
        MOD_BUILDER
            .as_ref()
            .expect("MOD_BUILDER not initialised")
    }
}

