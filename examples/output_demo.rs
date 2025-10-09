//! Demonstration of output formatting capabilities
//!
//! Run with:
//!   cargo run --example output_demo --features cli
//!   cargo run --example output_demo --features cli -- --format json
//!   cargo run --example output_demo --features cli -- --format compact
//!   cargo run --example output_demo --features cli -- --verbose
//!   cargo run --example output_demo --features cli -- --quiet

use octofhir_codegen::cli::{
    OutputFormat, OutputFormatter, ProgressReporter, Verbosity, create_progress_bar,
    create_spinner,
};
use std::thread;
use std::time::Duration;

fn main() {
    // Parse simple command-line args
    let args: Vec<String> = std::env::args().collect();

    let format = if args.contains(&"--format".to_string()) {
        let idx = args.iter().position(|a| a == "--format").unwrap();
        args.get(idx + 1)
            .and_then(|f| f.parse::<OutputFormat>().ok())
            .unwrap_or(OutputFormat::Pretty)
    } else {
        OutputFormat::Pretty
    };

    let verbosity = if args.contains(&"--quiet".to_string()) {
        Verbosity::Quiet
    } else if args.contains(&"--very-verbose".to_string()) || args.contains(&"-vv".to_string()) {
        Verbosity::VeryVerbose
    } else if args.contains(&"--verbose".to_string()) || args.contains(&"-v".to_string()) {
        Verbosity::Verbose
    } else {
        Verbosity::Normal
    };

    let use_color = !args.contains(&"--no-color".to_string());

    let formatter = OutputFormatter::with_verbosity(use_color, format, verbosity);

    // Demonstrate various output methods
    formatter.header("Output Formatting Demo");

    formatter.info("This demonstrates the various output formatting capabilities");
    formatter.separator();

    // Basic messages
    formatter.success("This is a success message");
    formatter.error("This is an error message");
    formatter.warning("This is a warning message");
    formatter.info("This is an info message");

    // Verbose output
    formatter.verbose("This is a verbose message (only shown with --verbose)");
    formatter.debug("This is a debug message (only shown with --very-verbose or -vv)");

    formatter.separator();

    // Key-value pairs
    formatter.header("Key-Value Pairs");
    formatter.key_value("Version", "1.0.0");
    formatter.key_value("Author", "OctoFHIR Team");
    formatter.key_value("License", "MIT OR Apache-2.0");

    formatter.separator();

    // List items
    formatter.header("Features List");
    formatter.list_item("Colored output with colored crate");
    formatter.list_item("Progress bars with indicatif");
    formatter.list_item("JSON output mode for CI");
    formatter.list_item("Verbose/quiet modes");
    formatter.list_item("User-friendly output");

    formatter.separator();

    // Table
    formatter.header("Generators Table");
    formatter.table(
        &["Generator", "Status", "Priority"],
        &[
            vec!["TypeScript".to_string(), "Ready".to_string(), "High".to_string()],
            vec!["Rust".to_string(), "In Progress".to_string(), "High".to_string()],
            vec!["Python".to_string(), "Planned".to_string(), "Medium".to_string()],
            vec!["Java".to_string(), "Planned".to_string(), "Low".to_string()],
        ],
    );

    formatter.separator();

    // Steps
    formatter.header("Multi-Step Process");
    formatter.step(1, 3, "Parsing configuration");
    thread::sleep(Duration::from_millis(500));
    formatter.step(2, 3, "Loading FHIR packages");
    thread::sleep(Duration::from_millis(500));
    formatter.step(3, 3, "Generating code");

    formatter.separator();

    // Progress bar (only in interactive mode)
    if !formatter.is_json() && !formatter.is_quiet() {
        formatter.header("Progress Bar Demo");
        let pb = create_progress_bar(100, "Processing");
        for _ in 0..100 {
            pb.inc(1);
            thread::sleep(Duration::from_millis(20));
        }
        pb.finish_with_message("Processing complete!");

        formatter.separator();

        // Spinner demo
        formatter.header("Spinner Demo");
        let spinner = create_spinner("Loading packages...");
        thread::sleep(Duration::from_secs(2));
        spinner.finish_with_message("✓ Packages loaded");
    } else {
        // For non-interactive mode, use ProgressReporter
        formatter.header("Progress Reporting");
        let mut reporter = ProgressReporter::new(format, 100, "Processing");
        for i in 0..=100 {
            if i % 10 == 0 {
                reporter.set(i);
            }
            thread::sleep(Duration::from_millis(10));
        }
        reporter.finish("Processing complete");
    }

    formatter.separator();
    formatter.success("Demo complete!");
}
