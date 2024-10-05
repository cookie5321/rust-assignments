use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split(" ");
    let count: i32 = input.next().unwrap().trim().parse().unwrap();
    let expected: f32 = input.next().unwrap().trim().parse().unwrap();

    let mut sum = 0;
    for i in 0..count - 1 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split(" ");
        let credit: i32 = input.next().unwrap().trim().parse().unwrap();

        sum += credit * round(match input.next().unwrap().trim() {
            "A+" => 4.50_f32,
            "A0" => 4.00,
            "B+" => 3.50,
            "B0" => 3.00,
            "C+" => 2.50,
            "C0" => 2.00,
            "D+" => 1.50,
            "D0" => 1.00,
            "F" => 0.00,
            _ => 0.00
        }, 2) as i32;
    }

    println!("{}", sum)
}

// https://stackoverflow.com/a/72706990
fn round(x: f32, decimals: u32) -> f32 {
    let y = 10_i32.pow(decimals) as f32;
    (x * y).round() / y
}