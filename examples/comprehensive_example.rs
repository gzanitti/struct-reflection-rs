#![allow(dead_code)]
use struct_reflection::StructReflection;
use struct_reflection::StructReflectionHelper;

// Example with primitive types and arrays
#[derive(StructReflection)]
struct ComplexStruct {
    // Primitive fields
    id: u64,
    name: String,
    active: bool,
    score: f64,

    // Array fields
    tags: [String; 3],
    scores: [f32; 5],

    // Nested arrays
    matrix: [[i32; 3]; 2],
}

fn main() {
    // Get field names for a complex struct with primitives and arrays
    if let Some(fields) = ComplexStruct::struct_reflection() {
        println!("ComplexStruct fields:");
        for field in fields {
            println!("  {}", field);
        }
    }
}
