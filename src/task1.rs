#[test]


/*
// Fill in the blanks to make it work
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Using the non-generic functions
    reg_fn(__);          // Concrete type.
    gen_spec_t(__);   // Implicitly specified type parameter `A`.
    gen_spec_i32(__); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(__);

    // Implicitly specified type parameter `char` to `generic()`.
    generic(__);

    println!("Success!");
}*/


fn main() {
    // Используем негенерик функции
    reg_fn(S(A));             // Конкретный тип.
    gen_spec_t(SGen(A));      // Неявно указанный параметр типа `A`.
    gen_spec_i32(SGen(32));   // Неявно указанный параметр типа `i32`.

    // Явно указанный параметр типа `char` для `generic()`.
    generic::<char>(SGen('a'));

    // Неявно указанный параметр типа `char` для `generic()`.
    generic(SGen('b'));

    println!("Success!");
}

struct A;          // Конкретный тип `A`.
struct S(A);       // Конкретный тип `S`.
struct SGen<T>(T); // Обобщенный тип `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

/*
Для функции reg_fn нужно передать объект типа S.
Для функции gen_spec_t нужно передать объект типа SGen<A>.
Для функции gen_spec_i32 нужен объект типа SGen<i32>.
Для вызова generic::<char>() нужно передать объект типа SGen<char>.
Для вызова generic() с неявно указанным параметром типа нужно передать объект того же типа.
*/
