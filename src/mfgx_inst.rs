extern crate gtk;
extern crate gio;

use gtk::{GtkWindowExt, MenuBar, MenuButton, prelude::*, subclass::header_bar};
use gio::prelude::*;

// Create UI -> Add context -> Add logic -> Open -> Destroy?

pub struct Mfgx {
    pub header: gtk::HeaderBar,
    pub window: gtk::ApplicationWindow,
}

impl Mfgx {
    pub fn new(application: &gtk::Application) -> Self {
        let header: gtk::HeaderBar = gtk::HeaderBar::new();
        let window: gtk::ApplicationWindow = gtk::ApplicationWindow::new(application);
        let panel: gtk::Paned = gtk::Paned::new(gtk::Orientation::Horizontal);
        let head_menu: gtk::MenuButton = gtk::MenuButton::new();
        let folder_view: gtk::FlowBox = gtk::FlowBox::new();
        let test_fldr: gtk::Image = gtk::Image::from_icon_name(
            Some("folder"), 
            gtk::IconSize::Button);
        let test_button: gtk::Button = gtk::Button::with_label("Console?");
        window.resize(800, 600);

        header.set_show_close_button(true);
        header.set_title(Some("Mfgx"));
        header.pack_end(&head_menu);
        window.set_titlebar(Some(&header));
        
        test_button.connect_clicked(|_| {
            println!("Hi!");
        });
        panel.add(&test_button);
 
        folder_view.insert(&test_fldr, -1);
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