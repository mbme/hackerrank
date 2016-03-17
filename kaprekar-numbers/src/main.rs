use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn is_kaprekar (n: u64) -> bool {
    let n2 = (n*n).to_string();

    let d = n2.len() / 2;

    let lstr = n2[0..d].to_string();
    let l: u32 = if lstr.is_empty() { 0 } else { must_parse(lstr) };
    let r: u32 = must_parse(n2[d..].to_string());

    l + r == n as u32
}

fn main() {
    let p: u32 = must_parse(readline());
    let q: u32 = must_parse(readline());

    let mut results = vec![];
    for n in p..q + 1 {
        if is_kaprekar(n as u64) {
            results.push(n.to_string());
        }
    }

    if results.len() > 0 {
        println!("{}", results.join(" "));
    } else {
        println!("INVALID RANGE");
    }
}
