#[derive(Debug)]
struct Vehicle{
    name: String,
    year: u32,
    brand: String,
    available: bool,
}

fn main() {
    let vehicle_1 = Vehicle{
        name: String::from("Aqua"),
        year: 2018,
        brand: String::from("Toyota"),
        available: true,
    };
    // Immutable or changeable struct
    let mut vehicle_2 = Vehicle{
        name: String::from("Civic"),
        year: 2019,
        brand: String::from("Honda"),
        available: true,
    };
    vehicle_2.year = 2020;
    
    // Update Struct
    let mut vehicle_3 = Vehicle{
        name: String::from("Viggo"),
        year: vehicle_2.year,
        brand: String::from("Honda"),
        available: vehicle_2.available,
    };

    // If only 1 field is similar 
    let mut vehicle_4 = Vehicle{
        name: String::from("Premio"),
        ..vehicle_1
    };

    println!("{:#?} {:#?} {:#?} ", vehicle_2,vehicle_3,vehicle_4);

    // Call from another function
    let vehicle_3 = structCall(String::from("City"), String::from("Honda"));
    println!("{:#?}", vehicle_3);
}

fn structCall(name: String, brand: String) -> Vehicle {
    Vehicle{
        name: name,
        brand: brand,
        year: 2016,
        available: true,
    }
}