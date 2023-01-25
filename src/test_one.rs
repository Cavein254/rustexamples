fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("{}, world!", s1); //does not compile
    println!("{s2}, world!");
}
