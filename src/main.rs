use gtk4::prelude::*;

trait BoxModifiers {
    fn label(self, label: &str) -> Self;
}

impl BoxModifiers for gtk4::Button {
    fn label(self, label: &str) -> Self {
        self.set_label(label);
        self
    }
}

fn Window(body: impl IsA<gtk4::Widget>) -> gtk4::Window {
    gtk4::Window::builder().child(&body).build()
}

fn Box(
    orientation: gtk4::Orientation,
    spacing: i32,
    append: Vec<impl IsA<gtk4::Widget>>,
) -> gtk4::Box {
    let bo = gtk4::Box::new(orientation, spacing);
    append.iter().for_each(|widget| bo.append(widget));
    bo
}

fn Button() -> gtk4::Button {
    gtk4::Button::new()
}

fn main() {
    let mut s = String::from("value");
    s = s.chars().rev().collect();
    println!("{s}");
    Window(Box(
        gtk4::Orientation::Vertical,
        5,
        vec![
            Button().label("Gnome"),
            Button().label("label"),
            Button().label("Press me!"),
        ],
    ))
    .present();
    loop {}
}
