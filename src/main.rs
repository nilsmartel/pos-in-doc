const ERROR: &'static str = "require 2 arguments <pos> <file>";

fn exit(s: &str) -> ! {
    eprintln!("{}", s);
    std::process::exit(1);
}

fn main() {
    let mut args = std::env::args().skip(1);

    let pos = match args.next() {
        Some(arg) => match arg.parse::<usize>() {
            Ok(n) => n,
            _ => exit("expect first argument to be a number"),
        },
        None => exit(ERROR),
    };

    let path = match args.next() {
        None => exit(ERROR),
        Some(path) => path,
    };

    let file = open(&path);

    let (line, pos) = file.chars().take(pos).fold((0, 0), |(line, pos), c| {
        if c == '\n' {
            (line + 1, 0)
        } else {
            (line, pos + 1)
        }
    });

    println!("line: {}\nposition: {}", line, pos);
}

fn open(path: &str) -> String {
    let content = match std::fs::read(path) {
        Ok(file) => file,
        Err(_) => exit("file not found"),
    };
    match String::from_utf8(content) {
        Ok(content) => content,
        Err(_) => exit("expected file to be valid utf-8"),
    }
}
