#![allow(dead_code)]
use std::marker::PhantomData;
use struct_reflection::StructReflection;
use struct_reflection::StructReflectionHelper;

// Example with multiple generic parameters
#[derive(StructReflection)]
struct MultiGeneric<T, U, V> {
    first_data: T,
    second_data: U,
    third_data: V,
    t_array: [T; 2],
    u_array: [U; 3],
    mixed_array: [(T, U); 2], // Array of tuples
    _phantom: PhantomData<(T, U, V)>,
}

// A struct with nested generics
#[derive(StructReflection)]
struct NestedGenerics<T, U> {
    outer_data: T,
    inner: MultiGeneric<U, String, bool>,
    counts: [u32; 4],
}

fn main() {
    // Using reflection with multiple generic parameters
    println!("MultiGeneric reflection:");
    let multi_fields = MultiGeneric::<i32, String, bool>::struct_reflection().unwrap();
    for field in multi_fields {
        println!("  {}", field);
    }

    // Using reflection with nested generics
    println!("\nNestedGenerics reflection:");
    let nested_fields = NestedGenerics::<f64, char>::struct_reflection().unwrap();
    for field in nested_fields {
        println!("  {}", field);
    }
}
