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

// fn test(a: &[&str]) {
//     for b in a {
//         println!("{b}");
//     }
// }

#![allow(unused_imports)]
mod document;
mod recipe;
mod whoops;

use std::{error::Error, fmt::Display, io::Write, ops::Sub, thread};

use crate::{
    document::{
        with, Create, Document, FileSystemEntity,
        Folder::{self, Project, User},
        LinesBufReaderFileExt, Map, Mode,
        Project::{Config, Data},
        ResultDocumentBoxErrorExt,
        User::{Documents, Downloads, Pictures},
    },
    recipe::{example::test, Discard, Log, Pass, Pipe, Recipe, Runnable, Step},
    whoops::{attempt, Catch, IntoWhoops, Whoops},
};

fn main() {
    // with(
    //     &[
    //         Document::at(User(Pictures(&[])), "1.png", Create::No),
    //         Document::at(User(Pictures(&[])), "42-44.png", Create::No),
    //         Document::at(
    //             User(Pictures(&["Across the Spider-verse"])),
    //             "thumb0404.png",
    //             Create::No,
    //         )
    //         .alias("pic"),
    //         Document::at(User(Downloads(&[])), "gdb.txt", Create::No),
    //     ],
    //     attempt(|mut d: Map| {
    //         println!("{}", d["1.png"].name());
    //         d["pic"].launch_with_default_app()?;
    //         d["gdb.txt"]
    //             .append(b"Something\nto be added")?
    //             .launch_with_default_app()?
    //             .lines()?
    //             .print()?;
    //         Ok(())
    //     })
    //     .catch(|error| eprintln!("{:?}", error)),
    // );

    // println!(
    //     "{}",
    //     Document::at(User(Pictures(&[])), "2.png", Create::No).suggest_rename()
    // );

    // attempt(|_| {
    //     let doc = Document::at(User(Pictures(&[])), "2.png", Create::No)?;
    //     println!("{}", doc.name());
    //     Ok(())
    // })
    // .catch(|error| {
    //     Document::at(User(Documents(&[])), "error.txt", Create::OnlyIfNotExists)?
    //         .append(error.to_string().as_bytes())?;
    //     Ok(())
    // })
    // .catch(|error| eprintln!("{error}"))
    // .run(())
    // .discard();

    // let ha = String::from("ha");
    // vec!["a", "b", "c"].into_iter().for_each(|str| {
    //     attempt(|_| {
    //         println!("{ha}");
    //         str.find("a")
    //     })
    //     .catch(|_error| eprintln!("Ha this works"))
    //     .run(())
    //     .discard()
    // });
    // thread::spawn(
    //     attempt(|ha| {
    //         println!("{ha}");
    //     })
    //     .pass(ha.clone()),
    // );
    // println!("{ha}");
    // attempt(|_| {
    //     let d = Document::at(User(Pictures(&[])), "1.png", Create::No)?;
    //     thread::spawn(
    //         attempt(|d: Document| {
    //             println!("{}", d.name());
    //         })
    //         .pass(d.clone()),
    //     );
    //     println!("{}", d.extension());
    //     Ok(())
    // })
    // .run(())
    // .discard();

    // attempt(|_: ()| {
    //     None?;
    //     Some(())
    // })
    // .catch(|error| eprintln!("{error}"))
    // .run(())
    // .discard();

    // Document::at(User(Pictures(&[])), "1.png", Create::No)
    //     .unwrap()
    //     .pipe(|d| {
    //         d.name().log();
    //         d
    //     })
    //     .pipe(|d| {
    //         d.extension().log();
    //         d
    //     })
    //     .log();

    // let num2 = 3;
    // let mut recipe1 = Recipe::initially("stringify", |mut num: i32| {
    //     num += num2;
    //     num.to_string()
    // })
    // .then("jump", |str| str + "jump")
    // .then("ha", |str| str + "ha");
    // recipe1
    //     .replace("jump", |str| str + "jumpyjump")
    //     .run(5)
    //     .log();
    // Recipe {
    //     initial_step: Step::action(
    //         "createDoc",
    //         |(folder, filename, create): (Folder, &str, Create)| match Document::at(
    //             folder, filename, create,
    //         ) {
    //             Ok(doc) => Some(doc),
    //             Err(_) => None,
    //         },
    //     ),
    //     steps: vec![
    //         Step::action(
    //             "printName",
    //             attempt(|d: Option<Document>| {
    //                 d.clone()?.name().log();
    //                 d
    //             }),
    //         ),
    //         Step::action(
    //             "printExtension",
    //             attempt(|d: Option<Document>| {
    //                 d.clone()?.extension().log();
    //                 d
    //             }),
    //         ),
    //     ],
    // }
    // .run((User(Pictures(&[])), "1.png", Create::No))
    // .discard();

    test();
}
