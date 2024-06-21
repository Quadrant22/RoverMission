use std::io::{self, Read};

fn introduction(){
    println!("\n\nOne hectare of Bamboo grove can capture up to 60 tons of CO2 each year! ");
    println!("You can ask Opportunity to plant a Bamboo tree in Mars!");
    println!("This way, it can use C02 to produce Oxygen.");
    println!("---------------------------------------------");
}

struct Rover {
    name: String,
    age: u8, 
    landing_site: String,
    mission: String,
}

struct Location {
    name: String,
    distance_from_landing_site: i32, 
    area: i32, 
    safety: bool,
}

struct Atmosphere {
    oxygen_level: u8,
    carbondioxide: u8,
    dust_storms: u8, 
    cosmic_radiation: u8, 
    sunlight_intensity: i32,
}

fn describe_rover(descrb_rover: &Rover){
    println!("Rover Name: {}", descrb_rover.name); 
    println!("Rover Age: {}", descrb_rover.age); 
    println!("Rover Landing Site: {}", descrb_rover.landing_site); 
    println!("Rover Mission: {}", descrb_rover.mission); 
    println!("---------------------------------------------");
}

fn describe_location(descrb_location: &Location){
    println!("Location Name: {}", descrb_location.name);
    println!("Distance from Landing Site: {}", descrb_location.distance_from_landing_site); 
    println!("Area of the land: {}", descrb_location.area);
    println!("Safety: {}", descrb_location.safety);
    println!("---------------------------------------------");
}

fn describe_atmosphere(descrb_atmosphere: &Atmosphere){
    println!("Oxygen Level: {}", descrb_atmosphere.oxygen_level);
    println!("Carbondioxide Level: {}", descrb_atmosphere.carbondioxide);
    println!("Dust Storms Per Year: {}", descrb_atmosphere.dust_storms);
    println!("Cosmic Radiation Level: {}", descrb_atmosphere.cosmic_radiation);
    println!("Sunlight Intensity in Degrees: {}", descrb_atmosphere.sunlight_intensity);
    println!("---------------------------------------------");
}

fn select_suitable_location(){
    println!("Please select a location with fewer dust storms.");
    println!("Meridiani Planum Or Terra Sirenum");
    let mut choice: String = String::new(); 
    io::stdin().read_line(&mut choice).unwrap();
    println!("Selected Location: {}", choice);
}

fn main() {
    introduction();

    let rover_1 = Rover {
        name: String::from("Opportunity"),
        age: 14,
        landing_site: String::from("Meridiani Planum"),
        mission: String::from("Creates Oxygen by Planting Bamboo Trees."),
    };

    let location_1 = Location {
        name: String::from("Meridiani Planum"),
        distance_from_landing_site: 0,
        area: 100,
        safety: true,
    }; 

    let atmosphere_1 = Atmosphere {
        oxygen_level: 2,
        carbondioxide: 95,
        dust_storms: 3,
        cosmic_radiation: 100,
        sunlight_intensity: 10,
    };

    let location_2 = Location {
        name: String::from("Terra Sirenum"),
        distance_from_landing_site: 1000,
        area: 3000,
        safety: true,
    }; 

    let atmosphere_2 = Atmosphere {
        oxygen_level: 2,
        carbondioxide: 95,
        dust_storms: 1,
        cosmic_radiation: 100,
        sunlight_intensity: 36,
    };

    describe_rover(&rover_1);

    println!("\n\nTO LEARN MORE ABOUT SUITABLE LOCATIONS, PLEASE TYPE A COMMAND.");
    println!("Commands: "); 
    println!("Meridiani Planum, Meridiani Atmosphere, Terra Sirenum, Terra Atmosphere"); 
    
    let mut user_choice: String = String::new(); 
    io::stdin().read_line(&mut user_choice).unwrap();
    user_choice = user_choice.trim().to_lowercase();

    loop {
        match user_choice.as_str() {
            "meridiani planum" => describe_location(&location_1),
            "meridiani atmosphere" => describe_atmosphere(&atmosphere_1),
            "terra sirenum" => describe_location(&location_2),
            "terra atmosphere" => describe_atmosphere(&atmosphere_2),
            _ => println!("I don't understand that command."),
        }
        break;
    }

    select_suitable_location(); 

    println!("Thank you for your selection!");    
    println!("Before we plant a tree, we'll need to build a 20x20 ft protective sheild.");  
}
