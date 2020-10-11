extern crate gtk;

use gtk::prelude::*;

pub fn input_dialog(window_title: &str)-> gtk::Window {

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title(window_title);
    
    // window.connect_delete_event(|_, _| {
    //     gtk::main_quit();
    //     Inhibit(true)
    // });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    
    let entry = gtk::Entry::new();
    hbox.pack_start(&entry, true, true, 5);
    
    let button = gtk::Button::new_with_label("OK");
    button.connect_clicked(move |_| {
        println!("Text: {}", entry.get_text().unwrap());
    });
    
    hbox.pack_start(&button, false, false, 5);

    window.connect_focus_out_event(|win, _| -> gtk::Inhibit {
        win.activate_focus();
        println!("Window unfocused!");
        gtk::Inhibit(false)
    });

    window.add(&hbox);
    window.set_resizable(false);
    // window.activate_focus();
    // window.set_keep_above(true);
    // window.stick();
    // window.show_all();
    return window
}
