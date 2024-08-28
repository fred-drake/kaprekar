struct KaprekarNumbers {
    largest: i32,
    smallest: i32,
}

impl KaprekarNumbers {
    fn difference(&self) -> i32 {
        self.largest - self.smallest
    }
}

fn rearrange_number(mut num: i32, largest: bool) -> i32 {
    let mut digits: Vec<i32> = Vec::new();
    
    // Extract digits
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    
    // Sort digits based on the 'largest' parameter
    if largest {
        digits.sort_by(|a, b| b.cmp(a)); // Descending order for largest
    } else {
        digits.sort(); // Ascending order for smallest
    }
    
    // Construct the rearranged number
    let mut result = 0;
    for &digit in &digits {
        result = result * 10 + digit;
    }
    
    result
}

fn process(mut value: i32) -> Vec<KaprekarNumbers> {
    let mut kaprekar_vec: Vec<KaprekarNumbers> = Vec::new();

    loop {
        let largest = rearrange_number(value, true);
        let smallest = rearrange_number(value, false);
        let kaprekar = KaprekarNumbers { largest, smallest };

        if kaprekar_vec.iter().any(|k| k.difference() == kaprekar.difference()) || kaprekar_vec.len() == 200 {
            break;
        }
        kaprekar_vec.push(kaprekar);
        value = kaprekar_vec.last().unwrap().difference();
   }

    kaprekar_vec
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let input: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number");
            std::process::exit(1);
        }
    };

    let kaprekar_numbers = process(input);

    println!("Kaprekar's routine for {}:", input);
    for (index, kaprekar) in kaprekar_numbers.iter().enumerate() {
        println!(
            "Step {}: {} - {} = {}",
            index + 1,
            kaprekar.largest,
            kaprekar.smallest,
            kaprekar.difference()
        );
    }

    if kaprekar_numbers.is_empty() {
        println!("No Kaprekar numbers found.");
    } else {
        println!(
            "Kaprekar's constant reached after {} steps.",
            kaprekar_numbers.len()
        );
    }
}
