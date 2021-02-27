extern crate gtk;
extern crate gio;

use std::borrow::Borrow;

use gtk::{GtkWindowExt, MenuBar, MenuButton, prelude::*, subclass::header_bar};
use gio::prelude::*;

// Create UI -> Add context -> Add logic -> Open -> Destroy?

pub struct Mfgx {
    pub header: gtk::HeaderBar,
    pub window: gtk::ApplicationWindow,
}

impl Mfgx {
    pub fn change_dir(path: &str) {
        // TODO
        println!("{0}", path);
    }
    pub fn new(application: & gtk::Application) -> Self {
        let header: gtk::HeaderBar = gtk::HeaderBar::new();
        let window: gtk::ApplicationWindow = gtk::ApplicationWindow::new(application);
        let panel: gtk::Paned = gtk::Paned::new(gtk::Orientation::Horizontal);
        let add_folder: gtk::Button = gtk::Button::new();
        let add_file: gtk::Button = gtk::Button::new();
        let folder_view: gtk::FlowBox = gtk::FlowBox::new();
        let folder_view_entity: gtk::FlowBoxChild = gtk::FlowBoxChild::new();
        let selected_entity: gtk::Image = gtk::Image::from_icon_name(
            Some("gtk-directory"), 
            gtk::IconSize::Dialog);
        let selected_text: gtk::Label = gtk::Label::new(Some("Directory"));
        let selected_info: gtk::Label = gtk::Label::new(Some("..."));
        let panel_vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        window.set_property_default_width(800);
        window.set_property_default_height(600);

        add_folder.add(&gtk::Image::from_icon_name(
            Some("folder-new"),
            gtk::IconSize::SmallToolbar));
        add_file.add(&gtk::Image::from_icon_name(
            Some("document-new"),
            gtk::IconSize::SmallToolbar));

        header.set_show_close_button(true);
        header.set_title(Some("Mfgx"));
        header.pack_start(&add_folder);
        header.pack_start(&add_file);
        window.set_titlebar(Some(&header));
        
        panel.add(&panel_vbox);
        panel_vbox.add(&selected_entity);

        panel_vbox.set_margin_start(5);
        panel_vbox.set_margin_end(5);

        panel_vbox.add(&selected_text);

        selected_info.set_justify(gtk::Justification::Center);
        selected_info.set_hexpand(true);
        selected_info.set_vexpand(true);
        panel_vbox.add(&selected_info);
 
        folder_view.insert(&folder_view_entity, -1);
        folder_view.set_property_expand(true);
        panel.add(&folder_view);


        window.add(&panel);

        let app = Self {
            header,
            window,
        };
        app
    }
}