use mdbook_kanagawa_theme::KanagawaTheme;
use mdbook_preprocessor::{errors::Error, parse_input, Preprocessor, MDBOOK_VERSION};
use semver::{Version, VersionReq};
use std::io;
use std::process;

fn main() {
    env_logger::init();
    let pre = KanagawaTheme::new();

    let args: Vec<String> = std::env::args().collect();

    // Human-friendly version flag
    if args.get(1).map(|s| s.as_str()) == Some("--version")
        || args.get(1).map(|s| s.as_str()) == Some("-V")
    {
        println!("{}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    // mdBook protocol: `supports <renderer>`
    if args.get(1).map(|s| s.as_str()) == Some("supports") {
        let renderer = args.get(2).map(|s| s.as_str()).unwrap_or("html");
        if renderer == "html" {
            process::exit(0);
        } else {
            process::exit(1);
        }
    }

    if let Err(e) = run(&pre) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        log::warn!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed)?;
    Ok(())
}
