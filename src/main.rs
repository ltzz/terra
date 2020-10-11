extern crate gtk;

use gtk::prelude::*;
use terra::{initialize::*, util::html_parse};
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


    let listbox : gtk::ListBox = builder.get_object("listbox").unwrap();
    let toots = dbg!(toots);

    let _ = toots.unwrap().unwrap().map(
        |items| {
            listbox.set_size_request(200, 200);
            for toot in items {
                
                // let display_name = gtk::Entry::new();
                // display_name.set_text(toot.account.display_name.as_ref());

                let display_name = gtk::Label::new(Some((toot.account.display_name).as_ref()));
                display_name.set_halign(gtk::Align::Start);
                display_name.set_line_wrap(true);

                let text = html_parse::html_to_text(toot.content.as_ref());
                let clabel = gtk::Label::new(Some((text).as_ref()));
                clabel.set_halign(gtk::Align::Start);
                clabel.set_line_wrap(true);

                let hbox : gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                hbox.pack_start(&display_name, true, true, 0);
                hbox.pack_start(&clabel, true, true, 0);

                let r = gtk::ListBoxRow::new();
                r.add(&hbox);
                listbox.prepend(&r);
            }
            listbox.show_all();
        }
    );

    gtk::main();
}