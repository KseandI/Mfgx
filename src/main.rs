extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window};

mod mfgx_inst;

fn main() {
    let mut mfgx = mfgx_inst::Mfgx::new();
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let msd: MessageDialog = MessageDialog::new(
        None::<&Window>,
        DialogFlags::empty(),
        MessageType::Info,
        ButtonsType::Ok,
        "Hello World",
    );
    msd.run();
}
