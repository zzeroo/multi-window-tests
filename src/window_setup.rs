use glib::translate::ToGlibPtr;
use gobject_sys;
use gtk;
use gtk::prelude::*;


// Basic Setup des Fensters
fn window_setup(window: &gtk::Window) {
    let window_title = format!("{} {}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"));
    window.set_title(&window_title);
    window.set_default_size(1024, 600);
    window.set_border_width(10);

    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(130.0);

    match ::std::env::var("XMZ_HARDWARE") {
        Ok(_) => {
            window.fullscreen();
        }
        Err(_) => {}
    }
}

pub fn index(builder: &gtk::Builder, title: &'static str) {
    let window: gtk::Window = builder.get_object("window_setup").unwrap();
    let label_window_setup_title: gtk::Label = builder.get_object("label_window_setup_title").unwrap();
    let button_setup: gtk::Button = builder.get_object("button_setup").unwrap();
    let button_back: gtk::Button = builder.get_object("button_back").unwrap();

    window_setup(&window);

    label_window_setup_title.set_text(title);

    let signal_id = button_setup.connect_clicked(move |_| {
        println!("{}", title);
    });

    button_back.connect_clicked(clone!(window, button_setup, signal_id => move |_| {
        unsafe {
            let instance = button_setup.to_glib_none().0;
            if gobject_sys::g_signal_handler_is_connected(instance, signal_id) == 1 {
                gobject_sys::g_signal_handler_disconnect(instance, signal_id);
            }
        }
        window.hide();
    }));

    window.show_all();
}
