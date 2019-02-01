use std::io; // std: Standard Library // io: Input/Output
use rand::Rng;
use std::cmp::Ordering; // Enum

fn main() {
    let number = rand::thread_rng().gen_range(1,101);
    println!("Guess the number!");

    loop {
        // Infinite loop

        println!("Please guess a number.");

        /*
            let - Create a variable immutable
            mut - Turn the variable into mutable
        */

        let mut guess = String::new();
        // Associated function/Static method
        // An associated function is implemented on a type.

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        /*
            stdin: Returns an instance of std::io::Stdin, which is a type that
            represents a handle to the standard input for your terminal.

            read_line: A method to handle the input from user

            &: Indicates that this argument is a reference, it's immutable by default
        */

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
                
            /*
                This is how you handle a error without crashing the program
                using match instead .expect().

                Result is a enum type that contains the variants 'Ok' 
                and 'Err'. match will compare the input value with the arm's
                pattern

                The underscore value "(_)" is a catch-all value.

                continue: Continues to the next iteration of loop
            */
        };

        /*
            "Shadowing" the previous value for a new one, casting the variable 'guess' to 
            u32 (32 bits number).
            trim(): Remove any whitespace at the beginning and end.
            parse(): Method on strings, parses a string into some kind of number. We need to 
            tell the Rust what type we want to parse.
            OBS: When a user inputs any value and press "enter", it adds a '/n' on the string.
        */

        println!("You've guessed: {}", guess);
        /*
            {}: Placeholder to the variable
        */

        match guess.cmp(&number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!"); 
                break;
            }
                /*
                    When the guessed number is correct, the game will stop from the infinite loop
                */
        }

        /*
            match: A expression is made up of arms. An arm consists of a pattern and the code
            that should be run if the value given to the beginning of the match expression fits
            that arm's pattern.
        */
    }
}
