#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32
}
fn main() {
    let rec_1 = (100,150);
    let result = another_call(rec_1);
    println!("Rectangle 1 Value: {}", result);

    let rec_2 = Rectangle{
        height: 34,
        width: 89,
    };
    let result_2 = area_measure(&rec_2);
    println!("Rectangle 2 Value: {:#?}", result_2);
    println!("Rectangle 2 Value: {:#?}", rec_2);
}

fn another_call(Dimensions: (u32,u32)) -> u32 {
    Dimensions.0 * Dimensions.1
}
fn area_measure(rec: &Rectangle) -> u32{
    rec.height * rec.width
}