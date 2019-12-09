use std::io;
use std::io::Read;
use std::path::PathBuf;

pub fn read_input_file<'a>(fname: &'a str) -> Vec<u8> {
    let mut path = PathBuf::new();
    path.push("inputs");
    path.push(fname);

    let mut out = vec![];
    let mut fp = std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .expect("should open");

    fp.read_to_end(&mut out).expect("read should work");
    out
}

pub fn read_stdin_lines() -> Vec<String> {
    let mut line = String::new();

    let mut out = vec![];
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }

        line = line.trim().to_owned();

        out.push(line);
        line = String::new();
    }

    out
}

pub fn read_stdin_numbers() -> Vec<i32> {
    read_stdin_lines()
        .iter()
        .map(|line| line.parse::<i32>().expect("failed to parse line as number"))
        .collect()
}
