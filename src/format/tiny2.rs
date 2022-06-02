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
use anyhow::anyhow;
use crate::MappedElementKind;
use crate::visitor::{MappingFlag, MappingVisitor, VisitResult};

// TODO: escape_names
/// A Tiny v2 writer as a [`MappingVisitor`] that outputs to [`std::fmt::Write`].
pub struct Tiny2Writer<W: std::fmt::Write> {
    write: W,
    dst_names: Vec<Option<String>>,
}

impl<W: std::fmt::Write> Tiny2Writer<W> {
    /// Creates a new Tiny v2 writer.
    pub fn new(write: W) -> Tiny2Writer<W> {
        Tiny2Writer { write, dst_names: Vec::new(), }
    }

    fn write_tab(&mut self) -> VisitResult<()> {
        self.write.write_char('\t')?;
        Ok(())
    }

    fn writeln(&mut self) -> VisitResult<()> {
        self.write.write_char('\n')?;
        Ok(())
    }
}

impl<W: std::fmt::Write> MappingVisitor for Tiny2Writer<W> {
    fn flags(&self) -> HashSet<MappingFlag> {
        HashSet::from(
            [MappingFlag::NeedsHeaderMetadata, MappingFlag::NeedsUniqueness, MappingFlag::NeedsSrcFieldDesc, MappingFlag::NeedsSrcMethodDesc]
        )
    }

    fn visit_namespaces(&mut self, src_namespace: &str, dst_namespaces: &[&str]) -> VisitResult<()> {
        self.dst_names = vec![None; dst_namespaces.len()];

        self.write.write_str("tiny\tv2\t0\t")?;
        self.write.write_str(src_namespace)?;

        for dst_namespace in dst_namespaces {
            self.write_tab()?;
            self.write.write_str(dst_namespace)?;
        }

        self.writeln()?;
        Ok(())
    }

    fn visit_class(&mut self, src_name: &str) -> VisitResult<bool> {
        self.write.write_str("c\t")?;
        self.write.write_str(src_name)?;
        Ok(true)
    }

    fn visit_field(&mut self, src_name: &str, src_desc: Option<&str>) -> VisitResult<bool> {
        self.write.write_str("\tf\t")?;
        self.write.write_str(src_desc.ok_or(anyhow!("Tiny2Writer needs src desc!"))?)?;
        self.write_tab()?;
        self.write.write_str(src_name)?;
        Ok(true)
    }

    fn visit_method(&mut self, src_name: &str, src_desc: Option<&str>) -> VisitResult<bool> {
        self.write.write_str("\tm\t")?;
        self.write.write_str(src_desc.ok_or(anyhow!("Tiny2Writer needs src desc!"))?)?;
        self.write_tab()?;
        self.write.write_str(src_name)?;
        Ok(true)
    }

    fn visit_method_arg(&mut self, _arg_position: i32, lv_index: i32, src_name: Option<&str>) -> VisitResult<bool> {
        self.write.write_str("\t\tp\t")?;
        write!(self.write, "{}", lv_index)?;
        self.write_tab()?;

        if let Some(src_name) = src_name {
            self.write.write_str(src_name)?;
        }

        Ok(true)
    }

    fn visit_method_var(&mut self, lvt_row_index: i32, lv_index: i32, start_op_idx: i32, src_name: Option<&str>) -> VisitResult<bool> {
        self.write.write_str("\t\tv\t")?;
        write!(self.write, "{}", lv_index)?;
        self.write_tab()?;
        write!(self.write, "{}", start_op_idx)?;
        self.write_tab()?;
        write!(self.write, "{}", i32::max(lvt_row_index, -1))?;
        self.write_tab()?;

        if let Some(src_name) = src_name {
            self.write.write_str(src_name)?;
        }

        Ok(true)
    }

    fn visit_dst_name(&mut self, _target_kind: MappedElementKind, namespace: usize, name: &str) -> VisitResult<()> {
        self.dst_names[namespace] = Some(name.to_owned());
        Ok(())
    }

    fn visit_element_content(&mut self, _target_kind: MappedElementKind) -> VisitResult<bool> {
        for dst_name in &self.dst_names.clone() {
            self.write_tab()?;

            if let Some(dst_name) = dst_name {
                self.write.write_str(dst_name)?;
            }
        }

        self.dst_names.fill(None);
        Ok(true)
    }

    fn visit_comment(&mut self, target_kind: MappedElementKind, comment: &str) -> VisitResult<()> {
        for _ in 0..target_kind.level() {
            self.write_tab()?;
        }

        self.write.write_str("\tc\t")?;
        // TODO: espace newlines etc
        self.write.write_str(comment)?;
        self.writeln()?;
        Ok(())
    }
}
