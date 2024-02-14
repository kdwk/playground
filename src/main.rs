// use std::{sync::mpsc::channel, thread, time::Duration};

// use gtk4::prelude::*;

// trait ButtonModifiers {
//     fn label(self, label: &str) -> Self;
// }

// impl ButtonModifiers for gtk4::Widget {
//     fn label(self, label: &str) -> Self {
//         if let Ok(button) = self.clone().downcast::<gtk4::Button>() {
//             button.set_label(label);
//             return self.upcast::<gtk4::Widget>();
//         }
//         self
//     }
// }

// fn Window(body: impl IsA<gtk4::Widget>) -> gtk4::Window {
//     gtk4::Window::builder().child(&body).build()
// }

// fn Box(
//     orientation: gtk4::Orientation,
//     spacing: i32,
//     append: Vec<gtk4::Widget>,
// ) -> gtk4::Widget {
//     let bo = gtk4::Box::new(orientation, spacing);
//     append.into_iter().for_each(|widget| bo.append(&widget));
//     bo.upcast::<gtk4::Widget>()
// }

// fn Button() -> gtk4::Widget {
//     gtk4::Button::new().upcast()
// }

// fn App(id: Option<&str>, window: impl Fn(&gtk4::Application) -> gtk4::Window + 'static) {
//     let app = gtk4::Application::new(id, Default::default());
//     gtk4::init();
//     app.connect_activate(move |app| {
//         let win = window(app);
//         win.set_application(Some(app));
//         win.present();
//     });
//     let (sender, receiver) = crossbeam::channel::unbounded::<()>();
//     app.connect_shutdown(move |_| _ = sender.send(()));
//     app.run();
//     let loopy = thread::spawn(move || while let Err(_) = receiver.recv() {
//         thread::sleep(Duration::from_millis(100));
//     });
//     _ = loopy.join();
// }
//
// fn main() {
//     // App(None, |_app| {
//     //     Window(
//     //         Box(
//     //             gtk4::Orientation::Vertical,
//     //             5,
//     //             vec![
//     //                 Button().label("Gnome"),
//     //                 Button().label("Label"),
//     //                 Box(gtk4::Orientation::Horizontal, 5, vec![
//     //                     Button().label("What"),
//     //                     Button().label("Where")
//     //                 ])
//     //             ])
//     //     )
//     // })
// }

mod document;

use std::error::Error;

use document::{with, Document, Folder, OpenMode, Project, User};

// // These should all be structs provided by gtk-rs
// enum Orientation {
//     Vertical,
//     Horizontal,
// }
// struct Window {}
// struct Box {}
// struct Button {}

// // This would be the IsA<gtk::Widget> trait had it been object-safe
// trait Widget {}
// impl Widget for Window {}
// impl Widget for Box {}
// impl Widget for Button {}

// trait ButtonModifier {
//     fn label(&self, label: &str) -> &Self {
//         // Set label for self
//         self
//     }
// }

// impl ButtonModifier for Button {}

// trait WindowModifier {
//     fn title(&self, title: &str) -> &Self {
//         // Set title for self
//         self
//     }
// }

// impl WindowModifier for Window {}

// fn Window(body: impl Widget) -> Window {
//     let window = Window {};
//     // Set the child of window
//     window
// }

// fn Box(orientation: Orientation, spacing: i32, append: Vec<&dyn Widget>) -> Box {
//     let bo = Box {};
//     // Set orientation of bo
//     // Set spacing of bo
//     for widget in append {
//         // bo.append(widget);
//     }
//     bo
// }

// fn Button() -> Button {
//     let button = Button {};
//     button
// }

fn main() -> Result<(), Box<dyn Error>> {
    // Window(Box(
    //     Orientation::Vertical,
    //     5,
    //     vec![
    //         Button().label("Gnome"),
    //         Button().label("Label"),
    //         &Box(
    //             Orientation::Horizontal,
    //             5,
    //             vec![Button().label("What"), Button().label("Where")],
    //         ),
    //     ],
    // ))
    // .title("Test App");
    with(
        Document::new(Folder::User(User::Pictures(vec![])), "1.png"),
        |mut document| {
            document.open(OpenMode::Read)?.launch_with_default_app()?;
            println!("{}", document.path());
            Ok(())
        },
    )?;
    with(
        Document::new(
            Folder::Project(Project::Data.with_id("com", "github.kdwk", "Spidey")),
            "test.txt",
        ),
        |mut document| {
            dbg!(document.name());
            dbg!(document.path());
            document.open(OpenMode::Read)?.launch_with_default_app()
        },
    )?;
    Ok(())
}
