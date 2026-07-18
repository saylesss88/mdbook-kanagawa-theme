pub mod config;
pub mod css;
pub mod landing;

use crate::config::KanagawaConfig;

use mdbook_preprocessor::{
    PreprocessorContext,
    book::{Book, BookItem},
};
use std::fs;

/// mdBook preprocessor that injects a Kanagawa-themed landing page
/// and wires Kanagawa CSS into the generated HTML output.
pub struct KanagawaTheme;

impl Default for KanagawaTheme {
    /// Construct a `KanagawaTheme` using the default constructor.
    fn default() -> Self {
        Self::new()
    }
}

impl KanagawaTheme {
    /// Create a new `KanagawaTheme` preprocessor with no internal state.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn read_config(ctx: &PreprocessorContext) -> KanagawaConfig {
        ctx.config
            .get::<KanagawaConfig>("preprocessor.kanagawa-theme")
            .unwrap_or_else(|e| {
                eprintln!("kanagawa-theme: [Error] Failed to parse config, using defaults: {e}");
                None
            })
            .unwrap_or_default()
    }

    fn inject_landing_page(book: &mut Book, cfg: &KanagawaConfig) {
        let mut injected = false;
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ch) = item
                && ch.path.as_ref().and_then(|p| p.file_stem()) == Some("index".as_ref())
                && !injected
            {
                ch.content = landing::build_landing_page(cfg);
                injected = true;
            }
        });
    }

    fn write_css_file(ctx: &PreprocessorContext, filename: &str, css: String) {
        let css_dir = ctx.root.join("theme").join("css");
        if let Err(e) = fs::create_dir_all(&css_dir) {
            eprintln!("\x1b[31mkanagawa-theme: failed to create theme/css dir\x1b[0m: {e}");
            return;
        }
        if let Err(e) = fs::write(css_dir.join(filename), css) {
            eprintln!("\x1b[31mkanagawa-theme: failed to write theme/css/{filename}\x1b[0m: {e}");
        }
    }
    fn write_chrome_css(ctx: &PreprocessorContext, cfg: &KanagawaConfig) {
        if cfg.disable_builtin_css {
            return;
        }
        Self::write_css_file(ctx, "chrome.css", css::build_full_chrome_css(cfg));
    }

    fn write_code_css(ctx: &PreprocessorContext, cfg: &KanagawaConfig) {
        if cfg.disable_builtin_code_css {
            return;
        }
        Self::write_css_file(ctx, "kanagawa-code.css", css::build_code_css(cfg));
    }

    fn add_support_footer(book: &mut Book, cfg: &KanagawaConfig) {
        if cfg.support_footer {
            let href = cfg
                .support_footer_href
                .as_deref()
                .unwrap_or("https://github.com/saylesss88/mdbook-kanagawa-theme");

            let footer_html = format!(
                r#"<footer id="kanagawa-support-footer" style="text-align:center; margin-top: 3rem; font-size: 0.85em; opacity: 0.75;"><p><a href="{href}">{text}</a></p></footer>"#,
                href = href,
                text = cfg.support_footer_text // Reference instead of clone
            );
            book.for_each_mut(|item| {
                if let BookItem::Chapter(ch) = item
                    && !ch.content.contains(r#"id="kanagawa-support-footer""#)
                {
                    ch.content.push_str(&footer_html);
                }
            });
        }
    }
}
