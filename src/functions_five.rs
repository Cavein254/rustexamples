fn main() {
    let x = my_fn();
    if x < 5 {
        println!("x is super small");
    }
    if x > 5 {
        println!("x is large as {x}");
    }
}

fn my_fn()->i32{
    45 * 23
}
