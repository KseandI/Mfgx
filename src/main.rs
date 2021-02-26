use gio::{ApplicationExt, prelude::ApplicationExtManual};

extern crate gtk;

mod mfgx_inst;

use gtk::{WidgetExt, prelude::GtkWindowExtManual};
use mfgx_inst::*;

fn main() {
    gtk::init().unwrap_or_else(|_| {panic!("Can't init GTK :(")});
    
    let application = gtk::ApplicationBuilder::new()
        .application_id("org.kseandi.mfgx")
        .flags(Default::default())
        .build();
    application.connect_activate(move |app| {
        let mut new_app = Mfgx::new(app);

        new_app.window.show_all();
        new_app.window.present();
    });

    application.run(&[]);
}
