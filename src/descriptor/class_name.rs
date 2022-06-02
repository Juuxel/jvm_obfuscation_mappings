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
use crate::descriptor::Type;

/// A JVM class name.
///
/// # Class name formats
///
/// Class names on the JVM typically come in three different varieties:
/// - Java names (packages and nested classes separated using `.`)
/// - Binary names used by reflection (packages separated using `.` and nested classes using `$`)
/// - Internal names in the bytecode (packages separated using `/` and nested classes using `$`)
///
/// Of these, Java names aren't dealt with here since they need actual knowledge of the bytecode
/// which isn't relevant for mappings.
///
/// Obfuscation mappings tend to use internal names, so class names are stored as internal names.
/// This struct also offers methods to convert between the two main formats.
///
/// # Displaying class names
///
/// The [`Display`][std::fmt::Display] implementation outputs the internal name.
///
/// ```
/// use jvm_obfuscation_mappings::descriptor::ClassName;
///
/// let name = ClassName::from_binary_name("java.lang.Object");
/// assert_eq!(name.to_string(), String::from("java/lang/Object"));
/// ```
///
/// You can also convert directly to a specified output format using [`internal_name`][Self::internal_name]
/// or [`binary_name`][Self::binary_name].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassName {
    internal_name: String,
}

impl ClassName {
    /// Creates a class name from an internal name.
    ///
    /// # Examples
    ///
    /// ```
    /// use jvm_obfuscation_mappings::descriptor::ClassName;
    ///
    /// let name = ClassName::from_internal_name("java/lang/String");
    /// assert_eq!(name.binary_name(), "java.lang.String");
    /// ```
    pub fn from_internal_name(internal_name: &str) -> ClassName {
        let internal_name = internal_name.to_string();
        ClassName { internal_name, }
    }

    /// Creates a class name from a binary name.
    ///
    /// # Examples
    ///
    /// ```
    /// use jvm_obfuscation_mappings::descriptor::ClassName;
    ///
    /// let name = ClassName::from_binary_name("java.lang.String");
    /// assert_eq!(name.internal_name(), "java/lang/String");
    /// ```
    pub fn from_binary_name(binary_name: &str) -> ClassName {
        let internal_name = binary_name.replace('.', "/");
        ClassName { internal_name, }
    }

    /// Returns the internal name of this class name.
    ///
    /// # Examples
    ///
    /// ```
    /// use jvm_obfuscation_mappings::descriptor::ClassName;
    ///
    /// let name = ClassName::from_binary_name("java.lang.String");
    /// assert_eq!(name.internal_name(), "java/lang/String");
    /// ```
    pub fn internal_name(&self) -> &str {
        self.internal_name.as_str()
    }

    /// Returns the binary name of this class name.
    ///
    /// # Examples
    ///
    /// ```
    /// use jvm_obfuscation_mappings::descriptor::ClassName;
    ///
    /// let name = ClassName::from_internal_name("java/lang/String");
    /// assert_eq!(name.binary_name(), "java.lang.String");
    /// ```
    pub fn binary_name(&self) -> String {
        self.internal_name.replace('/', ".")
    }

    /// Returns a [`Type`] representing an object type with this class name.
    ///
    /// # Examples
    /// 
    /// ```
    /// use jvm_obfuscation_mappings::descriptor::{ClassName, Type};
    ///
    /// let name = ClassName::from_internal_name("java/lang/String");
    /// assert_eq!(name.to_type(), Type::Object(name));
    /// ```
    pub fn to_type(&self) -> Type {
        Type::Object(self.clone())
    }
}

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.internal_name)
    }
}
