fn largest(str1: &'str, str2:&'str){
    if str1.len() > str2.len() {
        str1
    }else {
        str2
    }
}

fn main() {
    let str1 = "Another";
    let str2 = "Moster ";
    let results = largest(str1,str2);
    println!("Largerst {}", results);
}