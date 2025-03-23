use std::marker::PhantomData;

pub use struct_reflection_derive::StructReflection;

pub trait StructReflection {
    fn struct_reflection() -> Option<Vec<String>>;
}

pub trait StructReflectionHelper {
    fn struct_reflection() -> Option<Vec<String>>;
}

impl<T: StructReflectionHelper, const N: usize> StructReflectionHelper for [T; N] {
    fn struct_reflection() -> Option<Vec<String>> {
        match T::struct_reflection() {
            Some(inner_fields) => {
                let mut fields = Vec::new();

                for i in 0..N {
                    for field in &inner_fields {
                        fields.push(format!("{i}__{field}"));
                    }
                }

                Some(fields)
            }
            None => Some((0..N).map(|i| i.to_string()).collect()),
        }
    }
}

// Note on Option<T> implementation:
//
// Ideally, we would handle Option<T> differently based on T:
// - For primitives: return a single "optional" field
// - For structs: return internal fields with "__optional" suffix
//
// This isn't currently possible in stable Rust due to trait coherence rules
// and lack of specialization. As a compromise, we use a simplified implementation
// that treats all Option<T> the same way, regardless of what T is.
//
// This might be improved in future versions when Rust's type system evolves.
impl<T> StructReflectionHelper for Option<T> {
    fn struct_reflection() -> Option<Vec<String>> {
        // Simple implementation that always returns "optional"
        Some(vec!["optional".to_string()])
    }
}

impl<T> StructReflectionHelper for PhantomData<T> {
    fn struct_reflection() -> Option<Vec<String>> {
        None
    }
}
