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

type BitV = Vec<char>;

fn sum_bits (v1: &BitV, v2: &BitV) -> usize {
    let mut res: usize = 0;

    for (i, v) in v1.iter().enumerate() {
        if *v == '1' || v2[i] == '1' {
            res += 1;
        }
    }

    res
}

fn read_digits_matrix (n: usize) -> Vec<BitV> {
    (0..n).map(|_| readline().trim().chars().collect::<BitV>()).collect()
}

fn main() {
    let options: Vec<usize> = must_parse_arr(readline());
    let n = options[0];

    let items = read_digits_matrix(n);
    let mut max = 0;
    let mut maxc = 0;

    for i in 0..n-1 {
        let v1 = &items[i];
        for j in i..n {
            let v2 = &items[j];

            let sum = sum_bits(v1, v2);

            if sum > max {
                max = sum;
                maxc = 1;
            } else if sum == max {
                maxc += 1;
            }
        }
    }

    println!("{}", max);
    println!("{}", maxc);
}
