const MINUTES_IN_THREE_HOURS: u32 = 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    println!("There are {MINUTES_IN_THREE_HOURS} minutes in 3 hours");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    print!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("You want {spaces} spaces")
}
