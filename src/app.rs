extern crate gtk;

use std::{rc::Rc};

use crate::initialize::initialize::Setting;

use super::{initialize::*, timeline::htl::get_toots, util::html_parse};
use gtk::prelude::*;

pub struct InnerApp {
    pub articles: Vec<Article>,
    pub treeview: Rc<gtk::TreeView>,
    pub window: gtk::Window,
    pub builder: gtk::Builder,
    pub setting: Rc<Setting>
}

pub struct Article {
    pub username: String,
    pub description: String,
}

pub struct App {
    data: InnerApp
}

const USERNAME_COL: u8 = 0;
const DESCRIPTION_COL: u8 = 1;

fn create_model(articles: Vec<Article>) -> gtk::ListStore {
  let column_types   = [gtk::Type::String, gtk::Type::String];
  let store = gtk::ListStore::new(&column_types);
  
  let _ = articles.iter().for_each(|article| {
    let iter = store.insert(-1);
    store.set_value(&iter, USERNAME_COL.into(), &article.username.to_value() as &gtk::Value);
    store.set_value(&iter, DESCRIPTION_COL.into(), &article.description.to_value() as &gtk::Value);
  });

  store
}

impl App {
    pub fn new() -> Self {
        // glib::set_program_name(APP_NAME.into());

        gtk::init().expect("Failed to initialize GTK.");

        let articles = Vec::new();

        let setting = initialize::initialize().ok().unwrap();

        let ui = include_str!("layout/main.ui");
        let builder = gtk::Builder::new_from_string(ui);

        let treeview: gtk::TreeView = builder.get_object("treeview").unwrap();

        let name_column_num: u16 = 0;
        let description_column_num: u16 = 1;

        let name_column   = gtk::TreeViewColumn::new();
        name_column.set_title("Username");
        
        let cell_renderer = gtk::CellRendererText::new();
        name_column.pack_start(&cell_renderer, true);
        name_column.add_attribute(&cell_renderer, "text", name_column_num.into());

        let description_column   = gtk::TreeViewColumn::new();
        description_column.set_title("Description");

        let cell_renderer = gtk::CellRendererText::new();
        description_column.pack_start(&cell_renderer, true);
        description_column.add_attribute(&cell_renderer, "text", description_column_num.into());

        treeview.append_column(&name_column);
        treeview.append_column(&description_column);
        
        let window: gtk::Window = builder.get_object("main_window").unwrap();
        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();

        let load_button: gtk::Button = builder.get_object("load_button").unwrap();

        let data = InnerApp{articles, treeview: Rc::new(treeview), window, builder, setting: Rc::new(setting) };

        let setting = data.setting.clone();
        let treeview = data.treeview.clone();
        load_button.connect_button_release_event(move |_, _| {
          reload(&setting, &treeview);
            gtk::Inhibit(false)
        });

        Self { data: data }
    }

    pub fn start(self) -> Self {

        gtk::main();

        self
    }
}

fn reload(setting: &Setting, treeview: &gtk::TreeView) {
  let is = setting.instance_settings.get(0).map(|is|is);
  let toots = is.map(|is| get_toots(is));

  let articles: Vec<Article> = toots
      .unwrap()
      .unwrap()
      .into_iter()
      .map(|t| Article {
          username: t.account.display_name.clone(),
          description: html_parse::html_to_text(t.content.as_ref()).clone(),
      })
      .collect();

  treeview.set_model(Some(&create_model(articles)));

  treeview.set_size_request(200, 200);
  treeview.show_all();
}