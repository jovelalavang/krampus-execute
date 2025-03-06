use gtk::prelude::*;
use gtk::{Button, Entry, TextView, Window, WindowType};

pub struct App {
    window: Window,
    script_entry: Entry,
    execute_button: Button,
    output_view: TextView,
}

impl App {
    pub fn new() -> Self {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Krampus Executor");
        window.set_default_size(400, 300);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let script_entry = Entry::new();
        let execute_button = Button::with_label("Execute");
        let output_view = TextView::new();
        output_view.set_editable(false);

        vbox.pack_start(&script_entry, false, false, 0);
        vbox.pack_start(&execute_button, false, false, 0);
        vbox.pack_start(&output_view, true, true, 0);
        window.add(&vbox);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        Self {
            window,
            script_entry,
            execute_button,
            output_view,
        }
    }

    pub fn run(&self) {
        self.window.show_all();
        gtk::main();
    }
}