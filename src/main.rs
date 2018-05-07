extern crate prime_number;
use prime_number::*;

fn main() {
    println!("Welcome in Prime Numbers Generator!");

    loop {
        println!("Please input the start number:");
        let start_num = input_as_num();

        println!("Please input the end number:");
        let end_num = input_as_num();

        let primes_group = Primes::new(start_num, end_num);
        let primes = primes_group.generate_primes();

        if primes.len() != 0 {
            let primes_str = vec_to_string(&primes);
            println!("Prime numbers in selected range are:\n{}", primes_str);

            println!("Do you want to save prime numbers to a file? (y/n)");
            if read_user_input() == "y" {
                let file_name = format!("prime_numbers_{}_{}", start_num, end_num);

                let file =
                    SimpleFile::new(file_name, "txt".to_string(), "prime_numbers".to_string());

                file.create_dir();
                file.create_file();
                file.write_file(&primes_str);
            }
        } else {
            println!("There aren't any prime numbers in selected range.");
        }
    }
}
