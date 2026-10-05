fn main() {
    let age = 10;
    let ticket_price = if age < 12 { 5 } else { 10 };
    println!("The ticket price is: {}", ticket_price);
}