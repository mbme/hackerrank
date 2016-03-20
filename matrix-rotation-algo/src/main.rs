use std::io;
use std::cmp::min;

type Matrix = Vec<Vec<u64>>;

fn readline () -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).ok().expect("read error");

    input_str
}

fn read_matrix (n: usize) -> Matrix {
    (0..n).map(|_| must_parse_arr(&readline())).collect()
}

fn must_parse<T> (num_str: &str) -> T where T: std::str::FromStr {
    num_str.trim().parse().ok().expect("parse error")
}

fn must_parse_arr<T> (s: &str) -> Vec<T> where T: std::str::FromStr {
    s.split_whitespace().map(|item| must_parse(item)).collect()
}

fn num_arr_to_string (n: &Vec<u64>) -> String {
    n.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" ")
}

struct CircleIter {
    counter: usize,

    circle_size: usize,
    circle_pos: usize,
    circle_rows: usize,
    circle_cols: usize,

    first_row: usize,
    last_row: usize,

    first_col: usize,
    last_col: usize,
}

type Pos = (usize, usize, usize);

impl Iterator for CircleIter {
    type Item = Pos;
    fn next (&mut self) -> Option<Pos> {
        if self.counter == self.circle_size {
            return None;
        }

        let pos = if self.counter < self.circle_rows - 1 {

            (self.circle_pos + self.counter, self.first_col, self.counter)

        } else if self.counter < self.circle_rows - 1 + self.circle_cols - 1 {

            (self.last_row, self.first_col + self.counter + 1 - self.circle_rows, self.counter)

        } else if self.counter < 2 * self.circle_rows - 2 + self.circle_cols - 1 {

            let temp = self.counter + 2 - self.circle_rows - self.circle_cols;

            (self.last_row - temp, self.last_col, self.counter)

        } else {

            let temp = self.counter + 3 - 2 * self.circle_rows - self.circle_cols;
            (self.first_row, self.last_col - temp, self.counter)

        };

        self.counter += 1;

        Some(pos)
    }
}

fn iter_circle (rows: usize, cols: usize, circle_pos: usize) -> CircleIter {
    CircleIter {
        counter: 0,

        circle_size: calc_circle_size(rows, cols, circle_pos),
        circle_pos: circle_pos,
        circle_rows: rows - 2 * circle_pos,
        circle_cols: cols - 2 * circle_pos,

        first_row: circle_pos,
        first_col: circle_pos,
        last_col: cols - circle_pos - 1,
        last_row: rows - circle_pos - 1,
    }
}

fn calc_circle_size (rows: usize, cols: usize, circle_pos: usize) -> usize {
    2 * rows - 4 * circle_pos + 2 * cols - 4 * circle_pos - 4
}

fn read_circle(m: &Matrix, rows: usize, cols: usize, circle_pos: usize) -> Vec<u64> {
    let mut circle: Vec<u64> = Vec::with_capacity(calc_circle_size(rows, cols, circle_pos));

    for (row, col, _) in iter_circle(rows, cols, circle_pos) {
        circle.push(m[row][col]);
    }

    circle
}

fn main() {
    let params: Vec<u64> = must_parse_arr(&readline());
    let rows = params[0] as usize;
    let cols = params[1] as usize;
    let rotations = params[2];

    let matrix = &read_matrix(rows);

    let mut result: Matrix = (0..rows).map(|_| vec![0; cols]).collect();

    for circle_pos in 0..(min(rows, cols) / 2) {
        let circle = read_circle(matrix, rows, cols, circle_pos);
        let r = (rotations % circle.len() as u64) as usize;

        for (row, col, pos) in iter_circle(rows, cols, circle_pos) {
            let val_pos = if pos < r { pos + circle.len() - r } else { pos - r};

            result[row][col] = circle[val_pos];
        }
    }

    for vec in result {
        println!("{}", num_arr_to_string(&vec));
    }
}
