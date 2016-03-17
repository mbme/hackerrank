use std::io;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: String) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

type DigitV = Vec<u8>;

fn to_digitv (n: u32) -> DigitV {
    n.to_string().chars().map(|c| c as u8 - 48).collect()
}

fn multiply (n1: &DigitV, n2: &DigitV) -> DigitV  {
    if n1.len() < n2.len() {
        return multiply(n2, n1);
    }

    let mut longer = n1.clone();
    let mut shorter = n2.clone();

    longer.reverse();
    shorter.reverse();

    let mut result = vec![0; longer.len() + shorter.len()];

    for (j, b) in shorter.iter().enumerate() {
        let mut carry = 0;

        for (i, a) in longer.iter().enumerate() {
            let product = result[i + j] + a * b + carry;
            carry = product / 10;

            result[i + j] = product % 10;
        }

        result[j + longer.len()] += carry;
    }

    // remove trailing zeros
    while result.len() > 1 && *result.last().unwrap() == 0 {
        result.pop();
    }

    result.reverse();

    result
}

fn gen_test (a: u32, b: u32) {
    let res = multiply(&to_digitv(a), &to_digitv(b));
    assert_eq!((a * b).to_string(), digitv_to_string(&res));
}

#[test]
fn test_multiply () {
    gen_test(15, 10);
    gen_test(10, 10);
    gen_test(10, 15);
}


fn fact (n: u8) -> DigitV {
    let mut total = to_digitv(1);

    let mut current = n;
    while current > 0 {
        total = multiply(&total, &to_digitv(current as u32));
        current -= 1;
    }

    total
}

fn digitv_to_string (n: &DigitV) -> String {
    n.iter().map(|d| d.to_string()).collect::<String>()
}

fn main() {
    let n: u8 = must_parse(readline());
    println!("{}", digitv_to_string(&fact(n)));
}
