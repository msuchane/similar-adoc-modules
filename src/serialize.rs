use std::fs;
use std::path::Path;

use color_eyre::Result;
use serde::Serialize;

use crate::Cli;
use crate::Comparison;

/// A record of a file comparison, formatted to be serialized for the user.
#[derive(Serialize)]
struct OutputComparison {
    pct_similar: f64,
    file1: String,
    file2: String,
}

impl OutputComparison {
    /// Convert from the internal `Comparison` format to the serializable `OutputComparison` format.
    fn from_internal(comparison: Comparison, options: &Cli) -> Result<Self> {
        Ok(Self {
            pct_similar: comparison.similarity_pct.0,
            file1: stripped_path(comparison.path1, options)?,
            file2: stripped_path(comparison.path1, options)?,
        })
    }
}

/// Serialize the resulting comparisons as a structured file.
pub fn serialize(mut comparisons: Vec<Comparison>, options: &Cli) -> Result<()> {
    // Sort from highest to lowest. You can't sort f64 values, so convert them to u32
    // with a precision of percentage with a single decimal place, then subtract from 1000.
    comparisons
        .sort_by_key(|comparison| 1000 - (comparison.similarity_pct.0 * 10.0).round() as i32);

    let output_comparisons: Vec<OutputComparison> = comparisons
        .into_iter()
        .map(|comparison| OutputComparison::from_internal(comparison, options))
        .collect::<Result<Vec<_>>>()?;

    as_csv(&output_comparisons, options)?;
    as_json(&output_comparisons, options)?;

    Ok(())
}

/// Serialize and save the comparisons as a CSV file.
fn as_csv(comparisons: &[OutputComparison], options: &Cli) -> Result<()> {
    // Prepare to write to the CSV file.
    let mut wtr = csv::Writer::from_path(&options.csv)?;

    // The CSV header:
    wtr.write_record(&["% similar", "File 1", "File 2"])?;

    // Each comparison entry writes a row in the CSV table.
    for comparison in comparisons {
        wtr.write_record(&[
            &format!("{:.1}", comparison.pct_similar),
            &comparison.file1,
            &comparison.file2,
        ])?;
    }

    // Flush the CSV writer buffer.
    wtr.flush()?;

    Ok(())
}

/// Serialize and save the comparisons as a pretty-formatted JSON file.
fn as_json(comparisons: &[OutputComparison], options: &Cli) -> Result<()> {
    let json_text = serde_json::to_string_pretty(comparisons)?;

    fs::write(&options.json, json_text)?;

    Ok(())
}

/// Present the file path without the common, shared prefix
/// of the root comparison directory.
fn stripped_path(path: &Path, options: &Cli) -> Result<String> {
    let stripped = path.strip_prefix(&options.path)?;

    Ok(stripped.display().to_string())
}
