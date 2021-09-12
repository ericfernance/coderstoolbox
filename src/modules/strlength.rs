use gtk::prelude::*;
use gtk::{Builder, ApplicationWindow, Application};
use gtk::Window;
use glib::prelude::*;
use std::fmt::Debug;

use crate::helpers::gtk_helper;


static mut MOD_BUILDER:Option<gtk::Builder>=None;


pub fn calculate_length(param: &[glib::Value])->Option<glib::Value>{
    println!("Calculate length!!");
    let mod_builder = get_mod_builder();
    let txt_source_value: gtk::TextView = mod_builder.object("txtSourceValue").unwrap();
    let source_value: glib::GString = gtk_helper::get_text_view(&txt_source_value);
    let length = source_value.chars().count();
    let length_str: String = format!("{:?}",length);
    let txt_result: gtk::Entry = mod_builder.object("entry_result").unwrap();
    gtk_helper::set_entry_text(&txt_result, length_str);
    None
}

pub fn update_ui(builder: &gtk::Builder){
    let box_inner_right: gtk::Box = builder.object("box_right_inner").expect("Couldn't get box");

    let resources_bytes = include_bytes!("./../views/resources.gresource");
    let resource_data = glib::Bytes::from(&resources_bytes[..]);
    let res = gio::Resource::from_data(&resource_data).unwrap();
    gio::resources_register(&res);

    unsafe {
        //APP_BUILDER = Some(Builder::from_file("src/ui/main.ui"));
        MOD_BUILDER = Some(Builder::from_resource("/com/thisisericrobert/coderstoolbox/stringlength.ui"));
    }

    let mod_builder = get_mod_builder();
    let strlength_box: gtk::Box = mod_builder.object("box_strlength").expect("not here");

    let children = box_inner_right.children();
    children.iter().for_each(|x| box_inner_right.remove(x));
    box_inner_right.add_child(mod_builder,&strlength_box, None);
    mod_builder.connect_signals(|_,handler|{
        println!("{}" , handler);
        match handler {
            "signal_calculate_length"=>Box::new(calculate_length),
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

