#[allow(unused_variables, unused_mut, dead_code)]
fn main() {
    fn main() {
        let mut sum = 0;
        for i in -3..2 {
            sum += i
        }
        println!("sum is {}", sum);
        assert!(sum == -3);
    
        for c in 'a'..='z' {
            println!("{}",c);
        }
    }
    
}

#[allow(dead_code,unused_variables)]
fn arryas_code() {
    let arr = [1, 2, 3, 4];

   
    match arr {
        [first, middle @ .., last] => {
            println!("First is {}", first);
            println!("Middle is {:?}", middle);
            println!("Last is {}", last);
        } 

    }

    
}


#[allow(dead_code,unused_variables)]
fn compare(x: i32, y: i32) -> bool {
    x == y
}
#[allow(dead_code,unused_variables)]
fn test_var() {
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