// Immutable var can be change , ny default it creates immutable var 
fn main() {       
    let fee = 50000;
    println!("The value of Fee is: {}", fee);
    fee = 55000;
    println!("The value of Fee is: {}", fee);
}


// // Mutable var can be change , it defines with 'mut' keyword
// fn main(){
//     let mut voucher = 540;
//     println!("The value of voucher is {}", voucher);
//     voucher = 800;
//     println!("The value of voucher is {}", voucher);
// }