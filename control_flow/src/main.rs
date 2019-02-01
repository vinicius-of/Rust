fn main() {
    // If statement
    /*
        let number = 3;

        if number > 4 {
            println!("The number is greater than 4");
        } else {
            println!("The number is lesser than 4");
        }

        let condition = true;
        let input = if number > 2 {
            number
        } else {
            2
        };

        //OBS: A let statement can only receive one data type.

        println!("The value of input is: {}", input);
    */

    // Loop
    /*
        // loop

        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 3;
            }
        };

        // Compare two expressions are equal to each other
        // On panic, this macro will print the values of the expressions with their debug
        assert_eq!(result, 30);
    */

    // While
    /*
        let mut i = 0;

        while i <= 10 {
            println!("{}!", i);
            i = i + 1;
        }
    */

    // For
    /*
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("The value is: {}", element);
        }

        for n in (0..5).rev() {
            println!("{}!", n);
        }
    */
}
