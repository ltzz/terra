extern crate gtk;

use gtk::prelude::*;
use terra::{initialize::*, timeline::htl::get_toots, util::html_parse};
// use terra::ui_util::*;

pub struct Data {
    pub articles: Vec<Article>
}

pub struct Article {
    pub username: String,
    pub description: String
}

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

    let listbox: gtk::ListBox = builder.get_object("listbox").unwrap();

    let load_button: gtk::Button = builder.get_object("load_button").unwrap();
    load_button.connect_button_release_event(|_, _| {
        // TODO
        gtk::Inhibit(false)
    });

    let toots = setting.map(|op| 
        op.instance_settings.get(0)
        .map(|is|
            get_toots(is)
        )
    );
    
    
    let articles: Vec<Article> = toots.unwrap().unwrap().unwrap().into_iter()
    .map(|t|
        Article{
            username: t.account.display_name,
            description: html_parse::html_to_text(t.content.as_ref())
        }
    ).collect();
    
    let mut data: Data = Data{articles: articles};
    
    let _ = data.articles.iter().for_each(
        |article| {
                
            // let display_name = gtk::Entry::new();
            // display_name.set_text(toot.account.display_name.as_ref());

            let username = (article.username).as_ref();
            let display_name = gtk::Label::new(Some(username));
            display_name.set_halign(gtk::Align::Start);
            display_name.set_line_wrap(true);

            let text = (article.description).as_ref();
            let clabel = gtk::Label::new(Some(text));
            clabel.set_halign(gtk::Align::Start);
            clabel.set_line_wrap(true);

            let hbox : gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            hbox.pack_start(&display_name, true, true, 0);
            hbox.pack_start(&clabel, true, true, 0);

            let r = gtk::ListBoxRow::new();
            r.add(&hbox);
            listbox.prepend(&r);
            
        }
    );
    
    listbox.set_size_request(200, 200);
    listbox.show_all();

    gtk::main();
}