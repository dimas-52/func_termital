use crossterm::terminal;
use std::{i32, process::Command};

fn main() {
    let (mut cols_u, mut rows_u) = terminal::size().unwrap();
    println!("Ширина: {} символів", cols_u);
    println!("Висота: {} рядків", rows_u);

    // Command::new("clear").status().unwrap();

    let mut cols: i32 = cols_u as i32;
    let mut rows: i32 = rows_u as i32;

    cols = 160;
    rows = 80;

    //  Перевірка на парне число
    if cols % 2 == 0 {
        cols = cols - 1;
    };

    if rows % 2 == 0 {
        rows = rows - 1;
    };

    let cols_center = (cols / 2) as usize;
    let rows_center = (rows / 2) as usize;
    // println!("CENTER cols {} rows {}", cols_center, rows_center);

    //  Створення масива
    let mut matrix: Vec<Vec<&str>> = Vec::new();
    for i in 0..rows {
        if i as usize == rows_center {
            matrix.push(vec!["-"; cols as usize]);
        } else {
            matrix.push(vec![" "; cols as usize]);
        }
    }

    // println!("{:?}", matrix);

    //  Малювання
    for i in 0..rows {
        matrix[i as usize][cols_center] = "|";
    }
    matrix[(rows - 1) as usize][cols_center] = "▲";
    matrix[(rows - 1) as usize][cols_center - 1] = "Y";

    matrix[rows_center as usize][(cols - 1) as usize] = ">";
    matrix[(rows_center - 1) as usize][(cols - 1) as usize] = "X";

    let cols_min: i32 = (cols / 2) - cols;
    let cols_pl: i32 = cols / 2;

    ///////////////
    //    let mut y = x.pow(3) / 80;
    //    let mut y = (x.pow(2) - 100 * x) / 100;
    ///////////////
    let mut x_counter: i32 = 0;
    for x in cols_min..cols_pl {
        if x == 0 {
            continue;
        }
        let mut y = ((20 - x).pow(2) / x) / 15;

        y = y + rows_center as i32;

        if (x_counter >= 0) && (x_counter < cols) && (y >= 0) && (y < rows) {
            matrix[y as usize][x_counter as usize] = "/";
            // println!("x: {} y: {}", x_counter, y);
        }

        x_counter = x_counter + 1;
    }

    // print
    let print_xy = cols_center - 5;
    for _ in 0..print_xy {
        print!(" ");
    }
    print!("Y: {}", rows);
    for _ in 0..print_xy {
        print!(" ");
    }
    println!("X: {}", cols);

    for rows in matrix.iter().rev() {
        println!("{}", rows.join(""));
    }
}
