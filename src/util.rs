use std::io;

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
