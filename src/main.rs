use std::io::{self, Write};
use std::process::Command;

const MEOW: [(u8, u8, u8); 5] = [
    (255, 255, 255),
    (255, 255, 255),
    (255, 255, 255),
    (255, 255, 255),
    (255, 255, 255),
];

fn prompt() -> String {
    let mut out = String::new();
    for (i, ch) in "sillyshell".chars().enumerate() {
        let (r, g, b) = MEOW[i % MEOW.len()];
        out.push_str(&format!("\x1b[38;2;{r};{g};{b}m{ch}"));
    }
    out.push_str("\x1b[0m > ");
    out
}

fn main() -> io::Result<()> {
    loop {
        print!("{}", prompt());
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
        if cmd == "cd" {
            let target = splitinput.next().unwrap_or("/");
            if let Err(e) = std::env::set_current_dir(target) {
                eprintln!("sillyshell: cd: {target}: {e}");
            }
            continue;
        }
        if cmd == "blahaj" {
            const MEOW: [(u8, u8, u8); 5] = [
                (91, 206, 250),  // blue
                (245, 169, 184), // pink
                (255, 255, 255), // white
                (245, 169, 184), // pink
                (91, 206, 250),  // blue
            ];
            print!("{}", prompt());
            continue;
        }
        if cmd == "exit" {
            break;
        }
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
