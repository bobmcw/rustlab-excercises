use std::io;
fn main() {
    let x = take_num(String::from("podaj pierwszą liczbę"));
    let y = take_num(String::from("podaj drugą liczbę"));
    let sign = take_sign();
    println!("{} {} {}",x,sign,y);
    match sign {
        '+' => println!("{}",dodawanie(x, y)),
        '-' => println!("{}",odejmowanie(x, y)),
        '*' => println!("{}",mnozenie(x, y)),
        '/' => println!("{}",dzielenie(x, y)),
        _ => println!("Niepoprawna operacja"),

    }
}
fn take_num(msg: String) -> i32{
    let mut input = String::new();
    println!("{}",&msg);
    io::stdin().read_line(&mut input).expect("podano nieprawidłową daną");
    let x: i32 = input.trim().parse().expect("not a number");
    x
}
fn take_sign() -> char{
    let mut input = String::new();
    println!("Wybierz operację (+,-,*,/)");
    io::stdin().read_line(&mut input).expect("nie udało się pobrać znaku");
    let x: char = input.trim().parse().expect("not a char");
    x
}
fn dodawanie(x:i32,y:i32)-> i32{
    x+y
}
fn odejmowanie(x:i32,y:i32)-> i32{
    x-y
}
fn mnozenie(x:i32,y:i32)-> i32{
    x*y
}
fn dzielenie(x:i32,y:i32)-> i32{
    if y == 0{
        panic!("nie mozna dzielic przez zero");
    }
    else {
        x/y
    }
}
