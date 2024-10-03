#[test]

/*
// Implement the generic function below.
fn sum

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}
*/




fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}
use std::ops::Add;

// Реализуем обобщённую функцию `sum`
fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

/*
Обобщённая функция sum: Мы используем обобщение с ограничением по трейтам.
T: Add<Output = T> говорит компилятору, что тип T должен поддерживать операцию
сложения (Add), и результат сложения должен быть того же типа (Output = T).
*/