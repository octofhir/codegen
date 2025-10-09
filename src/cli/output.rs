//! CLI output formatting and display utilities
//!
//! This module provides utilities for formatting CLI output including
//! colored text, progress indicators, and structured data display.

use colored::*;
use std::fmt;

/// Verbosity level for output
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    /// Minimal output (errors only)
    Quiet,
    /// Normal output (default)
    Normal,
    /// Detailed output with additional information
    Verbose,
    /// Very detailed output with debug information
    VeryVerbose,
}

/// Output formatter for CLI
pub struct OutputFormatter {
    use_color: bool,
    format: OutputFormat,
    verbosity: Verbosity,
}

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Pretty formatted output with colors
    Pretty,
    /// JSON output
    Json,
    /// Compact single-line output
    Compact,
}

impl OutputFormatter {
    /// Create a new output formatter
    pub fn new(use_color: bool, format: OutputFormat) -> Self {
        Self { use_color, format, verbosity: Verbosity::Normal }
    }

    /// Create a new output formatter with verbosity
    pub fn with_verbosity(use_color: bool, format: OutputFormat, verbosity: Verbosity) -> Self {
        Self { use_color, format, verbosity }
    }

    /// Create a pretty formatter
    pub fn pretty(use_color: bool) -> Self {
        Self::new(use_color, OutputFormat::Pretty)
    }

    /// Create a JSON formatter
    pub fn json() -> Self {
        Self::new(false, OutputFormat::Json)
    }

