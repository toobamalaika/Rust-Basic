// Scalar data type is contain integer , float , char and boolean. It contain a sigle value only
// Compound data type is contain tuple , array. It conatins a multiple values.
fn main() {
    // // Scalar Data Type
    // let age = 22;
    // let percentage = 85.6;
    // let grade = 'A';
    // let passed = true;
    // println!("Scalar Data :\n{}\n{}\n{}\n{}",age,percentage,grade,passed);

    // // Integer Data type
    // // Signed & unsigned data types
    // // Signed denoted by i and it is use when we know the value result return will be the negative
    // // Unsigned denoted by u and it is use when we know the value result return will be the positive
    // // Bits are 8 , 16 , 32 , 64 , 128 arch

    // let first_number : u8 = 228;
    // println!("\nFirst Value: {}",first_number); 

    // // Number literals 
    // let decimalNumber = 120;
    // let hexadecimalNumber = 0xff;
    // let octalNumber = 0o77;
    // let binaryNumber = 0b1111_0000;
    // let byteNumber = b'D';
    // println!("\nNumber literals are: \nDecimal {} \nHexadecimal {} \nOctal {} \nBinary {} \nByte {}" , decimalNumber,hexadecimalNumber,octalNumber,binaryNumber,byteNumber);

    // // Arithmatic Operations are add , subtraction , division , multiplication , modulus
    // let numberOne = 12;
    // let numberTwo = 6;
    // let sum = numberOne + numberTwo;
    // let sub = numberOne - numberTwo;
    // let div = numberOne / numberTwo;
    // let mul = numberOne * numberTwo;
    // let modulus = numberOne % numberTwo;
    // println!("\nArithmatic results are: \nAddition: {} \nSubstraction: {} \nDivison: {} \nMultiplication: {}\nModulus: {}", sum,sub,div,mul,modulus);

    // Compound Data Type tuples and array

    // Tuple is heterogeneous , contains multiple data type.
    let result: (u32,char,f64,bool) = (25,'A',80.5,true);
    println!("Value of 0 index is: {}",result.0); 
    println!("Value of 1 index is: {}",result.1); 
    println!("Value of 2 index is: {}",result.2);
    println!("Value of 3 index is: {}",result.3); 
    println!("Destructuring Of tuple");
    let (a,b,c,d) = result;
    println!("Value of A index is: {}",a);
    println!("Value of B index is: {}",b);
    println!("Value of C index is: {}",c);
    println!("Value of D index is: {}",d);

    // array is homogeneous , contains single data type.
    let _temperature_note:[u8; 7] = [40,48,45,38,42,44,49];
    println!("\nLast 7 days Temperature is: {:#?}",_temperature_note);
    println!("\nAccess with indexs=es");
    println!("6th Index value is: {}",_temperature_note[6]);

    // Single value print in multiple times
    let dollars = [5;8];
    println!("\nAll dollar Values are: {:#?}",dollars);

}
