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

#[derive(Debug)]
pub struct ClassName {
    internal_name: String,
}

impl ClassName {
    pub fn from_internal_name(internal_name: &str) -> ClassName {
        let internal_name = internal_name.to_string();
        ClassName { internal_name, }
    }

    pub fn from_binary_name(binary_name: &str) -> ClassName {
        let internal_name = binary_name.replace('.', "/");
        ClassName { internal_name, }
    }

    pub fn internal_name(&self) -> &str {
        self.internal_name.as_str()
    }

    pub fn binary_name(&self) -> String {
        self.internal_name.replace('/', ".")
    }
}

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.internal_name)
    }
}