    /// Set verbosity level
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
    }

    /// Get current verbosity level
    pub fn verbosity(&self) -> Verbosity {
        self.verbosity
    }

    /// Check if output should be shown at this verbosity level
    fn should_show(&self, min_verbosity: Verbosity) -> bool {
        self.verbosity >= min_verbosity
    }

    /// Print a success message
    pub fn success(&self, message: &str) {
        match self.format {
            OutputFormat::Pretty => {
                if self.use_color {
                    println!("{} {}", "✓".green().bold(), message);
                } else {
                    println!("✓ {}", message);
                }
            }
            OutputFormat::Json => {
                println!(r#"{{"level":"success","message":"{}"}}"#, message.replace('"', "\\\""));
            }
            OutputFormat::Compact => {
                println!("SUCCESS: {}", message);
            }
        }
    }

    /// Print an error message
    pub fn error(&self, message: &str) {
        match self.format {
            OutputFormat::Pretty => {
                if self.use_color {
                    eprintln!("{} {}", "✗".red().bold(), message.red());
                } else {
                    eprintln!("✗ {}", message);
                }
            }
            OutputFormat::Json => {
                eprintln!(r#"{{"level":"error","message":"{}"}}"#, message.replace('"', "\\\""));
            }
            OutputFormat::Compact => {
                eprintln!("ERROR: {}", message);
            }
        }
    }

    /// Print a warning message
    pub fn warning(&self, message: &str) {
        match self.format {
            OutputFormat::Pretty => {
                if self.use_color {
                    println!("{} {}", "⚠".yellow().bold(), message.yellow());
                } else {
                    println!("⚠ {}", message);
                }
            }
            OutputFormat::Json => {
                println!(r#"{{"level":"warning","message":"{}"}}"#, message.replace('"', "\\\""));
            }
            OutputFormat::Compact => {
                println!("WARNING: {}", message);
            }
        }
    }

    /// Print an info message
    pub fn info(&self, message: &str) {
        if !self.should_show(Verbosity::Normal) {
            return;
        }
        match self.format {
            OutputFormat::Pretty => {
                if self.use_color {
                    println!("{} {}", "ℹ".cyan().bold(), message);
                } else {
                    println!("ℹ {}", message);
                }
            }
            OutputFormat::Json => {
                println!(r#"{{"level":"info","message":"{}"}}"#, message.replace('"', "\\\""));
            }
            OutputFormat::Compact => {
                println!("INFO: {}", message);
            }
        }
    }

    /// Print a verbose message (only shown in verbose mode)
    pub fn verbose(&self, message: &str) {
        if !self.should_show(Verbosity::Verbose) {
            return;
        }
        match self.format {
            OutputFormat::Pretty => {
                if self.use_color {
                    println!("{} {}", "→".blue(), message.dimmed());
                } else {
                    println!("→ {}", message);
                }
            }
            OutputFormat::Json => {
                println!(r#"{{"level":"verbose","message":"{}"}}"#, message.replace('"', "\\\""));
            }
            OutputFormat::Compact => {
                println!("VERBOSE: {}", message);
            }
        }
    }

    /// Print a debug message (only shown in very verbose mode)
    pub fn debug(&self, message: &str) {
        if !self.should_show(Verbosity::VeryVerbose) {
            return;
        }
        match self.format {
            OutputFormat::Pretty => {
                if self.use_color {
                    println!("{} {}", "🔍".dimmed(), message.dimmed());
                } else {
                    println!("DEBUG: {}", message);
                }
            }
            OutputFormat::Json => {
                println!(r#"{{"level":"debug","message":"{}"}}"#, message.replace('"', "\\\""));
            }
            OutputFormat::Compact => {
                println!("DEBUG: {}", message);
            }
        }
    }

    /// Print a section header
    pub fn header(&self, title: &str) {
        match self.format {
            OutputFormat::Pretty => {
                if self.use_color {
                    println!("\n{}", title.bold().underline());
                } else {
                    println!("\n{}", title);
                    println!("{}", "=".repeat(title.len()));
                }
            }
            OutputFormat::Json => {
                // JSON mode doesn't print headers
            }
            OutputFormat::Compact => {
                println!("=== {} ===", title);
            }
        }
    }

    /// Print a key-value pair
    pub fn key_value(&self, key: &str, value: &str) {
        match self.format {
            OutputFormat::Pretty => {
                if self.use_color {
                    println!("  {}: {}", key.cyan(), value);
                } else {
                    println!("  {}: {}", key, value);
                }
            }
            OutputFormat::Json => {
                println!(
                    r#"{{"key":"{}","value":"{}"}}"#,
                    key.replace('"', "\\\""),
                    value.replace('"', "\\\"")
                );
            }
            OutputFormat::Compact => {
                println!("{}={}", key, value);
            }
        }
    }

    /// Print a list item
    pub fn list_item(&self, item: &str) {
        match self.format {
            OutputFormat::Pretty => {
                println!("  • {}", item);
            }
            OutputFormat::Json => {
                println!(r#"{{"item":"{}"}}"#, item.replace('"', "\\\""));
            }
            OutputFormat::Compact => {
                println!("- {}", item);
            }
        }
    }

    /// Print a table
    pub fn table(&self, headers: &[&str], rows: &[Vec<String>]) {
        match self.format {
            OutputFormat::Pretty => {
                // Calculate column widths
                let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
                for row in rows {
                    for (i, cell) in row.iter().enumerate() {
                        if i < widths.len() {
                            widths[i] = widths[i].max(cell.len());
                        }
                    }
                }

                // Print header
                let header_line: Vec<String> = headers
                    .iter()
                    .enumerate()
                    .map(|(i, h)| format!("{:width$}", h, width = widths[i]))
                    .collect();

                if self.use_color {
                    println!("{}", header_line.join("  ").bold());
                } else {
                    println!("{}", header_line.join("  "));
                }

                // Print separator
                let separator: Vec<String> = widths.iter().map(|w| "─".repeat(*w)).collect();
                println!("{}", separator.join("  "));

                // Print rows
                for row in rows {
                    let row_line: Vec<String> = row
                        .iter()
                        .enumerate()
                        .map(|(i, cell)| {
                            format!("{:width$}", cell, width = widths.get(i).copied().unwrap_or(0))
                        })
                        .collect();
                    println!("{}", row_line.join("  "));
                }
            }
            OutputFormat::Json => {
                let json_rows: Vec<String> = rows
                    .iter()
                    .map(|row| {
                        let pairs: Vec<String> = headers
                            .iter()
                            .zip(row.iter())
                            .map(|(k, v)| {
                                format!(
                                    r#""{}":"{}""#,
                                    k.replace('"', "\\\""),
                                    v.replace('"', "\\\"")
                                )
                            })
                            .collect();
                        format!("{{{}}}", pairs.join(","))
                    })
                    .collect();
                println!("[{}]", json_rows.join(","));
            }
            OutputFormat::Compact => {
                println!("{}", headers.join("\t"));
                for row in rows {
                    println!("{}", row.join("\t"));
                }
            }
        }
    }

    /// Print JSON data
    pub fn print_json(&self, value: &serde_json::Value) {
        match self.format {
            OutputFormat::Pretty => {
                if let Ok(pretty) = serde_json::to_string_pretty(value) {
                    println!("{}", pretty);
                }
            }
            OutputFormat::Json => {
                if let Ok(compact) = serde_json::to_string(value) {
                    println!("{}", compact);
                }
            }
            OutputFormat::Compact => {
                if let Ok(compact) = serde_json::to_string(value) {
                    println!("{}", compact);
                }
            }
        }
    }

    /// Print a step in a process
    pub fn step(&self, current: usize, total: usize, message: &str) {
        if !self.should_show(Verbosity::Normal) {
            return;
        }
        match self.format {
            OutputFormat::Pretty => {
                if self.use_color {
                    println!("{} [{}/{}] {}", "▶".green(), current, total, message);
                } else {
                    println!("[{}/{}] {}", current, total, message);
                }
            }
            OutputFormat::Json => {
                println!(
                    r#"{{"level":"step","current":{},"total":{},"message":"{}"}}"#,
                    current,
                    total,
                    message.replace('"', "\\\"")
                );
            }
            OutputFormat::Compact => {
                println!("[{}/{}] {}", current, total, message);
            }
        }
    }

    /// Print a section separator
    pub fn separator(&self) {
        if !self.should_show(Verbosity::Normal) {
            return;
        }
        if self.format == OutputFormat::Pretty {
            println!();
        }
    }

    /// Check if the formatter is in quiet mode
    pub fn is_quiet(&self) -> bool {
        self.verbosity == Verbosity::Quiet
    }

    /// Check if the formatter is in verbose mode
    pub fn is_verbose(&self) -> bool {
        self.verbosity >= Verbosity::Verbose
    }

    /// Check if JSON output is enabled
    pub fn is_json(&self) -> bool {
        self.format == OutputFormat::Json
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        use std::io::IsTerminal;
        Self::pretty(std::io::stdout().is_terminal())
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Pretty => write!(f, "pretty"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Compact => write!(f, "compact"),
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pretty" => Ok(OutputFormat::Pretty),
            "json" => Ok(OutputFormat::Json),
            "compact" => Ok(OutputFormat::Compact),
            _ => Err(format!("Invalid format: '{}'. Valid formats: pretty, json, compact", s)),
        }
    }
}

/// Create a progress bar for long-running operations
pub fn create_progress_bar(len: u64, message: &str) -> indicatif::ProgressBar {
    let pb = indicatif::ProgressBar::new(len);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
            .expect("Failed to create progress style")
            .progress_chars("#>-"),
    );
    pb.set_message(message.to_string());
    pb
}

/// Create a spinner for indeterminate operations
pub fn create_spinner(message: &str) -> indicatif::ProgressBar {
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .expect("Failed to create spinner style"),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner
}

/// Create a progress bar with ETA and speed
pub fn create_detailed_progress_bar(len: u64, message: &str) -> indicatif::ProgressBar {
    let pb = indicatif::ProgressBar::new(len);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) ETA: {eta}")
            .expect("Failed to create progress style")
            .progress_chars("█▓▒░ "),
    );
    pb.set_message(message.to_string());
    pb
}

/// Create a multi-progress container for managing multiple progress bars
pub fn create_multi_progress() -> indicatif::MultiProgress {
    indicatif::MultiProgress::new()
}

/// Progress reporter that can work with different output modes
pub struct ProgressReporter {
    format: OutputFormat,
    total: u64,
    current: u64,
    message: String,
}

impl ProgressReporter {
    /// Create a new progress reporter
    pub fn new(format: OutputFormat, total: u64, message: impl Into<String>) -> Self {
        Self {
            format,
            total,
            current: 0,
            message: message.into(),
        }
    }

    /// Update progress
    pub fn inc(&mut self, delta: u64) {
        self.current += delta;
        self.report();
    }

    /// Set progress to a specific value
    pub fn set(&mut self, value: u64) {
        self.current = value;
        self.report();
    }

    /// Report progress based on format
    fn report(&self) {
        match self.format {
            OutputFormat::Pretty => {
                // For pretty format, we'd typically use a real progress bar
                // This is just for non-interactive reporting
                if self.current == self.total {
                    println!("✓ {}: Complete", self.message);
                }
            }
            OutputFormat::Json => {
                println!(
                    r#"{{"type":"progress","message":"{}","current":{},"total":{}}}"#,
                    self.message.replace('"', "\\\""),
                    self.current,
                    self.total
                );
            }
            OutputFormat::Compact => {
                if self.current == self.total {
                    println!("{}: {}/{}", self.message, self.current, self.total);
                }
            }
        }
    }

    /// Finish progress reporting
    pub fn finish(&self, message: impl Into<String>) {
        let msg = message.into();
        match self.format {
            OutputFormat::Pretty => {
                println!("✓ {}", msg);
            }
            OutputFormat::Json => {
                println!(r#"{{"type":"progress","status":"complete","message":"{}"}}"#, msg.replace('"', "\\\""));
            }
            OutputFormat::Compact => {
                println!("DONE: {}", msg);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_creation() {
        let formatter = OutputFormatter::pretty(false);
        assert_eq!(formatter.format, OutputFormat::Pretty);
        assert!(!formatter.use_color);
    }

    #[test]
    fn test_json_formatter() {
        let formatter = OutputFormatter::json();
        assert_eq!(formatter.format, OutputFormat::Json);
        assert!(!formatter.use_color);
    }

    #[test]
    fn test_output_format_from_str() {
        assert_eq!("pretty".parse::<OutputFormat>().unwrap(), OutputFormat::Pretty);
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("compact".parse::<OutputFormat>().unwrap(), OutputFormat::Compact);
        assert!("invalid".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn test_output_format_display() {
        assert_eq!(OutputFormat::Pretty.to_string(), "pretty");
        assert_eq!(OutputFormat::Json.to_string(), "json");
        assert_eq!(OutputFormat::Compact.to_string(), "compact");
    }

    #[test]
    fn test_default_formatter() {
        let formatter = OutputFormatter::default();
        assert_eq!(formatter.format, OutputFormat::Pretty);
    }

    #[test]
    fn test_verbosity_levels() {
        let formatter = OutputFormatter::with_verbosity(false, OutputFormat::Pretty, Verbosity::Quiet);
        assert_eq!(formatter.verbosity(), Verbosity::Quiet);
        assert!(formatter.is_quiet());
        assert!(!formatter.is_verbose());

        let formatter = OutputFormatter::with_verbosity(false, OutputFormat::Pretty, Verbosity::Verbose);
        assert_eq!(formatter.verbosity(), Verbosity::Verbose);
        assert!(!formatter.is_quiet());
        assert!(formatter.is_verbose());
    }

    #[test]
    fn test_verbosity_ordering() {
        assert!(Verbosity::Quiet < Verbosity::Normal);
        assert!(Verbosity::Normal < Verbosity::Verbose);
        assert!(Verbosity::Verbose < Verbosity::VeryVerbose);
    }

    #[test]
    fn test_set_verbosity() {
        let mut formatter = OutputFormatter::pretty(false);
        assert_eq!(formatter.verbosity(), Verbosity::Normal);

        formatter.set_verbosity(Verbosity::Verbose);
        assert_eq!(formatter.verbosity(), Verbosity::Verbose);
    }

    #[test]
    fn test_is_json() {
        let formatter = OutputFormatter::json();
        assert!(formatter.is_json());

        let formatter = OutputFormatter::pretty(false);
        assert!(!formatter.is_json());
    }

    #[test]
    fn test_progress_reporter() {
        let mut reporter = ProgressReporter::new(OutputFormat::Compact, 100, "Test");
        assert_eq!(reporter.current, 0);
        assert_eq!(reporter.total, 100);

        reporter.inc(10);
        assert_eq!(reporter.current, 10);

        reporter.set(50);
        assert_eq!(reporter.current, 50);
    }
}
