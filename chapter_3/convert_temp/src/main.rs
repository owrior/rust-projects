fn main() {
    let temp_c: f64 = 100.0;
    println!("Temperature in C: {temp_c}");

    let temp_f = convert_c_to_f(temp_c);
    println!("Temperature in F: {temp_f}");

    let temp_c_2 = convert_f_to_c(temp_f);
    println!("Temperature in C: {temp_c_2}");
}

fn convert_c_to_f(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

fn convert_f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}
