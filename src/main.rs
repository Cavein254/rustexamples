fn borrow_value(value:i32) -> i32 {
    let mut new_value =  value;
    new_value += 1;
    new_value
}

fn main(){
    let my_value: i32 = 42;
    let new_value = borrow_value(my_value);
    let other_value = borrow_value(my_value);

    println!("Hello world, {}{}", new_value, other_value);
}
