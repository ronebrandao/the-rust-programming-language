//consts variables need explicit type declaration
const THIS_IS_A_CONST: i32 = 7;
const THIS_IS_ANOTHER_CONST: f32 = 2.43;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is now: {x}");

    //immutable variables can have their value replaced
    //by explicitly adding let again on variable declation
    let test = "sth";

    {
        let test = "sth else";
        println!("Test is now: {test}");
    }

    println!("Different value on outer scope: {test}");
}
