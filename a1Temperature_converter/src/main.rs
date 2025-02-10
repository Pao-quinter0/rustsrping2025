const FREEZING_POINT: f64 = 32.0; 

fn fahr_to_cel(f: f64) -> f64 {
    (f - FREEZING_POINT) * 5.0 / 9.0
}

fn cel_to_fahr(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT
}

fn main() {
    let temp_f = 32.0; 
    
    println!("{:.2}°F = {:.2}°C", temp_f, fahr_to_cel(temp_f));
    
    for i in 1..=5 {
        let next_temp_f = temp_f + i as f64;
        println!("{:.2}°F = {:.2}°C", next_temp_f, fahr_to_cel(next_temp_f));
    }

    let temp_c = 0.0; 
    
    println!("{:.2}°C = {:.2}°F", temp_c, cel_to_fahr(temp_c));
    
    for i in 1..=5 {
        let next_temp_c = temp_c + i as f64;
        println!("{:.2}°C = {:.2}°F", next_temp_c, cel_to_fahr(next_temp_c));
    }
}
