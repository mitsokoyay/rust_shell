use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    loop {
        print!("sillyshell > ");
        io::stdout().flush()?;
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let input = line.trim();
        let mut splitinput = input.split_whitespace();
        splitinput.next();
    }
    Ok(())
}
