pub mod config;

use crate::config::{CardLayout, KanagawaConfig};

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
                ch.content = build_landing_page(cfg);
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
        Self::write_css_file(ctx, "chrome.css", build_full_chrome_css(cfg));
    }

    fn write_code_css(ctx: &PreprocessorContext, cfg: &KanagawaConfig) {
        if cfg.disable_builtin_code_css {
            return;
        }
        Self::write_css_file(ctx, "kanagawa-code.css", build_code_css(cfg));
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

/// Build the HTML source for the Kanagawa landing page by
/// filling `LANDING_PAGE_TEMPLATE` with configured titles, headers,
fn build_landing_page(cfg: &KanagawaConfig) -> String {
    let grid_class = match cfg.card_layout {
        CardLayout::Wide => "grid grid-wide",
        CardLayout::Compact => "grid",
    };

    let mut html = LANDING_PAGE_TEMPLATE.to_owned();

    // Core replacements
    html = html.replace("{{LANDING_TITLE}}", &cfg.landing_title);
    html = html.replace("{{LANDING_SUBTITLE}}", &cfg.landing_subtitle);

    // Header replacements - This is what was missing!
    html = html.replace("{{HEADER_LATEST}}", &cfg.header_latest);
    html = html.replace("{{HEADER_NOTES}}", &cfg.header_notes);
    html = html.replace("{{HEADER_TAGS}}", &cfg.header_tags);

    // Layout replacement
    html = html.replace("{{GRID_CLASS}}", grid_class);

    html
}

/// Build a complete `chrome.css` by:
/// 1. optionally inserting a user-provided `@import`,
/// 2. appending Kanagawa CSS variables,
/// 3. including the mdBook chrome template, and
/// 4. layering additional Kanagawa styles on top.
fn build_full_chrome_css(cfg: &KanagawaConfig) -> String {
    let base = include_str!("kanagawa_chrome_template.css");

    let mut out = String::new();

    // 1) Optional user CSS import (must be first)
    if let Some(path) = cfg.css_import.as_deref() {
        // path should be "theme/dracula.css" from book.toml
        out.push_str("@import url(\"");
        out.push_str(path);
        out.push_str("\");\n\n");
    }

    // 2) Kanagawa variables for each theme class
    out.push_str(KANAGAWA_VARS);
    out.push_str("\n\n");

    // 3) mdBook's stock chrome.css template
    out.push_str(base);
    out.push_str("\n\n");

    // 4) Extra Kanagawa styles layered on top
    out.push_str(KANAGAWA_EXTRA_CSS);
    out.push('\n');

    out
}

/// Build the Kanagawa code syntax CSS (for highlight.js).
fn build_code_css(cfg: &KanagawaConfig) -> String {
    let mut out = String::new();

    // Optional user import at the very top.
    if let Some(path) = cfg.code_css_import.as_deref() {
        out.push_str("@import url(\"");
        out.push_str(path);
        out.push_str("\");\n\n");
    }

    out.push_str(KANAGAWA_CODE_CSS);
    out.push('\n');

    out
}

// Note: Handlebars {{...}} in here are just literal HTML, not evaluated;
// the page is pure HTML + JS.
const LANDING_PAGE_TEMPLATE: &str = r#"<!-- kanagawa landing -->
<div class="wave-bg">
<div class="wave"></div>
<div class="wave"></div>
<div class="wave"></div>
</div>

<div class="landing">
<h1 class="title">{{LANDING_TITLE}}</h1>
<p class="subtitle">{{LANDING_SUBTITLE}}</p>

<div class="{{GRID_CLASS}}">
  <div class="card">
    <h2>{{HEADER_LATEST}}</h2>
    <div id="latest-posts"><em>Loading...</em></div>
  </div>
  <div class="card">
    <h2>{{HEADER_NOTES}}</h2>
    <div id="recent-notes"><em>Loading...</em></div>
  </div>
  <div class="card">
    <h2>{{HEADER_TAGS}}</h2>
    <div id="tag-cloud" class="tag-cloud"></div>
  </div>
</div>
</div>

<script>
  (function () {
    if (!window.CONTENT_COLLECTIONS) {
      console.warn("kanagawa-theme: window.CONTENT_COLLECTIONS not found; is mdbook-content-loader enabled?");
      return;
    }

    var data = window.CONTENT_COLLECTIONS;
    var entries = data.entries || [];
    var collections = data.collections || {};

    var link = function (p) {
      return (p.path || "").replace(/\.md(?:own|arkdown)?$/i, ".html");
    };

  // Render latest posts into #latest-posts (used on load and when filtering)
    function renderLatest(posts) {
      var latestEl = document.getElementById("latest-posts");
      if (!latestEl) return;

      var list = posts.slice(0, 6);
      latestEl.innerHTML = list.length
        ? list.map(function (p) {
            return (
              '<div class="post-preview">' +
                '<h3><a href="' + link(p) + '">' + (p.title || p.path) + '</a></h3>' +
                (p.date ? '<time>' + new Date(p.date).toISOString().slice(0,10) + '</time>' : '') +
                '<div class="preview">' + (p.preview_html || "") + "</div>" +
              "</div>"
            );
          }).join("")
        : "<p>No posts yet.</p>";
    }

    // Initial latest posts (blog, then fallback to posts)
    var initialPosts = (collections.blog || collections.posts || []);
    renderLatest(initialPosts);

    // Notes
    var notes = (collections.notes || []).slice(0, 8);
    var notesEl = document.getElementById("recent-notes");
    if (notesEl) {
      notesEl.innerHTML = notes.length
        ? notes.map(function (p) {
            return '• <a href="' + link(p) + '">' + (p.title || p.path) + "</a><br>";
          }).join("")
        : "<p>No notes yet.</p>";
    }

    // Tag cloud
    var tagCounts = {};
    (entries || []).forEach(function (p) {
      (p.tags || []).forEach(function (t) {
        tagCounts[t] = (tagCounts[t] || 0) + 1;
      });
    });

    var tags = Object.entries(tagCounts)
      .sort(function (a, b) { return b[1] - a[1]; })
      .slice(0, 15);

    var tagEl = document.getElementById("tag-cloud");
    if (tagEl) {
      // Render tags as clickable buttons
      tagEl.innerHTML = tags.map(function (pair) {
        var tag = pair[0], n = pair[1];
        return '<button class="tag-pill" type="button" data-tag="' + tag + '">' +
                 tag + " (" + n + ")" +
               "</button>";
      }).join("");

      // Clicking a tag filters "Latest posts" by that tag
      tagEl.addEventListener("click", function (ev) {
        var btn = ev.target.closest(".tag-pill");
        if (!btn) return;
        var tag = btn.getAttribute("data-tag");

        var source = (collections.blog || collections.posts || []);
        var filtered = source.filter(function (p) {
          return (p.tags || []).includes(tag);
        });

        renderLatest(filtered.length ? filtered : source);
      });
    }
  })();
</script>

<style>
  .post-preview { margin-bottom: 1.5rem; padding-bottom: 1rem; border-bottom: 1px solid rgba(126,156,216,0.2); }
  .post-preview:last-child { border-bottom: none; }
  .preview { margin-top: 0.5rem; opacity: 0.9; font-size: 0.95em; }
</style>
"#;

/// Theme variables: map the Kanagawa color palette onto mdBook theme classes,
/// providing dark and light variants via CSS custom properties.
const KANAGAWA_VARS: &str = r":root.navy,
.navy,
html.navy,
body.navy {
  /* Dark Kanagawa Wave-like */
  --bg: #1F1F28;        /* sumiInk1: default background */
  --bg-alt: #2A2A37;    /* sumiInk2: lighter background (cards) */
  --fg: #DCD7BA;        /* fujiWhite: main foreground */
  --fg-light: #9CABCA;  /* springViolet2: softer text / subtitles */

  /* Waves use blue-ish Kanagawa tones */
  --wave-1: #1F1F28;    /* background */
  --wave-2: #223249;    /* waveBlue1 */
  --wave-3: #2D4F67;    /* waveBlue2 */

  --accent: #7E9CD8;    /* crystalBlue: functions / titles */
  --red: #E46876;       /* waveRed */
  --blue: #7FB4CA;      /* springBlue */

  --heading: #7AA89F;   /* waveAqua2 */
  --links:  #7FB4CA;    /* springBlue: inline links */
  --bold: #C8C093;      /* oldWhite */
  --sidebar-title: #E46876;  /* Kanagawa Wave Red */
  --sidebar-active: var(--sidebar-title)

}

:root.light,
.light,
html.light,
body.light,
:root.rust,
.rust,
html.rust,
body.rust {
  /* Simple light variant, slightly bluish */
  --bg: #F5F5F5;
  --bg-alt: #E8E8E8;
  --fg: #283548;
  --fg-light: #4C5A6E;
  --wave-1: #E0E8F0;
  --wave-2: #C8D8E8;
  --wave-3: #A8C8E0;
  --accent: #345E8F;
  --red: #C4746E;
  --blue: #7FB4CA;

  /* kanagawa aqua for headings */
  --heading: #7AA89F;   /* waveAqua2 */
}
";

/// Extra Kanagawa styles layered on top of mdBook's chrome.css,
/// including the animated wave background, landing layout, and card styling.
const KANAGAWA_EXTRA_CSS: &str = r"
.sidebar .chapter li.part-title {
  color: var(--sidebar-title, var(--red));
  font-weight: 700;
  letter-spacing: 0.02em;
}

/* If your category headers are just text nodes inside the li */
#sidebar .chapter li.chapter-item > a.active {
  color: var(--sidebar-title, var(--red)) !important;
  font-weight: 600;
}

