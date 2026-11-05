use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        print!("sillyshell > ");
        io::stdout().flush()?;
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line)?;
        print!("{}", n);
        if n == 0 {
            break;
        }
    }
    Ok(())
}
