fn main() {
    // println!("Hello, world!");
    // another_function(5, 6);
    // statement();
    let x = return_function(1, 2);
    println!("The result is: {}", x);
}

// Necessary type annotation
// fn another_function(x: i32, y: i32){
//     println!("The value of x is: {}", x);
//     println!("The value of x is: {}", y);
// }

/*
    Statement: Statements are instructions that perform some
    action and do not return a value.

    Expressions: Evaluate to a resulting value.
*/

// Statemnet Example

// fn statement(){
//     let y = 6;
//     println!("{}", y);

//     //let x = (let y = 6); Error: Do not return a value.

//     let x = 5;
//     let y = {
//         let x = 3;
//         x + 1 // It is a expression, so it doesn't need semi colon, unlikely
//         // almost every that we have seen so far.
//     };
// }


// A function that return the sum of two numbers.
fn return_function(x: i32, y: i32) -> i32{x + y}