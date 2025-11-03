use react::prelude::*;
use stdext::prelude::*;

fn main() -> Result<()> {
    render(download("https://www.rust-lang.org"))
}
