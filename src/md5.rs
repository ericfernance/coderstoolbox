use gtk::prelude::*;
use gtk::{Builder, ApplicationWindow, Application};
use gtk::Window;
use glib::prelude::*;
use std::fmt::Debug;
use md5;
use md5::Digest;


static mut MOD_BUILDER:Option<gtk::Builder>=None;

pub fn calculate_md5(param: &[glib::Value])->Option<glib::Value>{
    let mod_builder = get_mod_builder();
    let txt_source_value: gtk::TextView = mod_builder.object("txtSourceValue").unwrap();
    let buffer_source_value: gtk::TextBuffer = txt_source_value.buffer().unwrap();
    let source_value = buffer_source_value.text(&buffer_source_value.start_iter(), &buffer_source_value.end_iter(), false).unwrap();
    let hash:Digest = md5::compute(source_value. as_str());
    println!("{:x}", hash);
    let hash_str: String = format!("{:x}",hash);
    println!("{}", hash_str);
    let txt_result: gtk::Entry = mod_builder.object("entry_result").unwrap();
    let buffer_result: gtk::EntryBuffer = txt_result.buffer();
    buffer_result.set_text(&hash_str);
    None
}

pub fn update_ui(builder: &gtk::Builder){
    let box_inner_right: gtk::Box = builder.object("box_right_inner").expect("Couldn't get box");

    let resources_bytes = include_bytes!("./ui/resources.gresource");
    let resource_data = glib::Bytes::from(&resources_bytes[..]);
    let res = gio::Resource::from_data(&resource_data).unwrap();
    gio::resources_register(&res);

    unsafe {
        //APP_BUILDER = Some(Builder::from_file("src/ui/main.ui"));
        MOD_BUILDER = Some(Builder::from_resource("/com/thisisericrobert/coderstoolbox/md5.ui"));
    }

    let mod_builder = get_mod_builder();
    let md5_box: gtk::Box = mod_builder.object("box_md5").expect("not here");

    let children = box_inner_right.children();
    children.iter().for_each(|x| box_inner_right.remove(x));
    box_inner_right.add_child(mod_builder,&md5_box, None);
    mod_builder.connect_signals(|_,handler|{
        println!("{}" , handler);
        match handler {
            "signal_calculate_md5"=>Box::new(calculate_md5),
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

