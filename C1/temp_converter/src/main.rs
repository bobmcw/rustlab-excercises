use std::io;


fn main() {
    println!("Podaj temperaturę:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("nie udało się wczytać danych");
    let value: f32 = input.trim().parse().expect("niepoprawna dana");
    println!("Konwertować na (F)ahrenheita czy na (C)elcjusza?");
    input = String::new();
    io::stdin().read_line(&mut input).expect("nie udało się wczytać danych");
    let target_temp: char = input.trim().parse().expect("Podano niepoprawny znak");
    match target_temp {
        'C' => println!("Wynik: {} C",(value-32.0) / 1.8),
        'F' => println!("Wynik: {} F",(value * 1.8 + 32.0)),
        _ => println!("Podano niepoprawny znak. Prawidłowe znaki to C lub F"),
    }
}
