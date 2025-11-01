use std::i32;

fn main() {
    // базовий код Брайля в Unicode
    let base: u32 = 0x2800;
    let base_char = std::char::from_u32(base).unwrap();

    /*  ,___,
     *        |1 4|
     *        |2 5|
     *        |3 6|
     *        |7 8|
     *        `````
     *    (0x01, 0x08)
     *    (0x02, 0x10)
     *    (0x04, 0x20)
     *    (0x40, 0x80)
     */

    let mut cols = 160;
    let mut rows = 80;

    // cols = cols * 2;
    // rows = rows * 4;

    //  Перевірка на парне число
    if cols % 2 == 0 {
        cols = cols - 1;
    };

    if rows % 2 == 0 {
        rows = rows - 1;
    };

    let cols_center = (cols / 2) as usize;
    let rows_center = (rows / 2) as usize;

    //  Створення масива
    let mut mask: u32 = 0x02 + 0x04 + 0x10 + 0x20;
    let code = base + mask;
    let ch = std::char::from_u32(code).unwrap();
    println!("{}", ch);

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for i in 0..rows {
        if i as usize == rows_center {
            matrix.push(vec![ch; cols as usize]);
        } else {
            matrix.push(vec![base_char; cols as usize]);
        }
    }

    //  Малювання
    for i in 0..rows {
        matrix[i as usize][cols_center] = '|';
    }
    matrix[(rows - 1) as usize][cols_center] = '▲';
    matrix[(rows - 1) as usize][cols_center - 1] = 'Y';

    matrix[rows_center as usize][(cols - 1) as usize] = '>';
    matrix[(rows_center - 1) as usize][(cols - 1) as usize] = 'X';

    // ОБЧИСЛЕННЯ
    let cols_min: f64 = (cols as f64 / 2.0) - cols as f64;
    let cols_pl: f64 = cols as f64 / 2.0;

    let mut x: f64 = cols_min as f64;

    let mut mask05: u32 = 0x0;
    let mut mask00: u32 = 0x0;

    while x < cols as f64 {
        mask = 0x0;
        mask00 = 0x0;
        mask05 = 0x0;

        if x == 0.0 {
            x = x + 0.5;
            continue; // Переходимо до наступної ітерації
        }
        //////////////
        let y: f64 = x / 4.0;
        //////////////
        let y_fract: f64 = y.fract().abs();
        let x_fract: f64 = x.fract().abs();

        if x_fract == 0.5 {
            if 0.0 <= y_fract && y_fract <= 0.25 {
                mask00 = 0x01;
            } else if 0.25 < y_fract && y_fract <= 0.50 {
                mask00 = 0x02;
            } else if 0.50 < y_fract && y_fract <= 0.75 {
                mask00 = 0x04;
            } else if 0.75 < y_fract && y_fract < 1.0 {
                mask00 = 0x40;
            } else {
                panic!(
                    "Сталася помилка: перевірка y_fract не була правильна. y_fract = {}",
                    y_fract
                );
            }
        } else if x_fract == 0.0 {
            if 0.0 <= y_fract && y_fract <= 0.25 {
                mask05 = 0x08;
            } else if 0.25 < y_fract && y_fract <= 0.50 {
                mask05 = 0x10;
            } else if 0.50 < y_fract && y_fract <= 0.75 {
                mask05 = 0x20;
            } else if 0.75 < y_fract && y_fract < 1.0 {
                mask05 = 0x80;
            } else {
                panic!(
                    "Сталася помилка: перевірка y_fract не була правильна. y_fract = {}",
                    y_fract
                );
            }
        } else {
            panic!("Сталася помилка: x не 0.0 та не 0.5. x_fract = {}", x_fract);
        }

        mask = mask00 + mask05;

        let matrix_col: usize = (x / 1.0 + cols_center as f64) as usize;
        let matrix_row: usize = (y / 2.0 + rows_center as f64) as usize;

        //let matrix_col = (x + cols_center as f64).round() as usize;
        //let matrix_row = (y + rows_center as f64).round() as usize;

        let code = base + mask;
        let ch = std::char::from_u32(code).unwrap();

        if matrix_row < rows && matrix_col < cols {
            matrix[matrix_row][matrix_col] = ch;
        }

        println!(
            "x: {x}   y: {y}   mask: {mask}   ch: [{ch}]   |   matrix_col: {matrix_col}   matrix_row {matrix_row}"
        );

        x = x + 0.5;
    }

    let print_xy = cols_center - 5;
    for _ in 0..print_xy {
        print!(" ");
    }
    print!("Y: {}", rows);
    for _ in 0..print_xy {
        print!(" ");
    }
    println!("X: {}", cols);

    for row in matrix.iter().rev() {
        let s: String = row.iter().collect();
        println!("{}", s);
    }
}

// fn brille() {}
