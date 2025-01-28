
fn fact(x: i32) -> i32 {
    if x == 0 {
        1
    } else if x > 0 {
        x * fact(x - 1)
    } else {
        println!("Invalid");
        return -1;
    }
}
fn main() {
    let x = 5;
    let result = fact(x);

    println!("result: {}", result);


}