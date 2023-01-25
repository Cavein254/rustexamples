fn add_suffix(s: String) {
    let mut p = String::from(" world!");
    p.push_str(&s);
    println!("{}", { p });
}
fn main() {
    let s = String::from("Hello");
    add_suffix(s);
    // println!("{}", { s }); //Here does not compile as s moves out of scope
}
