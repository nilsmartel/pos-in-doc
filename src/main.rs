const ERROR: &'static str = "require 2 arguments <pos> <file>";

fn main() {
    let mut args = std::env::args().skip(1);
    let pos = args
        .next()
        .expect(ERROR)
        .parse::<usize>()
        .expect("expect first argument to be a number");
    let path = args.next().expect(ERROR);
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
    let content = std::fs::read(path).expect("file not found");
    String::from_utf8(content).expect("file is not valid utf-8. Expect file to be be valid utf-8")
}
