use std::io ;
fn main() {
    loop {
    let mut choice = String::new();
    println!("escolha:\n[1] °F  to °C\n[2] °C to °F\n[3] Sair.");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: f32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if choice == 1.0 || choice == 2.0 || choice == 3.0 {
        if choice == 1.0 {
            let mut temperatura = String::new();
            println!("Digite sua temperatura em °F:");
            io::stdin()
                .read_line(&mut temperatura)
                .expect("Failed to read line");
            let temperatura: f32 = temperatura.trim().parse().expect("Not a number!");
            println!("{temperatura}°F é equivalente a {}°C", fahrenheit_to_celsius(temperatura)); 
        } else if choice == 2.0 {
            let mut temperatura = String::new();
            println!("Digite sua temperatura em °C:");
            io::stdin()
                .read_line(&mut temperatura)
                .expect("Failed to read line");
            let temperatura: f32 = temperatura.trim().parse().expect("Not a number!");
            println!("{temperatura}°C é equivalente a {}°F", celsius_to_fahrenheit(temperatura)); 
        } else if choice == 3.0 {
            break;
        }
    };
    }; 
}   

fn fahrenheit_to_celsius(temp: f32) -> f32{
    ( temp - 32.0 ) / 1.8
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    temp * 1.8 + 32.0
}