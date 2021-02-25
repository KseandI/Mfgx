extern crate gtk;

mod mfgx_inst;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let mut mfgx = mfgx_inst::Mfgx::new(800, 600, "/");
    mfgx.run();
}
