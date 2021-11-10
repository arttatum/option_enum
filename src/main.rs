// Rust does not have a 'null' that can be allocated to reference types. 
// Instead it has a construct called Option. Option<T> is an Enum with 2 variants: 
//  - None (essentially the same as null)
//  - Some<T> (where T is the same type as that in Option<T>)

// The magic of this, is that nulls/None MUST be handled, if a type is 
// nullable/expressed as Option<T>. No more null references!

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);}

    
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// This won't compile, because we're not handling the None case.
// matches on enums are 'exhaustive', if we haven't handled a specific case,
// the compiler will tell us about it, and not continue!

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }