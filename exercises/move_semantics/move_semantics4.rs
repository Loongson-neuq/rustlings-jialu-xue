// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let vec = Vec::new();

    let mut vec = foo(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `foo()` does NOT take `vec: Vec<i32>` as argument
fn foo() -> &mut Vec<i32> {


    vec.push(22);
    vec.push(44);
    vec.push(66);

}
