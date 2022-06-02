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

use std::collections::HashSet;
use crate::MappedElementKind;

/// The result of visiting mapping names, content or metadata.
pub type VisitResult<T> = anyhow::Result<T>;

/// Flags that describe the behaviour of a mapping visitor.
#[derive(PartialEq, Eq, Hash)]
pub enum MappingFlag {
    /// Indication that the visitor may require multiple passes.
    NeedsMultiplePasses,
    /// Requirement that metadata has to be provided in the header.
    NeedsHeaderMetadata,
    /// Requirement that an element has to be visited only once within a pass.
    ///
    /// This means that e.g. all members and properties of a class have to be visited after the same single
    /// `visit_class` invocation and no other `visit_class` invocation with the same `src_name` may occur.
    NeedsUniqueness,
    /// Requirement that source field descriptors have to be supplied.
    NeedsSrcFieldDesc,
    /// Requirement that source method descriptors have to be supplied.
    NeedsSrcMethodDesc,
    /// Requirement that destination field descriptors have to be supplied.
    NeedsDstFieldDesc,
    /// Requirement that destination method descriptors have to be supplied.
    NeedsDstMethodDesc
}

/// Visitor with order implied context and consecutive dst name visits.
///
/// The visitation order is as follows (omitting visit prefixes for brevity, lowercase for cross references):
/// - overall: `header -> content -> End -> overall`
/// - header: `Header -> Namespaces [-> Metadata]*`
/// - content: `Content [-> class|Metadata]*`
/// - class: `Class [-> DstName]* -> ElementContent [-> field|method|Comment]*`
/// - field: `Field [-> DstName|DstDesc]* -> ElementContent [-> Comment]`
/// - method: `Method [-> DstName|DstDesc]* -> ElementContent [-> arg|var|Comment]*`
/// - arg: `Arg [-> DstName]* -> ElementContent [-> Comment]`
/// - var: `Var [-> DstName]* -> ElementContent [-> Comment]`
///
///The elements with a skip-return (Header/Content/Class/Field/Method/Arg/Var/ElementContent) abort processing the
/// remainder of their associated item in the above listing if requested by a `true` return value. For example
/// skipping in Class does neither DstName nor ElementContent, but continues with another class or End.
///
/// Returning `false` in End requests another complete visitation pass if the flag
/// [`MappingFlag::NeedsMultiplePasses`] is provided, otherwise the behavior is unspecified. This is used for
/// visitors that first have to acquire some overall mapping knowledge before being able to perform their task.
/// Subsequent visitation passes need to use the same namespaces and data, only a new independent visitation may use
/// something else after a {@link #reset()}.
///
/// The same element may be visited more than once unless the flags contain
/// [`MappingFlag::NeedsUniqueness`].
pub trait MappingVisitor {
    /// Returns the flags describing this mapping visitor.
    fn flags(&self) -> HashSet<MappingFlag>;

    /// Reset the visitor including any chained visitors to allow for another independent visit (excluding visitEnd=false).
    fn reset(&mut self) {}

    /// Determine whether the header (namespaces, metadata if part of the header) should be visited.
    ///
    /// By default, returns `Ok(true)`.
    fn visit_header(&mut self) -> VisitResult<bool> {
        Ok(true)
    }

    /// Visit the list of namespaces.
    ///
    /// `dst_namespaces` may be empty, which indicates a single-namespace mapping file.
    fn visit_namespaces(&mut self, src_namespace: &str, dst_namespaces: &[&str]) -> VisitResult<()>;

    /// Visits a metadata property.
    ///
    /// Some mapping formats allow you to add a set of key-value properties
    /// which will be passed to this method.
    fn visit_metadata(&mut self, key: &str, value: &str) -> VisitResult<()> {
        Ok(())
    }

    /// Determine whether the mapping content (classes and anything below, metadata if not part of the header) should be visited.
    ///
    /// By default, returns `Ok(true)`.
    fn visit_content(&mut self) -> VisitResult<bool> {
        Ok(true)
    }

    /// Visits a class and its source name.
    ///
    /// The result describes whether the rest of the class (destination names, members and comments) should be read.
    fn visit_class(&mut self, src_name: &str) -> VisitResult<bool>;
    
    /// Visits a field and its source name and descriptor.
    ///
    /// The result describes whether the rest of the field (destination names and comments) should be read.
    fn visit_field(&mut self, src_name: &str, src_desc: Option<&str>) -> VisitResult<bool>;

    /// Visits a method and its source name and descriptor.
    ///
    /// The result describes whether the rest of the method (destination names, variables, arguments and comments) should be read.
    fn visit_method(&mut self, src_name: &str, src_desc: Option<&str>) -> VisitResult<bool>;

    /// Visits a method argument and its source name and indices.
    ///
    /// The result describes whether the rest of the argument (destination names and comments) should be read.
    fn visit_method_arg(&mut self, arg_position: i32, lv_index: i32, src_name: Option<&str>) -> VisitResult<bool>;

    /// Visits a method local variable and its source name and indices.
    ///
    /// The result describes whether the rest of the variable (destination names and comments) should be read.
    fn visit_method_var(&mut self, lvt_row_index: i32, lv_index: i32, start_op_idx: i32, src_name: Option<&str>) -> VisitResult<bool>;

    /// Finish the visitation pass.
    ///
    /// Returns true if the visitation pass is final, false if it should be started over.
    fn visit_end(&mut self) -> VisitResult<bool> {
        Ok(true)
    }

    /// Visits the destination name for the current element.
    ///
    /// `namespace` is the namespace index or index into the `dst_namespaces` list
    /// in [`Self::visit_namespaces`].
    fn visit_dst_name(&mut self, target_kind: MappedElementKind, namespace: usize, name: &str) -> VisitResult<()>;

    /// Visits the destination descriptor for the current element.
    ///
    /// `namespace` is the namespace index or index into the `dst_namespaces` list
    /// in [`Self::visit_namespaces`].
    fn visit_dst_desc(&mut self, target_kind: MappedElementKind, namespace: usize, desc: &str) -> VisitResult<()> {
        Ok(())
    }

    /// Determine whether the element content (comment, sub-elements) should be visited.
    ///
    /// Called after visiting the target itself (e.g. [`Self::visit_class`] for `target_kind`=[`Class`](MappedElementKind::Class)),
    /// its dst names and descs, but before any child elements or the comment.
    ///
    /// This is also a notification about all available dst names having been passed on.
    fn visit_element_content(&mut self, target_kind: MappedElementKind) -> VisitResult<bool>;

    /// Visits the comment for the specified element (last content-visited or any parent).
    /// The `comment` can potentially be a multi-line string.
    fn visit_comment(&mut self, target_kind: MappedElementKind, comment: &str) -> VisitResult<()>;
}
