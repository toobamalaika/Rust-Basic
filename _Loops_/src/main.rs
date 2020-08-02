fn main() {
    // Loops : Rust gives us three type of loop that is loop , while and for loop
    // loop condition is in loop block
    /* // Infinite loop
    loop{
        println!("This is infinite loop!");
    } */
    
    let mut counter = 0; 
    let counter_val = loop{
        counter = counter +1 ;
        println!("Hello Loop {}", counter);
        if counter == 10{
            break counter
        }
    };
    println!("Total number of print in loop: {}", counter_val);
    
    // while condition is out of while block
    /* let mut counter = 0;
    while counter < 5 {
        counter = counter+1;
        println!("Hello While loop {}",counter);   
    } */

    /* let mut counter = 0;
    let lottery_number = [87,54,4,84,31,71,61];
    while counter < lottery_number.len(){
        println!("{:#?}",lottery_number[counter]);
        counter = counter + 1;
    } */

    // for loop
    /* for number in 0..10{
        println!("{}. Hello for loop",number);
    } */

    /* // Reverse counter
    for number in (0..10).rev(){
        println!("{}. Hello for loop",number);
    } */

    /* let random_number = [10,20,30,40,60,80,14,48,24,92,49];
    for number in random_number.iter() {
        println!("{}. Iteration in radom number using array!",number);
    } */


    
}
