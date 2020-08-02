fn main() {
    /* // Stack memory allocation
    let a = 10;
    let b = a;
    println!("Stack Memory \nb: {} a: {}", b , a); */

    /* // Heap memory allocation
    let str_one = String::from("Hello Heap!");
    let str_two = str_one;
    println!("\nHeap Memory \nsting two: {} ", str_two);
    println!("\nHeap Memory \nsting two: {} string one: {}", str_two,str_one); */

    /* // Data clonning in heap memory
    let name = String::from("Tooba");
    let name_clone = name.clone();
    println!("Happy Heap Data copying! 😉");
    println!("Orignal String: {}\nClone String: {}", name,name_clone) */

    // Ownship in functions
    /* let s = String::from("Hey Ownership!");
    takeOwnership(s);
    println!("{}", s); */

    /* let result = giveOwnership();
    println!("{}", result); */

    let givenAndTake = String::from("Hey Give and Take!");
    let call_fun = takeNgiveOwnership(givenAndTake);
    println!("{}", call_fun);
}
fn takeNgiveOwnership(x: String) -> String {
    x
}

fn giveOwnership() -> String{
    let s = String::from("Gives Ownership to another function");
    s
}

fn takeOwnership(x: String){
    println!("{}",x)
}