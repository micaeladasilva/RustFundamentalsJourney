fn main() {
    let temperature_c = 20;
    let temperature_c = if temperature_c < 0 {"Freezing"} else if temperature_c < 25 {"Mild"} else {"Hot"};
    println!("{}", temperature_c);
}