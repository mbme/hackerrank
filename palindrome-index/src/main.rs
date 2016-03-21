use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    let newlines: &[_] = &['\n', '\r'];
    input_str.trim_right_matches(newlines).to_string()
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn is_pal (chars: &Vec<char>) -> bool {
    for i in 0..chars.len()/2 {
        if chars[i] != chars[chars.len() - 1 - i] {
            return false;
        }
    }

    return true;
}

fn main() {
    for _ in 0..must_parse(&readline()) {
        let chars = &readline().chars().collect::<Vec<char>>();

        if is_pal(chars) {
            println!("-1");
            continue;
        }

        for i in 0..chars.len() / 2 {
            if chars[i] == chars[chars.len() - 1 - i] {
                continue;
            }

            let mut chars = chars.clone();
            chars.remove(i);

            println!("{}", if is_pal(&chars) { i } else { chars.len() - i });
            break;
        }
    }
}
