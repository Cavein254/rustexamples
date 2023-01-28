fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(100);
    v.push(30);
    v.push(50);
    v.push(70);

    // let first = [0];
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    // println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}
