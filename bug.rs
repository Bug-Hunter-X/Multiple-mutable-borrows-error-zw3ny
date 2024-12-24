fn main() {
    let mut x = 5;
    let y = &mut x;
    let z = &mut x; //Error here
    *y = 6;
    *z = 7;
    println!("{}", x);
}