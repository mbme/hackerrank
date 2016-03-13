use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn must_parse_arr<T> (s: String) -> Vec<T> where T: std::str::FromStr {
    s.split_whitespace().map(|item| must_parse(item.to_string())).collect()
}

fn main() {
    for _ in 0..must_parse(readline()) {
        let params: Vec<u32> = must_parse_arr(readline());
        let total_money = params[0];
        let price = params[1];
        let pack_size = params[2];

        let mut chocolates = total_money / price;

        let mut wrappers = chocolates;
        loop {
            let chocos_from_pack = wrappers / pack_size;
            wrappers = wrappers % pack_size;

            if chocos_from_pack == 0 {
                break;
            }

            wrappers += chocos_from_pack;
            chocolates += chocos_from_pack;
        }

        println!("{}", chocolates);
    }
}
