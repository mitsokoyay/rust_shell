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
        if input.is_empty() {
            continue;
        }
        let mut splitinput = input.split_whitespace();
        let Some(cmd) = splitinput.next() else {
            continue;
        };
        match Command::new(cmd).args(splitinput).status() {
            Ok(_status) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                eprintln!("sillyshell: command not found: {cmd}");
            }
            Err(e) => {
                eprintln!("sillyshell: {cmd}: {e}");
            }
        }
    }
    Ok(())
}
