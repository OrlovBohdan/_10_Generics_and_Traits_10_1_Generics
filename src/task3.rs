#[test]

/*

// Implement struct Point to make it work.


fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}
*/


// Implement struct Point to make it work.


fn main() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}
#[allow(dead_code)]
struct Point<T>{
    x:T,
    y:T
}

/*
Обобщённая структура Point: Мы объявляем структуру с параметром типа T, который будет использоваться для обоих полей x и y.
*/
