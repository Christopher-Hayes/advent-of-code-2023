use regex::Regex;

pub fn a() {
    let output = crate::halp::read_file("day1.txt");
    let lines = output.split("\n");
    let mut sum: i64 = 0;

    for line in lines {
        let re = Regex::new(r"\d").unwrap();
        let numbers: Vec<&str> = re.find_iter(&line).map(|mat| mat.as_str()).collect();

        if numbers.is_empty() {
            continue;
        }

        let first_number = numbers[0];
        let last_number = numbers[numbers.len() - 1];

        let num: i64 = match format!("{}{}", first_number, last_number).parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to parse number");
                continue;
            }
        };

        sum = match sum.checked_add(num) {
            Some(value) => value,
            None => {
                println!("Overflow error");
                break;
            }
        };
    }

    println!("The code is {}", sum);
}

pub fn b() {
    println!("TODO");
}
