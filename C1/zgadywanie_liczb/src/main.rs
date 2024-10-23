use rand::Rng;
use std::io;
fn main() {
   let mut rng = rand::thread_rng();
   let random_num = rng.gen_range(0..=100);
   loop{
    let mut input = String::new();
    println!("Podaj liczbę");
    io::stdin().read_line(&mut input).expect("reading failed");
    let user_input: i32 = input.trim().parse().expect("invalid number");

    if user_input < random_num { println!("Za mało"); }
    else if user_input > random_num { println!("Za dużo"); }
    else if user_input == random_num { break; }
   }
   println!("Gratulacje! liczba to było {}",random_num);
}
