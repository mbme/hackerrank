use std::io;
use std::collections::HashSet;

type CountrySet = HashSet<u32>;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn must_parse_arr<T> (s: &str) -> Vec<T> where T: std::str::FromStr {
    s.split_whitespace().map(|item| must_parse(item)).collect()
}

fn find_country (countries: &Vec<CountrySet>, p1: &u32, p2: &u32) -> (Option<usize>, Option<usize>) {
    let mut first = None;
    let mut second = None;
    for (i, country) in countries.iter().enumerate() {
        if country.contains(p1) || country.contains(p2) {
            if first == None {
                first = Some(i);
            } else if second == None {
                second = Some(i);
                break;
            }
        }
    }
    (first, second)
}

fn main() {
    let config: Vec<u32> = must_parse_arr(&readline());
    let n = config[0];

    let mut countries: Vec<CountrySet> = vec![];
    let mut standalone_countries = CountrySet::new();
    for i in 0..n {
        standalone_countries.insert(i);
    }

    for _ in 0..config[1] {
        let pair: Vec<u32> = must_parse_arr(&readline());
        let p1 = pair[0];
        let p2 = pair[1];

        standalone_countries.remove(&p1);
        standalone_countries.remove(&p2);

        match find_country(&countries, &p1, &p2) {
            (None, None) => {
                let mut country = CountrySet::new();
                country.insert(p1);
                country.insert(p2);
                countries.push(country);
            },
            (Some(pos1), None) => {
                let ref mut country = countries[pos1];
                country.insert(p1);
                country.insert(p2);
            },
            (Some(pos1), Some(pos2)) => {
                let same_country;
                {
                    same_country = countries.remove(pos2);
                }

                let ref mut country = countries[pos1];
                country.insert(p1);
                country.insert(p2);

                for v in same_country {
                    country.insert(v);
                }
            },
            (_, _) => {}
        }
    }

    let mut sizes: Vec<usize> = countries.iter().map(|c| c.len()).collect();
    for _ in 0..standalone_countries.len() {
        sizes.push(1);
    }

    let mut total = 0;
    for i in 0..sizes.len() {
        for j in i+1..sizes.len() {
            total += sizes[i] * sizes[j];
        }
    }

    println!("{}", total);
}
