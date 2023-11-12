fn main() {
    let x = 5;
    let mut y:u8 = 10;
    let bool_int = false;
    let omega = '\u{03A9}';
    let tup = ('a', 32, true);

    println!("x:{}, y:{}", x, y);
    y = 255;
    println!{"x:{}, y:{}, bool:{}, omega:{}", x, y, bool_int, omega};
    println!{"second element: {}", tup.1};
}
