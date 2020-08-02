fn main() {
    println!("Functions Practice!\n");
    /* let (values,values1) = some_calculation(4, 5.2);
    println!("Values of square functionality perform: {} {}", values,values1); */
    another_function();
    // calculate(25, 5);
    /* let student_1 = student_details("Rabeet", 15, 'A');
    println!("Student Details are: {:?}", student_1); */
}

// Simple function
fn another_function(){
    let a = {
        let x = 44;
        x+1 //expression, always returen a value
    }; //statment bind a values
    println!("This is from another function");
    println!("{}", a);
}

// // calculate values from other functions
// fn calculate(x:u8 , y:u8){
//     let result_1 = x+y;
//     let result_2 = x-y;
//     let result_3 = x*y;
//     let result_4 = x/y;
//     println!("Add result: {}", result_1);
//     println!("Sub result: {}", result_2);
//     println!("Mul result: {}", result_3);
//     println!("Div result: {}", result_4);
// }

// Parameters & Arguments
// Parameters always declare in functions
// Arguments send the data in functions parameter
// fn some_calculation(x: u32, y:f64) -> (u32,f64){
//    let result_1 = x*x; //statement
//     let result_2 = y*y; //statement
//     (result_1,result_2) //expression

// }
// fn student_details(name: &str , age:u32 , grade: char) -> (&str,u32,char){
//     let std_name = name;
//     let std_age = age;
//     let std_grade = grade;
//     (std_name,std_age,std_grade)
// }