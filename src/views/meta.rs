//! Types and display logic for the file metadata view.
use std::io::Write;

use anyhow::Result;

use crate::styling::{
    names::{EXTRA_MARKER, FIELD_NAME},
    stylesheet::StyleSheet,
    text::{styled, StyledList},
};

/// Struct holding the human-readable metadata to disply for a field.
#[derive(Debug, Clone)]
pub struct FileMetaDisplay {
    pub headline: StyledList,
    pub fields: Vec<FileMetaField>,
}

/// A “field” in the file metadata.
#[derive(Debug, Clone)]
pub struct FileMetaField {
    /// The name of this field.
    pub name: String,
    /// The field's value.
    pub value: StyledList,
    /// Additional details, to be displayed as list items after the primary field value.
    pub extra: Vec<StyledList>,
}

impl FileMetaDisplay {
    pub fn render<W: Write>(&self, styles: &StyleSheet, mut out: W) -> Result<()> {
        writeln!(out, "{}", styles.render_list(&self.headline))?;
        for field in self.fields.iter() {
            writeln!(
                out,
                "{}: {}",
                styles.render(&styled(&field.name, FIELD_NAME)),
                styles.render_list(&field.value)
            )?;
            for line in field.extra.iter() {
                let marker = styled(" *  ", EXTRA_MARKER);
                writeln!(
                    out,
                    "{}{}",
                    styles.render(&marker),
                    styles.render_list(line)
                )?;
            }
        }
        Ok(())
    }
}
