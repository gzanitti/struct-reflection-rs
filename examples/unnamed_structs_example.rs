#![allow(dead_code)]
use struct_reflection::StructReflection;
use struct_reflection::StructReflectionHelper;

// Basic tuple struct (unnamed fields)
#[derive(StructReflection)]
struct BasicTuple(i32, String, bool);

// Generic tuple struct
#[derive(StructReflection)]
struct GenericTuple<T, U>(
    T,           // Field 0
    U,           // Field 1
    String,      // Field 2
    [T; 3],      // Field 3: Array of the first generic type
    [(U, T); 2], // Field 4: Array of tuples combining both generic types
);

// Nested tuple struct
#[derive(StructReflection)]
struct NestedTuple(
    BasicTuple,                // Field 0: Another tuple struct
    [[f64; 2]; 2],             // Field 1: A nested array
    GenericTuple<i32, String>, // Field 2: A generic tuple struct
);

fn main() {
    // Get field names for basic tuple struct
    println!("BasicTuple reflection:");
    let basic_fields = BasicTuple::struct_reflection().unwrap();
    for field in basic_fields {
        println!("  {}", field);
    }

    // Get field names for generic tuple struct
    println!("\nGenericTuple reflection:");
    let generic_fields = GenericTuple::<bool, f32>::struct_reflection().unwrap();
    for field in generic_fields {
        println!("  {}", field);
    }

    // Get field names for nested tuple struct
    println!("\nNestedTuple reflection:");
    let nested_fields = NestedTuple::struct_reflection().unwrap();
    for field in nested_fields {
        println!("  {}", field);
    }
}
