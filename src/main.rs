// use std::collections::HashMap;
mod tree_node;

mod distinct_names;

// mod solve;
// // Define a tuple struct
// #[derive(Debug)]
// struct KeyPress(String, char);

// // Define a classic struct
// #[derive(Debug)]
// struct MouseClick { x: i64, y: i64 }

// // Define the WebEvent enum variants to use the data from the structs
// // and a boolean type for the page Load variant
// #[derive(Debug)]
// enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

// fn order() -> HashMap<i32, String> {
//     let mut orders: HashMap<i32, String> = HashMap::new();
//     orders.insert(12, String::from('yyyy'));
//     return orders;
// }

// struct Car {
//     color: String,
//     transmission: Transmission,
//     convertible: bool,
//     mileage: u32,
// }

// #[derive(PartialEq, Debug)]
// enum Transmission {
//     Manual,
//     SemiAuto,
//     Automatic,
// }

// fn car_factory(color:String, transmission: Transmission, convertible: bool) -> Car {
//     Car {color: color, transmission: transmission, convertible:convertible, mileage:12}
// }

fn main() {
    // println!('Hello, world!');

    // get_max_repetitions::get_max_repetitions("bababababa".to_string(), 11, "baab".to_string(), 1);

    // let larger_len = longest::length_of_longest_substring('abcadefg'.to_string());

    // println!('{}', larger_len);

    // let mut board = vec![
    //     vec!['X', 'O', 'X', 'O', 'O', 'O', 'O'],
    //     vec!['X', 'O', 'O', 'O', 'O', 'O', 'O'],
    //     vec!['X', 'O', 'O', 'O', 'O', 'X', 'O'],
    //     vec!['O', 'O', 'O', 'O', 'X', 'O', 'X'],
    //     vec!['O', 'X', 'O', 'O', 'O', 'O', 'O'],
    //     vec!['O', 'O', 'O', 'O', 'O', 'O', 'O'],
    //     vec!['O', 'X', 'O', 'O', 'O', 'O', 'O'],
    // ];

    // solve::solve(&mut board);

    // println!("{:?}", board);

    // struct Student {  name: String, level: u8, remote: bool }

    // struct Grades(char, char, char ,char, f32);

    // struct Unit;

    // let car = car_factory(String::from('Red'), Transmission::Manual, false);
    // println!('Car 1 = {}, {:?} transmisstion, convertible: {}, mileage: {}', car.color, car.transmission, car.convertible, car.mileage);

    // let user_1 = Student { name: String::from('Constance Sharma'), remote: true, level:2};

    // let mark_1 = Grades('A','B','C','D', 2.75);

    // let fruits = vec!['banana', 'apple', 'coconut', 'orange'];
    // for &index in [0,2,99].iter() {
    //     match fruits.get(index) {
    //         Some(&'coconut') => println!('Coconuts are awesome!!!'),
    //         Some(fruit_name) => println!('It's a elicious {}!', fruit_name),
    //         None => println!('Threre is no fruit! :(')
    //     }
    // }

    // println!('{}, level{}. Remote: {}', user_1.name, user_1.level, user_1.remote);
    // println!('{}, level{}. Remote: {}', mark_1.0, mark_1.1, mark_1.2);

    // // Instantiate a MouseClick struct and bind the coordinate values
    // let click = MouseClick { x: 100, y: 250 };
    // println!('Mouse click location: {}, {}', click.x, click.y);

    // // Instantiate a KeyPress tuple and bind the key values
    // let keys = KeyPress(String::from('Ctrl+'), 'N');
    // println!('\nKeys pressed: {}{}', keys.0, keys.1);

    // // Instantiate WebEvent enum variants
    // // Set the boolean page Load value to true
    // let we_load = WebEvent::WELoad(true);
    // // Set the WEClick variant to use the data in the click struct
    // let we_click = WebEvent::WEClick(click);
    // // Set the WEKeys variant to use the data in the keys tuple
    // let we_key = WebEvent::WEKeys(keys);

    // // Print the values in the WebEvent enum variants
    // // Use the {:#?} syntax to display the enum structure and data in a readable form
    // println!('\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}', we_load, we_click, we_key);
    // let _ = file::open_file();
}
