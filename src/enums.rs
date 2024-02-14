#[derive(Debug)]
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
// Vectors can only store values that are the same type. 
// This can be inconvenient; there are definitely use cases for needing to store a list of items of different types. 
// Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!

// For example, say we want to get values from a row in a spreadsheet in which some of the columns in the row contain integers, some floating-point numbers, and some strings. We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type: that of the enum.
// Then we can create a vector to hold that enum and so, ultimately, holds different types. We’ve demonstrated this in Listing 8-9.


pub fn enum_type_vector() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("cell value is {:?}", i)
    }
}