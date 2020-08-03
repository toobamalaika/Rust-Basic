// struct Square(u32, u32);
fn main() {
    let square_1 = (100,150);
    let result = another_call(square_1);
    println!("Square formula: {}", result);

}

fn another_call(Dimensions: (u32,u32)) -> u32 {
    Dimensions.0 * Dimensions.1
}
