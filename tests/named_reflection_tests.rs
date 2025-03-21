#![allow(dead_code)]
use std::marker::PhantomData;

use struct_reflection::StructReflection;
use struct_reflection::StructReflectionHelper;

// Simple struct without generics
#[derive(StructReflection)]
struct BasicStruct {
    field_one: i32,
    field_two: String,
    field_three: bool,
}

// Generic struct with one type parameter
#[derive(StructReflection)]
struct GenericStruct<T> {
    data: T,
    count: usize,
    enabled: bool,
}

// Generic struct with multiple type parameters
#[derive(StructReflection)]
struct MultiGenericStruct<T, U> {
    first_data: T,
    second_data: U,
    description: String,
}

// Struct with nested struct
#[derive(StructReflection)]
struct OuterStruct {
    name: String,
    inner: BasicStruct,
}

// Struct with array fields
#[derive(StructReflection)]
struct ArrayStruct {
    values: [i32; 5],
    names: [String; 2],
}

// Struct with nested arrays (fixed types)
#[derive(StructReflection)]
struct NestedArrayStruct {
    // 2x3 array of integers
    matrix: [[i32; 3]; 2],
    // 2x2 array of strings
    string_grid: [[String; 2]; 2],
    // A simple field
    description: String,
}

// Struct with nested arrays containing generic type
#[derive(StructReflection)]
struct GenericArrayStruct<T> {
    // Array of T
    simple_array: [T; 4],
    // Nested array with generic inner type
    nested_array: [[T; 3]; 2],
    // Integer to ensure non-array fields work too
    count: i32,
}

// Mixed array struct with both generic and concrete types
#[derive(StructReflection)]
struct MixedArrayStruct<T> {
    // Array of generic type
    generic_array: [T; 3],
    // Nested array with concrete inner type
    concrete_nested: [[i32; 2]; 3],
    // Nested array with generic outer and concrete inner
    mixed_nested: [[i32; 2]; 4],
    // PhantomData field
    _phantom: PhantomData<T>,
}

// Struct with arrays of multiple generic types
#[derive(StructReflection)]
struct MultiGenericArrayStruct<T, U> {
    // Array of type T
    t_array: [T; 3],
    // Array of type U
    u_array: [U; 2],
    // Simple value
    name: String,
}

// Struct with nested arrays of multiple generic types
#[derive(StructReflection)]
struct NestedMultiGenericStruct<T, U> {
    // Nested array with type T
    t_matrix: [[T; 2]; 2],
    // Nested array with type U
    u_matrix: [[U; 3]; 1],
    // Numeric ID
    id: u32,
}

// Struct with arrays mixing multiple generic types and PhantomData
#[derive(StructReflection)]
struct ComplexMultiGenericStruct<T, U, V> {
    // Simple arrays of different types
    first_array: [T; 2],
    second_array: [U; 3],

    // Mixed array (combining multiple types)
    mixed_values: [(T, U); 2],

    // PhantomData for all types
    _phantom: PhantomData<(T, U, V)>,
}

// Struct with optional primitive fields
#[derive(StructReflection)]
struct OptionalFieldsStruct {
    id: u64,
    name: String,
    maybe_count: Option<i32>,
    maybe_active: Option<bool>,
}

// Struct with optional struct field
#[derive(StructReflection)]
struct NestedOptionalStruct {
    id: u64,
    maybe_basic: Option<BasicStruct>,
}

// Optional with generic type parameter
#[derive(StructReflection)]
struct OptionalGenericStruct<T> {
    id: u64,
    maybe_data: Option<T>,
    maybe_array: Option<[T; 2]>,
}

#[test]
fn test_basic_struct() {
    let names = BasicStruct::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec!["field_one", "field_two", "field_three"]
    );
}

#[test]
fn test_generic_struct() {
    let names = GenericStruct::<i32>::struct_reflection();
    assert_eq!(names.unwrap(), vec!["data", "count", "enabled"]);
}

