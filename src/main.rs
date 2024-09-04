struct KaprekarNumbers {
    largest: i32,
    smallest: i32,
}

impl KaprekarNumbers {
    fn difference(&self) -> i32 {
        self.largest - self.smallest
    }
}
// Rearranges the digits of a given number to form either the largest or smallest possible number
// Parameters:
//   - num: The input number to be rearranged
//   - largest: A boolean flag indicating whether to form the largest (true) or smallest (false) number
// Returns:
//   The rearranged number as an i32
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

// Processes the input value using Kaprekar's routine
// Parameters:
//   - value: The initial number to start the routine
// Returns:
//   A vector of KaprekarNumbers structs representing each step of the routine
fn process(mut value: i32) -> Vec<KaprekarNumbers> {
    let mut kaprekar_vec: Vec<KaprekarNumbers> = Vec::new();

    loop {
        let largest = rearrange_number(value, true);
        let smallest = rearrange_number(value, false);
        let kaprekar = KaprekarNumbers { largest, smallest };

        if kaprekar_vec
            .iter()
            .any(|k| k.difference() == kaprekar.difference())
            || kaprekar_vec.len() == 200
        {
            break;
        }
        kaprekar_vec.push(kaprekar);
        value = kaprekar_vec.last().unwrap().difference();
    }

    kaprekar_vec
}

// Main function: Entry point of the program
// Handles command-line arguments, processes the input number,
// and displays the results of Kaprekar's routine
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let input: i32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: Invalid number");
        std::process::exit(1);
    });

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
