pub mod config;
pub mod css;

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

/// Build the HTML source for the Kanagawa landing page by
/// filling `LANDING_PAGE_TEMPLATE` with configured titles, headers,
pub fn build_landing_page(cfg: &KanagawaConfig) -> String {
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
