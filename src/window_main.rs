use gtk;
use gtk::prelude::*;
use window_setup;


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

pub fn index() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = gtk::Builder::new_from_string(include_str!("main.ui"));
    let window: gtk::Window = builder.get_object("window_main").unwrap();
    let label_window_main_title: gtk::Label = builder.get_object("label_window_main_title").unwrap();
    let button_type_a: gtk::Button = builder.get_object("button_type_a").unwrap();
    let button_type_b: gtk::Button = builder.get_object("button_type_b").unwrap();

    window_setup(&window);

    label_window_main_title.set_text("Main Window");

    button_type_a.connect_clicked(clone!( builder => move |_| {
        window_setup::index(&builder, "Type A");
    }));

    button_type_b.connect_clicked(clone!( builder => move |_| {
        window_setup::index(&builder, "Type B");
    }));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
