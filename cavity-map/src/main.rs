use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str.trim().to_string()
}

fn read_digits_matrix (n: usize) -> Vec<Vec<char>> {
    (0..n).map(|_| readline().chars().collect::<Vec<char>>()).collect()
}

fn print_digits_line (line: &Vec<char>) {
    let line: String = line.iter().cloned().collect();
    println!("{}", line);
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn main() {
    let n: usize = must_parse(readline());

    let m = read_digits_matrix(n);
    print_digits_line(&m[0]);
    if n == 1 {
        return;
    }
    for i in 1..n-1 {
        let prev_line = &m[i-1];
        let line = &m[i];
        let next_line = &m[i+1];
        for (j, &d) in line.iter().enumerate() {
            if j == 0 {
                print!("{}", d);
                continue;
            }

            if j == n - 1 {
                println!("{}", d);
                continue;
            }

            if d > prev_line[j] && d > line[j-1] && d > line[j+1] && d > next_line[j] {
                print!("X");
            } else {
                print!("{}", d);
            }
        }
    }
    print_digits_line(&m[n-1]);
}
