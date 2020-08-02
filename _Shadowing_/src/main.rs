fn main() {
    // Shadowing is use for to reinitialized the var
    let movies = 6;
    println!("First value of shadowing is {} \n", movies);

    let movies = 25;
    println!("Second value of shadowing is {} \n", movies);

    let movies:f64 = 18.875;
    println!("Third value of shadowing is {} \n", movies);

    let movies = "Iron Man";
    println!("Fourth value of shadowing is {} \n", movies);

    println!("At the end of program the value is hold {}", movies);
}
