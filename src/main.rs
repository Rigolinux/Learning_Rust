#[allow(unused_variables, unused_mut, dead_code)]
fn main() {
    let (x, y) = (1, 2);
    let mut z = 5;

    
    
}

fn compare(x: i32, y: i32) -> bool {
    x == y
}

fn testVar() {
    let mut x = 5;
   
    println!("The value of x is: {}", x);
    println!("Hello i am from docker!");
    
    x = 6;
    println!("The value of x is: {}", x);

    const PI: f32 = 3.1416;
    println!("The value of PI is: {}", PI);

    x = x * PI as i32;

    println!("The value of x with pi is: {}", x);


    let  number = 3;

    println!("The value of number is: {}", number);

    let number: &str = "i have pen ";

    println!("The value of number is: {}", number);

    let are_equal: bool = compare(1, 2);

    print!("Are equal: {}", are_equal);
    let abdc = 1;

}