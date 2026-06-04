use std::thread::current;

fn main() {
    const MAX_TEMP_C: f64 = 100.0;
    let mut current_temp = 20.0;
    current_temp += 15.5;
    current_temp += 10.0;

    let number = "42";
    let number: i32 = number.parse().unwrap();
}