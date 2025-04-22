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
mod async_exp;
mod compose;
mod fruits;
mod go;
mod guard_map;
mod linked_list;
mod map;
mod mixture;
mod numbers;
mod object;
mod quicksort;
mod recipe;
mod tree;
mod whoops;

use std::{
    any::Any,
    collections::HashMap,
    default,
    error::Error,
    fmt::Display,
    future::poll_fn,
    io::Write,
    ops::Sub,
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use documents::prelude::*;
use extend::ext;
use object::prelude::*;
use object_derive::{Enum, Object};
use recipe::identity;

use crate::{
    fruits::prelude::*,
    mixture::prelude::*,
    quicksort::prelude::*,
    recipe::{Apply, Discard, Log, Pipe, Recipe, Runnable},
    whoops::{Catch, attempt},
};

use anyhow::Result;
use chrono::prelude::*;
use enclose::enclose;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};
use tokio::{
    join,
    runtime::Handle,
    select,
    task::{self, JoinHandle},
    time::sleep,
};

#[tokio::main]
async fn main() {
    async_exp::test::test2().await;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Object, Clone, Hash, Eq, PartialOrd, Ord)]
struct A {
    i: i32,
}

impl A {
    fn a1(&mut self) {
        self.i = 1;
    }
    fn b1(&mut self) {
        self.i = 2;
    }
    fn c1(&mut self) {
        self.i = 3;
    }
}

#[derive(Debug, Clone, PartialEq, Enum, Hash, Eq, PartialOrd, Ord)]
enum B {
    Something,
    SomethingElse,
}

fn test1() {
    with(
        &[
            Document::at(User(Pictures(&[])), "doesnotexist.png", Create::No),
            Document::at(User(Pictures(&[])), "42-44.png", Create::No),
            Document::at(
                User(Pictures(&["Across the Spider-verse"])),
                "thumb0404.png",
                Create::No,
            )
            .alias("pic"),
            Document::at(User(Downloads(&[])), "gdb.txt", Create::No),
        ],
        |mut d| {
            attempt(|| {
                println!("{}", d["1.png"].name());
                d["pic"].launch_with_default_app()?;
                d["gdb.txt"]
                    .append(b"Something\nto be added")?
                    .launch_with_default_app()?
                    .lines()?
                    .print()?;
                Ok(())
            })
            .catch(|error| eprintln!("{:?}", error))
        },
    );
}

fn test1a() {
    let a: &[&dyn FileSystemEntity] = &[
        &Document::at(User(Pictures(&[""])), "1.png", Create::No),
        &User(Pictures(&["Across the Spider-verse"])),
        &Project(Config(&[]).with_id("com", "gutolution", "Gutolution")),
        &PathBuf::new(),
    ];
    for b in a {
        println!(
            "{:?} {} exist.",
            b,
            if b.exists() { "does" } else { "doesn't" }
        );
    }
}

fn test2() {
    println!(
        "{}",
        Document::at(User(Pictures(&[])), "2.png", Create::No).suggest_rename()
    );
}

fn test3() {
    let mut a = "hi";
    attempt(|| {
        a = "go";
        let doc = Document::at(User(Pictures(&[])), "2.png", Create::No)?;
        println!("{}", doc.name());
        Ok(())
    })
    .catch(|error| {
        attempt(|| {
            Document::at(User(Documents(&[])), "error.txt", Create::OnlyIfNotExists)?
                .append(error.to_string().as_bytes())?;
            Ok(())
        })
        .catch(|error| eprintln!("{error}"))
    })
    .discard();
    println!("{a}")
}

fn test4() {
    let ha = Arc::new("ha".to_string());
    ["a", "b", "c"].into_iter().for_each(|str| {
        attempt(|| {
            println!("{ha}");
            str.find("a")
        })
        .catch(|_error| eprintln!("Ha this works"))
        .discard()
    });
    thread::spawn(enclose!(
        (ha) move || {
            attempt(|| {
                println!("{ha}");
            })
            .discard()
        }
    ));
    println!("{ha}");
}

