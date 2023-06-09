// Conditionals - Used to check the condition of something and act on the result

pub fn run(){

    let age: u8 = 22;

    let check_id: bool = true;

    let knows_person_of_age = true;


    // iif/else

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    }
    else if age <21 && check_id {
        println!("Sorry, you have to leave!");
    } else {
        println!("Bartender: I'll need to see youre ID");
    }

    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
} 