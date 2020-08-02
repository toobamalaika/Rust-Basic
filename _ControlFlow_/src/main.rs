fn main() {
    // Control flow
    /* let name = String::from("Rabeeta");
    if name != "Rabeet"{
        println!("User is anonymous!");
    }
    else{
        println!("User is Rabeet");
    } */

   /*let number = 0;
   if number > 0{
       println!("You entered positive number");
   } 
   else if number == 0 {
       println!("You entered 0 number");
   }
   else{
       println!("You entered negative number")
   }*/
    //Nested Control flow
    let name = String::from("Tooba");
    let marks = 80;

    if name == "Tooba"{
        if marks >= 80{
            println!("You got A+ grade!");
        }
        else if marks >= 70{
            println!("You got A grade!");
        }
        else if marks >= 60{
            println!("You got B grade!");
        }
        else if marks >= 50{
            println!("You got C grade!");
        }
        else if marks <= 40{
            println!("You're failed!");
        }
        else{
            println!("Need more improvement!");
        }
    }
    else{
        println!("You're not authorized person");
    }
   
}
