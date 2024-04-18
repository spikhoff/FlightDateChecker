use chrono::{NaiveDate, Datelike}; // Import chrono for date handling
use std::io;

// Define a struct for FlightBooking
struct FlightBooking {
    date: NaiveDate,
}

impl FlightBooking {
    // Validate the date input and create a new FlightBooking
    fn new(date_input: &str) -> Result<FlightBooking, String> {
        match NaiveDate::parse_from_str(date_input, "%Y-%m-%d") {
            Ok(date) => Ok(FlightBooking { date }),
            Err(_) => Err(String::from("Invalid date format. Please use YYYY-MM-DD format.")),
        }
    }
}

fn main() {
    println!("Enter the date for your flight booking (YYYY-MM-DD):");
    let mut date_input = String::new();

    // Read the user input
    io::stdin()
        .read_line(&mut date_input)
        .expect("Failed to read line");

    let date_input = date_input.trim(); // Trim the input to remove any extraneous whitespace

    // Create a flight booking
    match FlightBooking::new(date_input) {
        Ok(booking) => {
            println!("Flight booked for: {} (Year: {}, Month: {}, Day: {})",
                date_input, booking.date.year(), booking.date.month(), booking.date.day());
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
