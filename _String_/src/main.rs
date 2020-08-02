fn main() {
    // &str is fixed data type , it's immutable and called by string literals
    // String is mutable , dynamic size and callef by Stribg type

    // String type
    let mut name = String::from("Hey! ");
    name.push_str("Tooba");
    println!("{}", name);
    

    /* // String lioteral
    let name_1 = "Hello ";
    name_1.push_str(" Tooba");
    println!("{}", name_1);
    */
}
