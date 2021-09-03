use gtk::prelude::*;
use gtk::{Builder, ApplicationWindow, Application};
use gtk::Window;
use glib::prelude::*;
use std::fmt::Debug;
use glib::GString;

pub fn get_text_view(text_view: &gtk::TextView)->GString{
    println!("get text view");
    let buffer_source_value: gtk::TextBuffer = text_view.buffer().unwrap();
    buffer_source_value.text(&buffer_source_value.start_iter(), &buffer_source_value.end_iter(), false).unwrap()
}

pub fn set_entry_text(entry: &gtk::Entry, text: String){
    let buffer_result: gtk::EntryBuffer = entry.buffer();
    buffer_result.set_text(&text);
}