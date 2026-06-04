fn main() {
    let square_num = square(5);
    println!("Square of 5 is: {}", square_num);
    let celsius = fahrenheit_to_celsius(68.0);
    println!("68°F is {}°C", celsius);
    let is_even = is_even(5);
    println!("5 is even: {}", is_even);
    describe_number(5);

    let nums = [1, 2, 3, 4, 5];
    let (sum, min, max) = stats(&nums);
    println!("Sum: {}, Min: {}, Max: {}", sum, min, max);

    let square_then_add_one_result = square_then_add_one(5);
    println!("Square of 5 plus one is: {}", square_then_add_one_result);
}

fn square(n: i32) -> i32 {
    n * n
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn describe_number(n: i32) {
    if n < 0 {
        println!("It is negative.");
    } else if (n == 0) {
        println!("It is zero.");
    } else {
        println!("It is positive.");
    }
}

fn stats(nums: &[i32]) -> (i32, i32, i32) {
    let mut sum = 0;

    let mut max = i32::MIN;
    let mut min = i32::MAX;

    for &n in nums {
        sum += n;
        if n > max {
            max = n;
        }
        if n < min {
            min = n;
        }
    }
    (sum, min, max)
}

fn square_then_add_one(n: i32) -> i32 {
    let squared = {
        n * n
    };
    return squared + 1;
}