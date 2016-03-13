use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn convert (time12: String) -> String {
    // remove AM/PM
    let mut time24 = time12.trim_right_matches(char::is_alphabetic).to_string();
    let mut hour: u8 = must_parse(time12[..2].to_string());

    if time12.ends_with("PM") {
        if hour != 12 {
            hour += 12;
        }
        time24 = format!("{0:#02}{1}", hour, &time24[2..]);
    } else { // AM
        if hour == 12 {
            hour = 0;
        }
        time24 = format!("{0:#02}{1}", hour, &time24[2..]);
    }

    time24
}

fn main() {
    let time12 = readline();

    println!("{}", convert(time12));
}

#[test]
fn test_am () {
    assert_eq!("12:00:00", convert("12:00:00PM".to_string()));
    assert_eq!("00:00:00", convert("12:00:00AM".to_string()));

    assert_eq!("13:24:33", convert("01:24:33PM".to_string()));
    assert_eq!("14:00:00", convert("02:00:00PM".to_string()));

    assert_eq!("01:24:33", convert("01:24:33AM".to_string()));
    assert_eq!("02:00:00", convert("02:00:00AM".to_string()));
}

