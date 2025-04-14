// Task 3: Vector Transformation
// Version 2: Using a for loop
fn process_vector_with_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector_with_loop(numbers.clone(), |x| x * 2);
    let replaced = process_vector_with_loop(numbers, |x| if x > 2 { 0 } else { x });

    println!("Doubled (loop): {:?}", doubled);
    println!("Replaced (loop): {:?}", replaced);
}
