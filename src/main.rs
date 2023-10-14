fn main() {
    println!("fibonacci is {}", fibonacci(10));
}

fn fibonacci(num: u32) -> u32{
    if num == 0 {
        return 0;
    }
    else if num == 1{
        return  1;
    }
    else {
        return fibonacci(num - 1) + fibonacci(num - 2);
    }
}