#[test]
fn test_multi_generic_struct() {
    let names = MultiGenericStruct::<i32, String>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec!["first_data", "second_data", "description"]
    );
}

#[test]
fn test_outer_struct() {
    let names = OuterStruct::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "name",
            "inner__field_one",
            "inner__field_two",
            "inner__field_three"
        ]
    );
}

#[test]
fn test_array_struct() {
    let names = ArrayStruct::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "values__0",
            "values__1",
            "values__2",
            "values__3",
            "values__4",
            "names__0",
            "names__1"
        ]
    );
}

#[test]
fn test_nested_array_struct() {
    let names = NestedArrayStruct::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "matrix__0__0",
            "matrix__0__1",
            "matrix__0__2",
            "matrix__1__0",
            "matrix__1__1",
            "matrix__1__2",
            "string_grid__0__0",
            "string_grid__0__1",
            "string_grid__1__0",
            "string_grid__1__1",
            "description"
        ]
    );
}

#[test]
fn test_generic_array_struct() {
    let names = GenericArrayStruct::<i32>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "simple_array__0",
            "simple_array__1",
            "simple_array__2",
            "simple_array__3",
            "nested_array__0__0",
            "nested_array__0__1",
            "nested_array__0__2",
            "nested_array__1__0",
            "nested_array__1__1",
            "nested_array__1__2",
            "count"
        ]
    );
}

#[test]
fn test_mixed_array_struct() {
    let names = MixedArrayStruct::<String>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "generic_array__0",
            "generic_array__1",
            "generic_array__2",
            "concrete_nested__0__0",
            "concrete_nested__0__1",
            "concrete_nested__1__0",
            "concrete_nested__1__1",
            "concrete_nested__2__0",
            "concrete_nested__2__1",
            "mixed_nested__0__0",
            "mixed_nested__0__1",
            "mixed_nested__1__0",
            "mixed_nested__1__1",
            "mixed_nested__2__0",
            "mixed_nested__2__1",
            "mixed_nested__3__0",
            "mixed_nested__3__1",
            "_phantom"
        ]
    );
}

#[test]
fn test_multi_generic_array_struct() {
    let names = MultiGenericArrayStruct::<i32, String>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "t_array__0",
            "t_array__1",
            "t_array__2",
            "u_array__0",
            "u_array__1",
            "name"
        ]
    );
}

#[test]
fn test_nested_multi_generic_struct() {
    let names = NestedMultiGenericStruct::<bool, f64>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "t_matrix__0__0",
            "t_matrix__0__1",
            "t_matrix__1__0",
            "t_matrix__1__1",
            "u_matrix__0__0",
            "u_matrix__0__1",
            "u_matrix__0__2",
            "id"
        ]
    );
}

#[test]
fn test_complex_multi_generic_struct() {
    let names = ComplexMultiGenericStruct::<i32, String, f32>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "first_array__0",
            "first_array__1",
            "second_array__0",
            "second_array__1",
            "second_array__2",
            "mixed_values__0__0",
            "mixed_values__0__1",
            "mixed_values__1__0",
            "mixed_values__1__1",
            "_phantom"
        ]
    );
}

#[test]
fn test_optional_fields_struct() {
    let names = OptionalFieldsStruct::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec![
            "id",
            "name",
            "maybe_count__optional",
            "maybe_active__optional"
        ]
    );
}

#[test]
fn test_nested_optional_struct() {
    let names = NestedOptionalStruct::struct_reflection();
    assert_eq!(names.unwrap(), vec!["id", "maybe_basic__optional"]);
}

#[test]
fn test_optional_generic_struct() {
    let names = OptionalGenericStruct::<i32>::struct_reflection();
    assert_eq!(
        names.unwrap(),
        vec!["id", "maybe_data__optional", "maybe_array__optional"]
    );
}
