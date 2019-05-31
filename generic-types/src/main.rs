fn largest_number(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Wont compile -> std::cmp::PartialOrd must be implemented
// fn largest_item<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x: 32, y: 23 };
    let float = Point { x: 3.2, y: 2.2 };
    let mix = Point { x: 3.2, y: 2 };

    println!("Distance from origin: {}", float.distance_from_origin());

    println!("{:?}", integer);
    println!("{:?}", float);
    println!("{:?}", mix);
    println!("{}", mix.x());

    let number_list = vec![34, 50, 25, 100, 65];

    let largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    let result = largest_number(&number_list);
    println!("The largest number is: {}", result);

    println!("The largest number is: {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    let result = largest_number(&number_list);
    println!("The largest number is: {}", result);

    println!("The largest number is: {}", largest);
}
