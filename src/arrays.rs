fn main() {
    let values = vec![1, 2, 3, 4, 5];

    for value in &values {
        println!("Value {} :", value);
    }

    if values.len() > 5 {
        println!("The values in values {}:", values.len());
    }

    match values.len() {
        0 => println!("Empty"),
        1 => println!("One value"),
        2..=10 => println!("Between two and ten"),
        11 => println!("Eleven values"),
        _ => println!("Many Values"),
    }

    // while let some(value) = values.pop() {
    //     println!("Value = {value}");
    // }
}
