#[derive(Debug)]
// Tuple Struct , only value
struct Color(i32, i32, i32);
/*struct Student {
    name: &str,
    age: u32,
    bio: &str,
} */

fn main() {
    let black = Color(0,0,0);
    println!("Here is RGB Tuple struct{:#?}", black);    

    /* let student_1 = Student{
        name: "Rabeet",
        age: 1,
        bio: "Programmer"
    };
    println!("Here is String literal struct{:#?}", student_1); */
}
