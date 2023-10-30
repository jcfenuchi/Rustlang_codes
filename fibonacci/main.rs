use std::io;

fn main() {
    println!("Digite nth Fibonacci number!");
    let mut pos = String::new();
    io::stdin()
        .read_line(&mut pos)
        .expect("Failed to read line");
    let pos: i32 = pos.trim().parse().expect("Error to convert");
    get_fibonacci(pos);
}

fn get_fibonacci(num: i32) {
    let mut pos = 2;
    let mut number1: u128 = 0;
    let mut number2: u128 = 1;
    let mut aux: u128 = 0;
    if num == 0 {
        println!("can't get 0th position of Fibonacci.")
    } else if num == 1 {
        println!("pos:1th => {number1}");
    } else if num > 0 {
    println!("pos:1th => {number1}");
    println!("pos:2th => {number2}");
    for _ in 2..num {
        aux = number1;
        number1 = number2;
        number2 = aux+number2;
        pos += 1;
        println!("pos:{}th => {}",pos,number2);
        }
    }
}