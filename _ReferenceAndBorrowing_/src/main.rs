fn main() {
    // References N Borrowing
    /* let a = 10;
    let b = &a;
    let c = &b;
    println!("a: {}, b: {}, c: {}", a,b,c);
    println!("The address of b is: {:p}",b);
    println!("The address of c is: {:p}",c); */

    /* //length function 
    let country = String::from("Pakistan");
    let result = length(&country);
    println!("{} {}", result , country); 
    */

    // change function , mut references
    let mut greeting = String::from("Hello");
    change(&mut greeting);

}

fn length(name: &String) -> usize {
    name.len()
}

fn change(x: &mut String){
    x.push_str(" Rust");
    println!("{:?}", x);
}