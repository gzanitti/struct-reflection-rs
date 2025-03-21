# struct-reflection-rs

A Rust library for obtaining struct field names at runtime through reflection-like capabilities.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
struct-reflection = "0.1.0"
```

## Overview

This library provides a derive macro `StructReflection` that allows you to get field names from struct instances at runtime, which is useful for serialization, debugging, schema generation, and more.

## Basic Usage

```rust
use struct_reflection::StructReflection;

#[derive(StructReflection)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

fn main() {
    let fields = Person::struct_reflection().unwrap();
    println!("Person fields: {:?}", fields);
    // Output: Person fields: ["name", "age", "email__optional"]
}
```

## Features

- Get field names from named structs and tuple structs
- Support for generic structs
- Nested struct field reflection
- Array type handling
- Support for `Option<T>` fields

## Advanced Examples

### Nested Structs

```rust
#[derive(StructReflection)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[derive(StructReflection)]
struct User {
    id: u64,
    name: String,
    address: Address,
}

fn main() {
    let fields = User::struct_reflection().unwrap();
    println!("User fields: {:?}", fields);
    // Output: ["id", "name", "address__street", "address__city", "address__zip"]
}
```

### Array Fields

```rust
#[derive(StructReflection)]
struct Matrix {
    values: [[i32; 3]; 2],
    name: String,
}

fn main() {
    let fields = Matrix::struct_reflection().unwrap();
    println!("Matrix fields: {:?}", fields);
    // Output includes fields like: ["values__0_0", "values__0_1", ..., "name"]
}
```

### Generic Structs

```rust
#[derive(StructReflection)]
struct Container<T> {
    id: u64,
    data: T,
    timestamp: u64,
}

fn main() {
    let fields = Container::<String>::struct_reflection().unwrap();
    println!("Container<String> fields: {:?}", fields);
    // Output: ["id", "data", "timestamp"]
}
```

## Limitations

Currently, `Option<T>` fields are handled with a simplified approach that always returns a field with the `optional` suffix regardless of what `T` is. This is due to limitations in Rust's trait system and lack of specialization in stable Rust.
