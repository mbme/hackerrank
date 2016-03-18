use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn main() {
    let text = readline();
    let len = text.len();
    let l = (len as f32).sqrt();
    let mut rows = l.floor() as usize;
    let cols = l.ceil() as usize;
    if rows * cols < len {
        rows += 1;
    }
    assert!(rows * cols >= len, ":(");

    for i in 0..cols {
        for j in 0..rows {
            match text.chars().nth(i + j*cols) {
                Some(c) => print!("{}", c),
                None => break,
            }
        }
        if i < cols - 1 {
            print!(" ");
        }
    }
}
