// Problem 1 (String Concatenation with Borrowing)
fn concat_strings(s1: &String, s2: &String) -> String {
    format!("{}{}", s1, s2)  
}

// Problem 2 (Clone and Modify)
fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone();
    cloned.push_str("World!");
    cloned
}

// Problem 3 (Mutable Reference Sum)
fn sum(total: &mut i32, low: i32, high: i32) {
    for i in low..=high {
        *total += i;
    }
}


fn main() {
    // Problem 1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); 

    // Problem 2
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); 
    println!("Modified: {}", modified); 

    // Problem 3
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total sum: {}", total); 
}
