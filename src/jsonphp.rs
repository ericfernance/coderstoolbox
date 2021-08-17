use gtk::prelude::*;
use gtk::{Builder, ApplicationWindow, Application, MessageDialog, DialogFlags, MessageType, ButtonsType};
use gtk::Window;
use glib::prelude::*;
use std::fmt::Debug;
use serde_json::{Result, Value};
use std::collections::HashMap;

static mut MOD_BUILDER:Option<gtk::Builder>=None;

pub fn unpack(key: &str, value: &Value, result_string: &mut String){
    println!("key {:?} has value {:?}",key, value);
    //result_string.push_str("TEST\t");
    if value.is_object() {
        println!("{} is an object", key);
        result_string.push_str(&*format!("\"{}\"=>[",key));
        for (k,v) in value.as_object().unwrap(){
            println!("\t{} has key {} with value {}",key, k, v);
            unpack(k,v,result_string);
        }
        result_string.push_str("],");
    } else {
        println!("{} is not an object", key);
        result_string.push_str(&*format!("\"{}\" => {},", key, value))
    }
}

pub fn convert_json(param: &[glib::Value])->Option<glib::Value>{
    println!("convert json");

    let mod_builder = get_mod_builder();
    let txt_source_value: gtk::TextView = mod_builder.object("txtSourceValue").unwrap();
    let buffer_source_value: gtk::TextBuffer = txt_source_value.buffer().unwrap();
    let source_value = buffer_source_value.text(&buffer_source_value.start_iter(), &buffer_source_value.end_iter(), false).unwrap();
    //let source_value = "{\"foo\":\"bar\",\"fiz\":1,\"deep\":{\"first_name\":\"Eric\", \"last_name\":\"Fernance\",\"awards\":[\"hero\",\"legend\"]}}";
    let json:HashMap<String, Value> = serde_json::from_str(&source_value).map_err(|err: serde_json::Error|{
        println!("There was an error {:?}", err );
        //MessageDialog::new(None::<&Window>,DialogFlags::empty(),MessageType::Info,ButtonsType::Close,"Hello world").run().await();
        let error_dialog = gtk::MessageDialogBuilder::new()
            .modal(true)
            .buttons(gtk::ButtonsType::Close)
            .text("Problem with json")
            .secondary_text("Your json is not valid.")
            .build();

        error_dialog.run();
        error_dialog.close();
    }).ok()?;
    let mut result_string = String::from("[");
    for (key, value) in &json{
        unpack(key, value, &mut result_string);
    }
    result_string.push_str(&*String::from("]"));
    println!("{}", result_string);

    // push the result string into the buffer.
    let buffer_result_value: gtk::TextBuffer = mod_builder.object("buffer_result").unwrap();
    buffer_result_value.set_text(&result_string);
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
        MOD_BUILDER = Some(Builder::from_resource("/com/thisisericrobert/coderstoolbox/jsonphp.ui"));
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

