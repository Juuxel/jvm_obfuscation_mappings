/*
 * Copyright (c) 2021-2022 FabricMC, 2022 Juuz
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

pub mod descriptor;
pub mod format;
pub mod visitor;

pub enum MappedElementKind {
    Class,
    Field,
    Method,
    MethodArg,
    MethodVar
}

impl MappedElementKind {
    /// Returns the nesting level of this element kind.
    /// Classes are 0, members are 1 and attributes of members are 2.
    fn level(&self) -> u32 {
        match self {
            MappedElementKind::Class => 0,
            MappedElementKind::Field => 1,
            MappedElementKind::Method => 1,
            MappedElementKind::MethodArg => 2,
            MappedElementKind::MethodVar => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
