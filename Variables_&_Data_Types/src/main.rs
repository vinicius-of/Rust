fn main() {
    /*
        /*
        Variables
         -Immutable (use mut)
         */

        // let x = 5;
        // let mut y = 6;

        // y = 7;
        // println!("{}", y);
        // x = 8;
        // println!("{}", x);

        /*
            Constants
            - Only can be settled on a expression, but not the result of a function call.
            - There is a convencion to create a constant: All uppercase with undescores between words
        */

        // const MAX_POINTS: u32 = 100_000;
        // println!("{}", MAX_POINTS);

        /*
            Shadowing - Redefining the inmut(immutable) variable
            - You can cast efficiently just shadowing the casted variable
            and you can't do it if a mut variable, because its static type,
            when shadowing allow to recreate the variable and consequently 
            your type.
        */

        // let x = 5;
        // println!("{}", x);
        // let x = x + 1;
        // println!("{}", x);
        // let x = x * 2;
        // println!("{}", x);

        // let spaces = "   ";
        // let spaces = spaces.len();
        // println!("{}", spaces);

    */

    /*
        Data types
    */

    // let guess: u32 = "42".parse()
    //     .expect("Not a number");

    /*
        Scalar types: Represents a single value like:
            integers
            floating-points
            numbers
            booleans
            chars
    */

    //Integers
    /*
        (Never negative) u(8-128): unsigned intergers
                         i(8-128): signed intergers  
    */

    //Floating-points
    /*
        let x = 2.0; //f64
        let y: f32 = 3.0; //f32
    */

    //Numeric operations
    /* +, -, *, /, % */

    /*
        Compound Types: Can group multiple values into one type:
        Arrays
        Tuples
    */

        //Tuple Type: Differents data types into just one tuple
    /*
        let _tup: (i32, f64, u8) = (-500, 6.4, 1);

        //To destruct in individual values out of a tuple type
        let (x, y, z) = _tup;
        println!("X: {}\nY: {}\nZ: {}", x, y, z);

        //We can access any data using (.) after '_tup' and select the value by index
        println!("\n0: {}\n1: {}\n2: {}", _tup.0, _tup.1, _tup.2);
    */

    //Array Type: Same data type into just one Array
    /*
        let _a = [1, 2, 3, 4, 5, 6];

        let _b: [i32; 5] = [1, 2, 3, 4, 5];

        // Shadowing a array
        let _b: [i32; 6] = [1, 2, 3, 4, 5, 6];

        println!("{}", _a[3]);
    */
}
