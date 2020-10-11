extern crate gtk;

use gtk::prelude::*;
use terra::{initialize::*};
// use terra::ui_util::*;
use terra::http::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let ui = include_str!("layout/main.ui");
    let builder = gtk::Builder::new_from_string(ui);
    
    let window1 : gtk::Window = builder.get_object("main_window").unwrap();
    window1.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window1.show_all();

    let setting = initialize::initialize();

    let array = setting.map(|op| op.instance_settings);

    // let array = dbg!(array);
    let toots = 
    array.map(|iss| {
        iss.get(0).map(|is|
            connection::get_toot(&is.host_name,
                &is.access_token)
        )
    });

    let toots = dbg!(toots);

    gtk::main();
}