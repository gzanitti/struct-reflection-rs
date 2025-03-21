#![allow(dead_code)]
use std::marker::PhantomData;

use struct_reflection::StructReflection;
use struct_reflection::StructReflectionHelper;

// Simple tuple struct without generics
#[derive(StructReflection)]
struct BasicTupleStruct(
    i32,    // 0
    String, // 1
    bool,   // 2
);

// Generic tuple struct with one type parameter
#[derive(StructReflection)]
struct GenericTupleStruct<T>(
    T,     // 0
    usize, // 1
    bool,  // 2
);

// Generic tuple struct with multiple type parameters
#[derive(StructReflection)]
struct MultiGenericTupleStruct<T, U>(
    T,      // 0
    U,      // 1
    String, // 2
);

// Tuple struct with nested struct
#[derive(StructReflection)]
struct OuterTupleStruct(
    String,           // 0
    BasicTupleStruct, // 1
);

// Tuple struct with array fields
#[derive(StructReflection)]
struct ArrayTupleStruct(
    [i32; 5],    // 0
    [String; 2], // 1
);

// Tuple struct with nested arrays (fixed types)
#[derive(StructReflection)]
struct NestedArrayTupleStruct(
    [[i32; 3]; 2],    // 0: 2x3 array of integers
    [[String; 2]; 2], // 1: 2x2 array of strings
    String,           // 2: A simple field
);

// Tuple struct with nested arrays containing generic type
#[derive(StructReflection)]
struct GenericArrayTupleStruct<T>(
    [T; 4],      // 0: Array of T
    [[T; 3]; 2], // 1: Nested array with generic inner type
    i32,         // 2: Integer to ensure non-array fields work too
);

// Mixed array tuple struct with both generic and concrete types
#[derive(StructReflection)]
struct MixedArrayTupleStruct<T>(
    [T; 3],         // 0: Array of generic type
    [[i32; 2]; 3],  // 1: Nested array with concrete inner type
    [[i32; 2]; 4],  // 2: Nested array with generic outer and concrete inner
    PhantomData<T>, // 3: PhantomData field
);

// Tuple struct with arrays of multiple generic types
#[derive(StructReflection)]
struct MultiGenericArrayTupleStruct<T, U>(
    [T; 3], // 0: Array of type T
    [U; 2], // 1: Array of type U
    String, // 2: Simple value
);

// Tuple struct with nested arrays of multiple generic types
#[derive(StructReflection)]
struct NestedMultiGenericTupleStruct<T, U>(
    [[T; 2]; 2], // 0: Nested array with type T
    [[U; 3]; 1], // 1: Nested array with type U
    u32,         // 2: Numeric ID
);

// Tuple struct with arrays mixing multiple generic types and PhantomData
#[derive(StructReflection)]
struct ComplexMultiGenericTupleStruct<T, U, V>(
    [T; 2],                 // 0: Simple arrays of different types
    [U; 3],                 // 1
    [(T, U); 2],            // 2: Mixed array (combining multiple types in tuples)
    PhantomData<(T, U, V)>, // 3: PhantomData for all types
);

// Tuple struct with optional primitive fields
#[derive(StructReflection)]
struct OptionalFieldsTuple(
    u64,          // 0
    String,       // 1
    Option<i32>,  // 2
    Option<bool>, // 3
);

// Tuple struct with optional struct field
#[derive(StructReflection)]
struct NestedOptionalTuple(
    u64,                      // 0
    Option<BasicTupleStruct>, // 1
);

// Optional with generic type parameter
#[derive(StructReflection)]
struct OptionalGenericTuple<T>(
    u64,            // 0
    Option<T>,      // 1
    Option<[T; 2]>, // 2
);

#[test]
fn test_basic_tuple_struct() {
    let names = BasicTupleStruct::struct_reflection();
    assert_eq!(names.unwrap(), vec!["0", "1", "2"]);
}

#[test]
fn test_generic_tuple_struct() {
    let names = GenericTupleStruct::<i32>::struct_reflection();
    assert_eq!(names.unwrap(), vec!["0", "1", "2"]);
}

#[test]
fn test_multi_generic_tuple_struct() {
    let names = MultiGenericTupleStruct::<i32, String>::struct_reflection();
    assert_eq!(names.unwrap(), vec!["0", "1", "2"]);
}

#[test]
fn test_outer_tuple_struct() {
    let names = OuterTupleStruct::struct_reflection();
    assert_eq!(names.unwrap(), vec!["0", "1"]);
}

#[test]
fn test_array_tuple_struct() {
    let names = ArrayTupleStruct::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec!["0__0", "0__1", "0__2", "0__3", "0__4", "1__0", "1__1"]
    );
}

#[test]
fn test_nested_array_tuple_struct() {
    let names = NestedArrayTupleStruct::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "0__0__0", "0__0__1", "0__0__2", "0__1__0", "0__1__1", "0__1__2", "1__0__0", "1__0__1",
            "1__1__0", "1__1__1", "2"
        ]
    );
}

#[test]
fn test_generic_array_tuple_struct() {
    let names = GenericArrayTupleStruct::<i32>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "0__0", "0__1", "0__2", "0__3", "1__0__0", "1__0__1", "1__0__2", "1__1__0", "1__1__1",
            "1__1__2", "2"
        ]
    );
}

#[test]
fn test_mixed_array_tuple_struct() {
    let names = MixedArrayTupleStruct::<String>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "0__0", "0__1", "0__2", "1__0__0", "1__0__1", "1__1__0", "1__1__1", "1__2__0",
            "1__2__1", "2__0__0", "2__0__1", "2__1__0", "2__1__1", "2__2__0", "2__2__1", "2__3__0",
            "2__3__1", "3"
        ]
    );
}

#[test]
fn test_multi_generic_array_tuple_struct() {
    let names = MultiGenericArrayTupleStruct::<i32, String>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec!["0__0", "0__1", "0__2", "1__0", "1__1", "2"]
    );
}

#[test]
fn test_nested_multi_generic_tuple_struct() {
    let names = NestedMultiGenericTupleStruct::<bool, f64>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec!["0__0__0", "0__0__1", "0__1__0", "0__1__1", "1__0__0", "1__0__1", "1__0__2", "2"]
    );
}

#[test]
fn test_complex_multi_generic_tuple_struct() {
    let names = ComplexMultiGenericTupleStruct::<i32, String, f32>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "0__0", "0__1", "1__0", "1__1", "1__2",
            "2__0__0", // First element of tuple at index 0
            "2__0__1", // Second element of tuple at index 0
            "2__1__0", // First element of tuple at index 1
            "2__1__1", // Second element of tuple at index 1
            "3"
        ]
    );
}

#[test]
fn test_optional_fields_tuple() {
    let names = OptionalFieldsTuple::struct_reflection();
    assert_eq!(names.unwrap(), vec!["0", "1", "2__optional", "3__optional"]);
}

#[test]
fn test_nested_optional_tuple() {
    let names = NestedOptionalTuple::struct_reflection();
    assert_eq!(names.unwrap(), vec!["0", "1__optional"]);
}

#[test]
fn test_optional_generic_tuple() {
    let names = OptionalGenericTuple::<i32>::struct_reflection();
    assert_eq!(names.unwrap(), vec!["0", "1__optional", "2__optional"]);
}
