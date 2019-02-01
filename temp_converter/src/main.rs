use std::io;

fn main() {
    let mut fahrenheit = String::new();

    println!("Coloque o valor da temperatura: ");
    io::stdin().read_line(&mut fahrenheit)
        .expect("Error Input!");

    let fahrenheit: f64 = fahrenheit.trim().parse::<f64>()
        .expect("Error parsing!");

    println!("{}", fahrenheit);

    let celsius = {
        ((fahrenheit - 32_f64) * 5_f64)/9_f64
    }.round();

    println!("The temperature in Celsius is: {}", celsius);
}
