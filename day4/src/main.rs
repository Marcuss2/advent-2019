
fn criteria(number : i32) -> bool {
    let digits : Vec<u32> = {
        number.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
    };
    let mut double_encountered = false;
    let mut sequence_number = digits[0];
    let mut sequence = 1;
    for i in 1..digits.len() {
        if digits[i - 1] > digits[i] {
            return false;
        }
        if sequence_number == digits[i] {
            sequence += 1;
        } else if sequence == 2 {
            double_encountered = true;
        } else {
            sequence_number = digits[i];
            sequence = 1;
        }
    }
    if sequence == 2 {
        double_encountered = true;
    }
    double_encountered
}

fn passwords(lower_bound : i32, upper_bound : i32) -> i32 {
    let mut options = 0;
    for i in lower_bound..upper_bound {
        if criteria(i) {
            options += 1;
        }
    }
    options
}

fn main() {
    println!("{}", passwords(273025, 767253));
}
