fn add_suffix(mut s: String) {
    s.push_str(" world");
    println!("{}", { s });
}
fn main() {
    let s = String::from("Hello");
    add_suffix(s);
    // println!("{}", { s }); //Here does not compile as s moves out of scope
}
