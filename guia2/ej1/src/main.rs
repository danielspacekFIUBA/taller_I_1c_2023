fn main() {
    let s1 = String::from("hola");
    let mut v = Vec::new();
    v.push(s1);
    //println!("{:?}", v);
    let s2 = v.swap_remove(0);
    println!("{}", s2);
}
