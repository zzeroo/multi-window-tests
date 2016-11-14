extern crate gtk;
extern crate glib;
extern crate gobject_sys;

#[macro_use] mod macros;
mod window_main;
mod window_setup;


fn main() {
    window_main::index();
}
