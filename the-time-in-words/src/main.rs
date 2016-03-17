use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn hour_to_string (hour: u8) -> String {
    (match hour {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        _ => "something else",
    }).to_string()
}

fn minutes_to_string (minutes: u8) -> String {
    if minutes > 20 && minutes < 30 {
        "twenty ".to_string() + &minutes_to_string(minutes - 20)
    } else {
        (match minutes {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            20 => "twenty",
            _ => "something else",
        }).to_string()
    }
}


fn main() {
    let hour: u8 = must_parse(readline());
    let minutes: u8 = must_parse(readline());
    let h_str = hour_to_string(hour);
    let h_str_next = hour_to_string(hour + 1);
    let m_str = minutes_to_string(minutes);
    let m_str_rev = minutes_to_string(60 - minutes);
    println!("{}", match minutes {
        0 => h_str + " o' clock",
        15 => format!("quarter past {}", h_str),
        30 => format!("half past {}", h_str),
        45 => format!("quarter to {}", h_str_next),
        1 => format!("one minute past {}", h_str),
        m if m < 30 => format!("{} minutes past {}", m_str, h_str),
        59 => format!("one minute to {}", h_str_next),
        _ => format!("{} minutes to {}", m_str_rev, h_str_next)
    });
}
