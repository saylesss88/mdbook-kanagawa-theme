use crate::KanagawaTheme;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext, book::Book, errors::Error};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CardLayout {
    #[default]
    Compact,
    Wide,
}

#[derive(Debug, Deserialize)]
#[serde(default)] // This allows us to omit the [preprocessor.kanagawa-theme] table entirely
pub struct KanagawaConfig {
    #[serde(default = "default_title")]
    pub landing_title: String,
    #[serde(default = "default_subtitle")]
    pub landing_subtitle: String,

    #[serde(default = "default_latest")]
    pub header_latest: String,
    #[serde(default = "default_notes")]
    pub header_notes: String,
    #[serde(default = "default_tags")]
    pub header_tags: String,

    pub card_layout: CardLayout, // Already has #[default] on the Enum

    // Fields that represent "On/Off" or "Something/Nothing" should stay as Option/bool
    pub css_import: Option<String>,
    pub disable_builtin_css: bool, // Default is false

    pub code_css_import: Option<String>,
    pub disable_builtin_code_css: bool,

    pub support_footer: bool,
    pub support_footer_href: Option<String>,
    #[serde(default = "default_footer_text")]
    pub support_footer_text: String,
}

// --- Helper functions for Serde defaults ---
fn default_title() -> String {
    "mdTheme".into()
}
fn default_subtitle() -> String {
    "A dope landing powered by rust".into()
}
fn default_latest() -> String {
    "Latest Posts".into()
}
fn default_notes() -> String {
    "Recent Notes".into()
}
fn default_tags() -> String {
    "Popular Tags".into()
}
fn default_footer_text() -> String {
    "Made with mdbook-kanagawa-theme".into()
}

// --- Implement Default manually to use these same helpers ---
impl Default for KanagawaConfig {
    fn default() -> Self {
        Self {
            landing_title: default_title(),
            landing_subtitle: default_subtitle(),
            header_latest: default_latest(),
            header_notes: default_notes(),
            header_tags: default_tags(),
            card_layout: CardLayout::default(),
            css_import: None,
            disable_builtin_css: false,
            code_css_import: None,
            disable_builtin_code_css: false,
            support_footer: false,
            support_footer_href: None,
            support_footer_text: default_footer_text(),
        }
    }
}

impl Preprocessor for KanagawaTheme {
    /// Returns the preprocessor name as used in `book.toml`
    /// under `[preprocessor.kanagawa-theme]`.
    fn name(&self) -> &'static str {
        "kanagawa-theme"
    }

    /// Apply the Kanagawa theme:
    /// * read configuration from the `PreprocessorContext`,
    /// * replace `index.md` with a dynamic landing page, and
    /// * optionally write `theme/css/chrome.css` and `theme/css/kanagawa-code.css`.
    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let cfg = Self::read_config(ctx);
        Self::inject_landing_page(&mut book, &cfg);
        Self::write_chrome_css(ctx, &cfg);
        Self::write_code_css(ctx, &cfg);
        Self::add_support_footer(&mut book, &cfg);
        Ok(book)
    }

    /// Only support the HTML renderer, as the theme CSS and landing page
    /// are specific to HTML output.
    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}
