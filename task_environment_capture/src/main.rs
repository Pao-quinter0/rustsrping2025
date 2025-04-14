// Task 2: Environment Capture
fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Tracker count: {}", tracker);
    };

    update(); 
    update(); 
}

fn main() {
    track_changes();
}
