use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        print!("sillyshell >");
        io::stdout().flush()?;
    }
}
