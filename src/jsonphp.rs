use gtk::prelude::*;
use gtk::{Builder, ApplicationWindow, Application};
use gtk::Window;
use glib::prelude::*;
use std::fmt::Debug;
use serde_json::{Result, Value};
use std::collections::HashMap;

static mut MOD_BUILDER:Option<gtk::Builder>=None;


pub fn convert_json(param: &[glib::Value])->Option<glib::Value>{
    println!("convert json");

    let mod_builder = get_mod_builder();
    let txt_source_value: gtk::TextView = mod_builder.object("txtSourceValue").unwrap();
    let buffer_source_value: gtk::TextBuffer = txt_source_value.buffer().unwrap();
    let source_value = buffer_source_value.text(&buffer_source_value.start_iter(), &buffer_source_value.end_iter(), false).unwrap();
    println!("{}", source_value);
    let json:HashMap<String, Value> = serde_json::from_str(&source_value).ok()?;
    println!("got the following object back");
    println!("{:?}",json);

    // walk the array and build the json string
    let mut result_string = "";
    println!("printing the results");
    for (key, value) in &json{
        println!("key {:?} has value {:?}",key, value);
        if value.is_string() {
            println!("**THIS IS A STRING **`");
        }
    }
    None
}

pub fn update_ui(builder: &gtk::Builder){
    let box_inner_right: gtk::Box = builder.object("box_right_inner").expect("Couldn't get box");

    unsafe {
        MOD_BUILDER = Some(Builder::from_file("src/ui/jsonphp.ui"));
    }

    let mod_builder = get_mod_builder();
    let md5_box: gtk::Box = mod_builder.object("box_jsonphp").expect("not here");
    box_inner_right.add_child(mod_builder,&md5_box, None);

    mod_builder.connect_signals(|_,handler|{
        println!("{}" , handler);
        match handler {
            "signal_convert_json"=>Box::new(convert_json),
            _ => Box::new(|_| {None})
        }
    });
}

pub fn get_mod_builder() -> &'static gtk::Builder {
    unsafe {
        MOD_BUILDER
            .as_ref()
            .expect("MOD_BUILDER not initialised")
    }
}

