use std::io::{BufRead, Write};

pub mod prelude {
    pub use super::input;
}

pub fn input(prompt: impl AsRef<str>) -> String {
    print!("{}", prompt.as_ref());
    std::io::stdout().flush().unwrap();
    let stdin = std::io::stdin();
    let mut ret = String::new();
    stdin.lock().read_line(&mut ret).unwrap();
    ret.pop();
    ret
}

pub mod test {
    use super::prelude::*;

    use crate::recipe::prelude::*;

    pub fn test1() {
        input("Name: ").log();
    }
}