body {
  background: var(--bg);
  color: var(--fg);
}

a {
  color: var(--accent);
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

/* Bold / strong emphasis */
.content strong,
.content b {
  color: var(--bold) !important;
  font-weight: 600;
}

.content a:link,
.content a:visited {
  color: var(--links) !important;
}

.content a:hover,
.content a:focus {
  color: var(--accent) !important;
  text-decoration: underline;
}

.content h1,
.content h2,
.content h3,
.content h4,
.content h5,
.content h6,
.content .header:link,
.content .header:visited,
.content .header:hover,
.content .header:active {
  color: var(--heading) !important;
}
.content h1 { font-weight: 500; }
.content h2 { font-weight: 500; }
      

.wave-bg {
  position: fixed;
  inset: 0;
  z-index: -1;
  background: var(--bg);
  // overflow: hidden;
}

.wave {
  position: absolute;
  bottom: 0;
  left: -50%;
  width: 200%;
  height: 40vh;
  background: var(--wave-1);
  border-radius: 45%;
  animation: wave 20s linear infinite;
}

.wave:nth-child(2) {
  background: var(--wave-2);
  animation-duration: 25s;
  opacity: 0.7;
}

.wave:nth-child(3) {
  background: var(--wave-3);
  animation-duration: 30s;
  opacity: 0.5;
}

@keyframes wave {
  from { transform: translateX(0); }
  to { transform: translateX(-50%); }
}

.landing {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
  padding: 2rem;
}

.title {
  font-size: 4.5rem;
  font-weight: 300;
  margin: 0 0 1rem;
  color: var(--accent);
  text-shadow: 0 2px 10px rgba(0,0,0,0.3);
}

.subtitle {
  font-size: 1.6rem;
  max-width: 700px;
  color: var(--fg-light);
  margin-bottom: 4rem;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 2rem;
  max-width: 1200px;
  width: 100%;
  margin: 0 auto;
}

.grid-wide {
  max-width: 1600px;
}

@media (min-width: 1200px) {
  .grid-wide {
    grid-template-columns: repeat(auto-fit, minmax(360px, 1fr));
  }
}

.card {
  background: var(--bg-alt);
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  transition: transform 0.2s ease, border-color 0.2s ease;
  // border: 1px solid rgba(126,156,216,0.2);
  // backdrop-filter: blur(10px);
}

.card:hover {
    border-color: var(--accent);
    transform: translateY(-2px)
}

.card h2 {
  margin-top: 0;
  color: var(--accent);
  border-bottom: 1px solid var(--accent);
  padding-bottom: 0.5rem;
}

.tag-cloud .tag-pill {
  display: inline-block;
  background: rgba(126,156,216,0.15);
  color: var(--accent);
  padding: 0.5rem 1rem;
  margin: 0.4rem;
  border-radius: 2rem;
  font-size: 0.9rem;
  transition: all 0.2s;
  cursor: pointer;
}

.tag-cloud .tag-pill:hover {
  background: var(--accent);
  color: var(--bg);

.img-wrapper {
  display: none;
}
.checkbox-img:checked ~ .img-wrapper {
  display: flex;
  position: fixed;
  inset: 0;
  z-index: 999;
  background: rgba(0,0,0,0.85);
  align-items: center;
  justify-content: center;
}
.checkbox-img:checked ~ .img-wrapper img {
  max-width: 90vw;
  max-height: 90vh;
}}
";

/// Kanagawa-flavored syntax highlighting for highlight.js.
/// This assumes mdBook's default highlighter and class names.
const KANAGAWA_CODE_CSS: &str = r"
/* Block code: slightly lifted off main bg/card */
pre code.hljs {
  background: #2a3146; /* pick a shade with clear contrast vs --bg and --bg-alt */
  color: var(--fg);
  border: 1px solid rgba(0, 0, 0, 0.5);
  border-radius: 6px;
}

/* Inline highlighted code (no box) */
:not(pre) > code.hljs {
  background: transparent;
  border: none;
  padding: 0;
}

/* Keywords, control flow */
.hljs-keyword,
.hljs-selector-tag,
.hljs-literal {
  color: #E46876; /* waveRed */
}

/* Strings, attributes */
.hljs-string,
.hljs-attr,
.hljs-template-tag {
  color: #98BB6C; /* springGreen */
}

/* Numbers, builtins, types */
.hljs-number,
.hljs-built_in,
.hljs-type {
  color: #7E9CD8; /* crystalBlue */
}

/* Comments */
.hljs-comment {
  color: #727169;
  font-style: italic;
}

/* Function names */
.hljs-title,
.hljs-title.function_ {
  color: #7FB4CA; /* springBlue */
}

/* Constants, variables */
.hljs-variable,
.hljs-constant,
.hljs-symbol {
  color: #FFA066; /* surimiOrange */
}

/* Punctuation / operators */
.hljs-operator,
.hljs-punctuation {
  color: var(--fg);
}
";
