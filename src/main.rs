use std::{sync::mpsc::channel, thread, time::Duration};

use gtk4::prelude::*;

trait ButtonModifiers {
    fn label(self, label: &str) -> Self;
}

impl ButtonModifiers for gtk4::Widget {
    fn label(self, label: &str) -> Self {
        if let Ok(button) = self.clone().downcast::<gtk4::Button>() {
            button.set_label(label);
            return self.upcast::<gtk4::Widget>();
        }
        self
    }
}

fn Window(body: impl IsA<gtk4::Widget>) -> gtk4::Window {
    gtk4::Window::builder().child(&body).build()
}

fn Box(
    orientation: gtk4::Orientation,
    spacing: i32,
    append: Vec<gtk4::Widget>,
) -> gtk4::Widget {
    let bo = gtk4::Box::new(orientation, spacing);
    append.into_iter().for_each(|widget| bo.append(&widget));
    bo.upcast::<gtk4::Widget>()
}

fn Button() -> gtk4::Widget {
    gtk4::Button::new().upcast()
}

fn App(id: Option<&str>, window: impl Fn(&gtk4::Application) -> gtk4::Window + 'static) {
    let app = gtk4::Application::new(id, Default::default());
    gtk4::init();
    app.connect_activate(move |app| {
        let win = window(app);
        win.set_application(Some(app));
        win.present();
    });
    let (sender, receiver) = crossbeam::channel::unbounded::<()>();
    app.connect_shutdown(move |_| _ = sender.send(()));
    app.run();
    let loopy = thread::spawn(move || while let Err(_) = receiver.recv() {
        thread::sleep(Duration::from_millis(100));
    });
    _ = loopy.join();
}

fn main() {
    let mut s = String::from("value");
    s = s.chars().rev().collect();
    println!("{s}");
    // let app = gtk4::Application::new(None::<String>, Default::default());
    // gtk4::init();
    // app.connect_activate(|app| {
    //     let window = Window(Box(
    //         gtk4::Orientation::Vertical,
    //         5,
    //         vec![
    //             Button().label("Gnome"),
    //             Button().label("label"),
    //             Button().label("Press me!"),
    //         ],
    //     ));
    //     window.set_application(Some(app));
    //     window.present()
    // });
    // app.run();
    // let loopy = thread::spawn(|| loop {});
    // loopy.join();
    App(None, |_app| {
        Window(
            Box(   
                gtk4::Orientation::Vertical,
                5,
                vec![
                    Button().label("Gnome"),
                    Button().label("Label"),
                    Box(gtk4::Orientation::Horizontal, 5, vec![
                        Button().label("What"),
                        Button().label("Where")
                    ])
                ])
        )
    })
}
