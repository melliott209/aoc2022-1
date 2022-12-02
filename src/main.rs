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

    //let max_calories = get_max(&totals);
    //println!("Elf with the most calories has {} kcals in snacks.", max_calories);

    let (first, second, third) = get_top_three(&mut totals);
    let total_kcals = first + second + third;

    println!("#1 elf: {} kcals", first);
    println!("#2 elf: {} kcals", second);
    println!("#3 elf: {} kcals", third);
    println!("Total kcals for top three: {}", total_kcals);
}

// Get the largest element out of a vector of integers
fn get_max(v: &Vec<usize>) -> usize {
    *v.iter().max().unwrap()
}

// Get the top three largest elements out of a vector of integers,
// as a tuple.
fn get_top_three(v: &mut Vec<usize>) -> (usize, usize, usize) {
    v.sort();
    v.reverse();
    (v[0], v[1], v[2])
}