fn test5() {
    attempt(|| {
        let d = Document::at(User(Pictures(&[])), "1.png", Create::No)?;
        thread::spawn(enclose!(
            (d) move || {
                d.name().log();
            }
        ));
        println!("{}", d.extension());
        Ok(())
    })
    .discard();
    attempt(|| {
        None?;
        Some(())
    })
    .catch(|error| eprintln!("{error}"))
    .discard();
}
fn test6() {
    Document::at(User(Pictures(&[])), "1.png", Create::No)
        .unwrap()
        .pipe(|d| {
            d.name().log();
            d
        })
        .pipe(|d| {
            d.extension().log();
            d
        })
        .log();
}
fn test7() {
    let num2 = 3;
    let mut recipe1 = Recipe::initially("stringify", |mut num: i32| {
        num += num2;
        num.to_string()
    })
    .then("jump", |str| str + "jump")
    .then("ha", |str| str + "ha");
    recipe1
        .replace("jump", |str| str + "jumpyjump")
        .run(5)
        .log();
    Recipe::initially(
        "createDoc",
        |(folder, filename, create)| match Document::at(folder, filename, create) {
            Ok(doc) => Some(doc),
            Err(_) => None,
        },
    )
    .then("printName", |d: Option<Document>| {
        attempt(|| Some(d.clone()?.name().log())).discard();
        d
    })
    .then("printExtension", |d: Option<Document>| {
        attempt(|| Some(d.clone()?.extension().log())).discard();
        d
    })
    .run((User(Pictures(&[])), "1.png", Create::No))
    .discard();
}

fn test8() {
    let a = A { i: 0 }
        .apply(|b| b.a1())
        .apply(|b| b.b1())
        .apply(|b| b.c1());
    println!("{}", a.i);
}
fn test9() {
    let a = mix![
        1,
        "hi",
        Document::at(User(Documents(&[])), "bytes.pdf", Create::No)
    ];
    let mut b = Mixture::new();
    b.add(3);
    b.add("hi");
    b[1].set(A { i: 4 });
    println!("{:?}", b[1].get::<A>());
    let c = mix!["abc", User(Pictures(&[])), A { i: 5 }];
    for mut item in c {
        item.case::<i32>(|int| println!("It's an i32! {int}"))
            .case::<&str>(|string| println!("It's an &str! {string}"))
            .case::<Folder>(|folder| println!("{folder:?}"));
    }
}

async fn test10() -> String {
    let result1 = task::spawn(async {
        for _ in 1..10 {
            sleep(Duration::from_millis(500)).await;
            println!("First task: Hello")
        }
        format!("First task done at {}", Local::now())
    });
    let result2 = task::spawn(async {
        println!("Second task");
        format!("Second task done at {}", Local::now())
    });
    result1.await.unwrap() + &result2.await.unwrap()
}

fn test11<const N: usize>(list: [&dyn std::fmt::Debug; N]) {
    for item in list {
        println!("{item:?}");
    }
}

fn test12() {
    let now = Local::now;
    let apple1 = Apple {
        date_of_picking: now(),
        dimensions: Dimensions::new(5.0, 5.0, 5.0),
    };
}

fn test13() {
    for variant in B::variants() {
        println!("{variant:?}");
    }
    with(
        &[Document::at(
            User(Home(&[])),
            "test_serde",
            Create::OnlyIfNotExists,
        )],
        |mut d| {
            let a = Apple {
                date_of_picking: Local.with_ymd_and_hms(2024, 9, 1, 0, 0, 0).unwrap(),
                dimensions: Dimensions::new(3, 4, 5),
            };
            d["test_serde"].replace_with(to_string_pretty(&a)?.as_bytes())?;
            Ok(())
        },
    )
}

fn test14<'it>(a: impl IntoIterator<Item = &'it dyn FileSystemEntity>) {
    let a = tokio::spawn(async {
        sleep(Duration::from_millis(200)).await;
        4
    });
    with(
        &[Document::at(User(Home(&[])), "test_serde", Create::No)],
        |d| {
            let a: Fruit = from_str(d["test_serde"].content()?.as_str())?;
            println!("{a:?}");
            println!("{}", a.is_ripe());
            // println!("{}", a.class_name());
            Ok(())
        },
    )
}

fn test15() {
    let romania = HashMap::from([
        ("A", vec!["S", "T", "Z"]),
        ("Z", vec!["A", "O"]),
        ("O", vec!["S", "Z"]),
        ("T", vec!["A", "L"]),
        ("L", vec!["M", "T"]),
        ("M", vec!["D", "L"]),
        ("D", vec!["C", "M"]),
    ]);
    let map = map! {
        "A" => any(4),
        "B" => any(vec!["Whee", "Whoops", "Oddwit"]),
        "C" => any(Some("thing")),
    };
    let mut map2 = mixedmap! {
        "A" => 4,
        "B" => "Web",
        "C" => mix!["A".to_string(), Box::new(3), &7]
    };
    let s = Box::new("A");
    let s1 = *s;
    // let a = map2.get_any::<i32>("A");
}
