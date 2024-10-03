#[test]

/*
// Fix the errors to make the code work.
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point{x: 5, y: 10};
    println!("{}",p.distance_from_origin());
}
*/




fn main() {
    let p = Point { x: 5.0, y: 10.0 };  // Используем значения типа f32
    println!("{}", p.distance_from_origin());
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


/*
В main создаём объект Point<f32>, чтобы соответствовать реализации метода distance_from_origin,
который работает с числами с плавающей запятой (f32).
Для Point используем значения 5.0 и 10.0 типа f32, чтобы избежать ошибки типов.
*/