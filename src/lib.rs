use walkdir::WalkDir;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use serde_json::json; // Make sure to include serde_json

pub fn count_files_and_lines(path: &str) -> HashMap<String, (usize, usize)> {
    let mut counts = HashMap::new(); // Extension -> (files count, lines count)

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| !e.path().to_string_lossy().contains("node_modules")) {
        
        if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
            let mut lines = 0;
            let file = File::open(entry.path());
            if let Ok(file) = file {
                lines = io::BufReader::new(file).lines().filter_map(Result::ok).count();
            }

            let counter = counts.entry(ext.to_string()).or_insert((0, 0));
            counter.0 += 1;
            counter.1 += lines;
        }
    }

    counts
}

pub fn save_results_to_json(file_counts: HashMap<String, (usize, usize)>) -> std::io::Result<()> {
    // Calculate total pages and total lines
    let total_pages = file_counts.values().map(|(file_count, _)| file_count).sum::<usize>();
    let total_lines = file_counts.values().map(|(_, line_count)| line_count).sum::<usize>();

    // Create a new structure to hold the summary and the details, with the updated terms
    let summary = json!({
        "pages": total_pages, // The total number of files, labeled as "pages"
        "lines": total_lines, // The total number of lines, still labeled as "lines"
        "details": file_counts.iter().map(|(key, &(files, lines))| {
            (key.clone(), json!({"files": files, "lines": lines})) // Change structure for each file type
        }).collect::<HashMap<String, serde_json::Value>>() // Convert to a suitable JSON structure
    });

    // Convert to a pretty JSON string
    let to_write = serde_json::to_string_pretty(&summary)?;

    // Write to a file
    let mut file = File::create("file_summary.json")?;
    file.write_all(to_write.as_bytes())?;

    Ok(())
}

