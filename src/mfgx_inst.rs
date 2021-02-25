extern crate gtk;
extern crate gio;

// To import all needed traits.
use gtk::{GtkWindowExt, prelude::*};
use gio::prelude::*;

pub struct Mfgx {
    path: &'static str,
    window: gtk::Application,
}

impl Mfgx {
    pub fn new(x: i32, y: i32, _path: &'static str) -> Mfgx {
        let mut mfgx: Mfgx = Mfgx {
            path: _path,
            window: gtk::Application::new(Some("org.kseandi.mfgx"),
                                          gio::ApplicationFlags::FLAGS_NONE)
                                      .expect("Application::new failed :("),
        };
        mfgx.window.connect_activate(move |app| {
            let win = gtk::ApplicationWindow::new(app);
            let hb = gtk::HeaderBar::new();

            win.set_default_size(x, y);
            win.set_titlebar(Some(&hb));

            hb.set_show_close_button(true);

            hb.set_title(Some("Mfgx"));
            hb.set_subtitle(Some(_path));

            win.show_all();
        });
        mfgx
    }
    pub fn run(self: &mut Mfgx) {
        self.window.run(&[]);
    }
}
