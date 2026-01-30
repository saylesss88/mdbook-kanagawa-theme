/// CLI entrypoint for the `mdbook-kanagawa-theme` preprocessor.
///
/// This binary is invoked by mdBook, reads the book context and contents
/// from stdin as JSON, runs the `KanagawaTheme` preprocessor, and writes
/// the transformed book back to stdout in the same JSON format.
use mdbook_kanagawa_theme::KanagawaTheme;
use mdbook_preprocessor::{MDBOOK_VERSION, Preprocessor, errors::Error, parse_input};
use semver::{Version, VersionReq};
use std::io;
use std::process;
use std::string::String;

/// Main function for the preprocessor binary.
///
/// Responsibilities:
/// - Initialize logging.
/// - Handle user-facing flags like `--version` / `-V`.
/// - Implement the `supports` probe required by mdBook.
/// - Delegate actual preprocessing work to `run`.
fn main() {
    // Initialize env_logger so `log` macros in the preprocessor produce output
    // when RUST_LOG is set.
    env_logger::init();

    // Construct the concrete preprocessor implementation.
    let pre = KanagawaTheme::new();

    // Collect raw CLI arguments once for simple flag parsing.
    let args: Vec<String> = std::env::args().collect();

    // Human-friendly version flag:
    // `mdbook-kanagawa-theme --version` or `-V` prints the crate version and exits.
    if args.get(1).map(String::as_str) == Some("--version")
        || args.get(1).map(String::as_str) == Some("-V")
    {
        println!("{}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    // mdBook protocol: `preprocessor supports <renderer>`
    //
    // mdBook calls this to ask whether the preprocessor supports a given renderer.
    // Returning exit code 0 means "yes", non-zero means "no".
    if args.get(1).map(String::as_str) == Some("supports") {
        // Default to "html" if no renderer is provided.
        let renderer = args.get(2).map_or("html", String::as_str);
        if renderer == "html" {
            // Support the HTML renderer.
            process::exit(0);
        } else {
            // Unsupported renderer.
            process::exit(1);
        }
    }

    // Normal preprocessor mode: read from stdin, process, write to stdout.
    if let Err(e) = run(&pre) {
        // Print a human-readable error to stderr and exit with failure.
        eprintln!("{e}");
        process::exit(1);
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

    // Parse the mdBook version that invoked this preprocessor.
    let book_version = Version::parse(&ctx.mdbook_version)?;
    // Parse the version requirement this preprocessor declares compatibility with.
    let version_req = VersionReq::parse(MDBOOK_VERSION)?;

    // If the running mdBook does not satisfy our expected version range,
    // emit a warning but continue. This helps diagnose subtle incompatibilities.
    if !version_req.matches(&book_version) {
        log::warn!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
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
