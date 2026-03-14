/// CLI entrypoint for the `mdbook-kanagawa-theme` preprocessor.
///
/// This binary is invoked by mdBook, reads the book context and contents
/// from stdin as JSON, runs the `KanagawaTheme` preprocessor, and writes
/// the transformed book back to stdout in the same JSON format.
use mdbook_kanagawa_theme::KanagawaTheme;
use mdbook_preprocessor::{MDBOOK_VERSION, Preprocessor, errors::Error, parse_input};
use semver::{Version, VersionReq};
use std::{env, io, process};

/// Main function for the preprocessor binary.
///
/// Responsibilities:
/// - Initialize logging.
/// - Handle user-facing flags like `--version` / `-V`.
/// - Implement the `supports` probe required by mdBook.
fn main() {
    env_logger::init();

    let pre = KanagawaTheme::new();
    let args: Vec<String> = env::args().collect();

    // Check for flags
    match args.get(1).map(String::as_str) {
        Some("--version" | "-V") => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        Some("supports") => {
            let renderer = args.get(2).map_or("html", |s| s.as_str());
            // Use the trait method instead of hardcoding "html" here
            if pre.supports_renderer(renderer).unwrap_or(false) {
                process::exit(0);
            }
            process::exit(1);
        }
        _ => {
            // Normal execution
            if let Err(e) = run(&pre) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
    }
}

/// Run the preprocessor using mdBook's JSON protocol.
///
/// This:
/// - Parses the `PreprocessorContext` and `Book` from stdin.
/// - Checks that the mdBook version calling us matches the version
///   range the plugin was built against, logging a warning if not.
/// - Invokes `pre.run(...)` to transform the book.
/// - Writes the processed book back to stdout as JSON using mdBook’s
///   stdin/stdout preprocessor protocol.
fn run(pre: &dyn Preprocessor) -> Result<(), Error> {
    // Read and deserialize the context + book from stdin using mdBook helpers.
    let (ctx, book) = parse_input(io::stdin())?;

    // Version Check
    if let (Ok(book_v), Ok(req_v)) = (
        Version::parse(&ctx.mdbook_version),
        VersionReq::parse(MDBOOK_VERSION),
    ) && !req_v.matches(&book_v)
    {
        log::warn!(
            "Plugin {} (built for mdbook {}) called by mdbook {}",
            pre.name(),
            MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    // Run the actual preprocessor implementation, transforming the book.
    let processed = pre.run(&ctx, book)?;

    // Serialize the processed book as JSON to stdout so mdBook can consume it.
    serde_json::to_writer(io::stdout(), &processed)?;
    Ok(())
}
