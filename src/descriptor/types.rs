/*
 * Copyright (c) 2022 Juuz
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 *
 * You may obtain a copy of the License at
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

use std::fmt;
use crate::descriptor::ClassName;

/// A JVM type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// A named object type (`L<class name>;`).
    Object(ClassName),
    /// An array of a nested type (`[<nested type>`).
    Array(Box<Type>),
    /// The primitive type `byte` (`B`).
    /// Corresponds to Rust's `i8`.
    Byte,
    /// The primitive type `short` (`S`).
    /// Corresponds to Rust's `i16`.
    Short,
    /// The primitive type `int` (`I`).
    /// Corresponds to Rust's `i32`.
    Int,
    /// The primitive type `long` (`J`).
    /// Corresponds to Rust's `i64`.
    ///
    /// `long` occupies two method argument indices.
    Long,
    /// The primitive type `float` (`F`)
    /// Corresponds to Rust's `f32`.
    Float,
    /// The primitive type `double` (`D`).
    /// Corresponds to Rust's `f64`.
    ///
    /// `double` occupies two method argument indices.
    Double,
    /// The primitive type `boolean` (`Z`).
    /// Corresponds to Rust's `bool`.
    Boolean,
    /// The primitive type `char` (`C`).
    Char,
    /// The pseudo-type `void` (`V`).
    Void,
}

impl Type {
    /// Gets the JVM bytecode descriptor for this type.
    ///
    /// # Examples
    ///
    /// ```
    /// use jvm_obfuscation_mappings::descriptor::{ClassName, Type};
    ///
    /// let string = Type::Object(ClassName::from_binary_name("java.lang.String"));
    /// assert_eq!(&string.descriptor(), "Ljava/lang/String;");
    /// assert_eq!(&Type::Boolean.descriptor(), "Z");
    /// ```
    pub fn descriptor(&self) -> String {
        match self {
            Type::Object(name) => format!("L{};", name.internal_name()),
            Type::Array(element_type) => format!("[{}", element_type.descriptor()),
            Type::Byte => "B".to_owned(),
            Type::Short => "S".to_owned(),
            Type::Int => "I".to_owned(),
            Type::Long => "J".to_owned(),
            Type::Float => "F".to_owned(),
            Type::Double => "D".to_owned(),
            Type::Boolean => "Z".to_owned(),
            Type::Char => "C".to_owned(),
            Type::Void => "V".to_owned(),
        }
    }

    /// Gets the name this type would use in Java.
    ///
    /// # Examples
    ///
    /// ```
    /// use jvm_obfuscation_mappings::descriptor::{ClassName, Type};
    ///
    /// let string = Type::Object(ClassName::from_internal_name("java/lang/String"));
    /// assert_eq!(&string.java_name(), "java.lang.String");
    /// assert_eq!(&Type::Boolean.java_name(), "boolean");
    /// ```
    pub fn java_name(&self) -> String {
        match self {
            Type::Object(name) => name.binary_name(),
            Type::Array(element_type) => format!("{}[]", element_type.java_name()),
            Type::Byte => "byte".to_owned(),
            Type::Short => "short".to_owned(),
            Type::Int => "int".to_owned(),
            Type::Long => "long".to_owned(),
            Type::Float => "float".to_owned(),
            Type::Double => "double".to_owned(),
            Type::Boolean => "boolean".to_owned(),
            Type::Char => "char".to_owned(),
            Type::Void => "void".to_owned(),
        }
    }

    /// Returns an array type containing this type as its element type.
    ///
    /// # Examples
    ///
    /// ```
    /// use jvm_obfuscation_mappings::descriptor::ClassName;
    ///
    /// let string = ClassName::from_binary_name("java.lang.String").to_type();
    /// assert_eq!(string.array().java_name(), String::from("java.lang.String[]"));
    /// assert_eq!(string.array().array().array().java_name(), String::from("java.lang.String[][][]"));
    /// ```
    pub fn array(&self) -> Self {
        Type::Array(Box::new(self.clone()))
    }

    /// Returns the depth of array layers in this type.
    ///
    /// All non-array types return 0. Arrays return 1 + their element type's array depth.
    ///
    /// # Examples
    ///
    /// ```
    /// use jvm_obfuscation_mappings::descriptor::ClassName;
    ///
    /// let string = ClassName::from_binary_name("java.lang.String").to_type();
    /// assert_eq!(string.array_depth(), 0);
    /// assert_eq!(string.array().array().array().array_depth(), 3);
    /// ```
    pub fn array_depth(&self) -> u32 {
        match self {
            Type::Array(element_type) => 1 + element_type.array_depth(),
            _ => 0,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.descriptor())
    }
}
