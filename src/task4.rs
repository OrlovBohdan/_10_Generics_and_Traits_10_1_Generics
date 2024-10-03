#[test]

/*
// Modify this struct to make the code work
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}
*/




fn main() {
    // DON'T modify this code.
    let _p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}

// Modify this struct to make the code work
#[allow(dead_code)]
struct Point<T,U> {
    x: T,
    y: U
}

/*
Чтобы структура Point могла принимать разные типы для полей x и y,
необходимо сделать её полями обобщёнными, но с разными параметрами типа.
Это можно сделать, указав два параметра типа: один для x, а другой для y.
*/