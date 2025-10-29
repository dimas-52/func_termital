fn main() {
    // базовий код Брайля в Unicode
    let base: u32 = 0x2800;

    /*  ,___,
        |1 4|
        |2 5|
        |3 6|
        |7 8|
        `````
    (0x01, 0x08)
    (0x02, 0x10)
    (0x04, 0x20)
    (0x40, 0x80)
    */

    // вибираємо, які точки підняті
    // наприклад: точки 1 і 3 (0x01 + 0x04)

    for _ in bar {}

    let mask: u32 = 0x01 + 0x04;

    // формуємо код символу
    let code = base + mask;

    // перетворюємо код у символ
    let ch = std::char::from_u32(code).unwrap();

    println!("Точки: 1 і 3");
    println!("Маска: 0x{:02X}", mask);
    println!("Unicode: U+{:04X}", code);
    println!("Символ: {}", ch);
}
