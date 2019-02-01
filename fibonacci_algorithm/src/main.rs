fn main() {

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for i in 0..6 {
        c = a + b;
        println!("{}, {}, {}", a, b, c);
        a = b + c;
        b = a + c;
    }
}
