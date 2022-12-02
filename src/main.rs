use std::{fs::File, io::{BufReader, BufRead}};

fn main() {

    let mut totals: Vec<usize> = Vec::new();
    let mut count: usize = 0;

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        match line.unwrap().parse::<usize>() {
            Ok(num) => count += num,
            Err(_) => {
                totals.push(count);
                count = 0;
            }
        }
    }

    let max_calories = totals.iter().max().unwrap();
    println!("Max calorie value is: {}", max_calories);

    totals.sort();
    let l = totals.len()-1;
    let top_three = (totals[l], totals[l-1], totals[l-2]);
    println!("1st calorie elf: {} kcals", top_three.0);
    println!("2nd calorie elf: {} kcals", top_three.1);
    println!("3rd calorie elf: {} kcals", top_three.2);
    println!("Total kcals for top three: {}", top_three.0+top_three.1+top_three.2);
}
