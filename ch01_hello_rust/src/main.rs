fn fibonacci(n: u64) -> u64 {
    let mut lookup: [u64; 2] = [0, 1];

    let n = n as usize;
    if n < 2 {
        return lookup[n];
    }

    let mut index = 2;
    while index < n {
        let temp = lookup[0] + lookup[1];
        lookup[0] = lookup[1];
        lookup[1] = temp;
        index += 1;
    }

    return lookup[0] + lookup[1];
}

const TEMPERATURE_CONVERSION_FACTOR: f32 = 1.8;
const TEMPERATURE_CONVERSION_CONSTANT: f32 = 32.0;

fn to_fahrenheit(celsius: f32) -> f32 {
    (celsius * TEMPERATURE_CONVERSION_FACTOR) + TEMPERATURE_CONVERSION_CONSTANT
}

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - TEMPERATURE_CONVERSION_CONSTANT) / TEMPERATURE_CONVERSION_FACTOR
}

fn main() {
    println!("Rust basic concepts");
    println!("======");

    for n in 0..9 {
        println!("Fibonacci N={} is {}", n, fibonacci(n));
    }

    println!("======");

    println!("37ºC  is {}ºF", to_fahrenheit(37.0));
    println!("212ºF is {}ºC", to_celsius(212.0));
}
