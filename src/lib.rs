use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, BookItem},
    errors::Error,
};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

pub struct KanagawaTheme;

impl Default for KanagawaTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl KanagawaTheme {
    pub fn new() -> Self {
        KanagawaTheme
    }
}

#[derive(Debug, Default, Deserialize)]
struct KanagawaConfig {
    /// Landing page main title
    landing_title: Option<String>,
    /// Landing page subtitle
    landing_subtitle: Option<String>,

    /// Column headers
    header_latest: Option<String>,
    header_notes: Option<String>,
    header_tags: Option<String>,

    /// Optional CSS @import to append to theme/css/chrome.css
    css_import: Option<String>,

    /// If true, don't write theme/css/chrome.css at all
    disable_builtin_css: Option<bool>,

    /// Card layout preset: "compact" (default) or "wide"
    card_layout: Option<String>,
}

impl Preprocessor for KanagawaTheme {
    fn name(&self) -> &str {
        "kanagawa-theme"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // Read config from [preprocessor.kanagawa-theme] in book.toml
        let cfg = KanagawaConfig {
            landing_title: ctx
                .config
                .get::<String>("preprocessor.kanagawa-theme.landing_title")
                .ok()
                .flatten(),
            landing_subtitle: ctx
                .config
                .get::<String>("preprocessor.kanagawa-theme.landing_subtitle")
                .ok()
                .flatten(),
            header_latest: ctx
                .config
                .get::<String>("preprocessor.kanagawa-theme.header_latest")
                .ok()
                .flatten(),
            header_notes: ctx
                .config
                .get::<String>("preprocessor.kanagawa-theme.header_notes")
                .ok()
                .flatten(),
            header_tags: ctx
                .config
                .get::<String>("preprocessor.kanagawa-theme.header_tags")
                .ok()
                .flatten(),
            css_import: ctx
                .config
                .get::<String>("preprocessor.kanagawa-theme.css_import")
                .ok()
                .flatten(),
            disable_builtin_css: ctx
                .config
                .get::<bool>("preprocessor.kanagawa-theme.disable_builtin_css")
                .ok()
                .flatten(),
            card_layout: ctx
                .config
                .get::<String>("preprocessor.kanagawa-theme.card_layout")
                .ok()
                .flatten(),
        };

        // Overwrite index.md with the landing page
        let mut landing_injected = false;
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item
                && chapter.path.as_ref().and_then(|p| p.file_stem()) == Some("index".as_ref())
                && !landing_injected
            {
                chapter.content = build_landing_page(&cfg);
                landing_injected = true;
            }
        });

        // Write theme/css/chrome.css to override the built-in theme.
        if !cfg.disable_builtin_css.unwrap_or(false) {
            let css_dir: PathBuf = ctx.root.join("theme").join("css");
            if let Err(e) = fs::create_dir_all(&css_dir) {
                log::warn!("kanagawa-theme: failed to create theme/css dir: {e}");
            } else {
                let css = build_full_chrome_css(&cfg);
                if let Err(e) = fs::write(css_dir.join("chrome.css"), css) {
                    log::warn!("kanagawa-theme: failed to write theme/css/chrome.css: {e}");
                }
            }
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}

fn build_landing_page(cfg: &KanagawaConfig) -> String {
    let title = cfg.landing_title.as_deref().unwrap_or("mdTheme");
    let subtitle = cfg
        .landing_subtitle
        .as_deref()
        .unwrap_or("A dope landing powered by rust");

    let header_latest = cfg.header_latest.as_deref().unwrap_or("Latest Posts");
    let header_notes = cfg.header_notes.as_deref().unwrap_or("Recent Notes");
    let header_tags = cfg.header_tags.as_deref().unwrap_or("Popular Tags");

    let layout = cfg.card_layout.as_deref().unwrap_or("compact");

    let grid_class = match layout {
        "wide" => "grid grid-wide",
        _ => "grid",
    };

    let mut html = LANDING_PAGE_TEMPLATE.to_owned();
    html = html.replace("{{LANDING_TITLE}}", title);
    html = html.replace("{{LANDING_SUBTITLE}}", subtitle);
    html = html.replace("{{HEADER_LATEST}}", header_latest);
    html = html.replace("{{HEADER_NOTES}}", header_notes);
    html = html.replace("{{HEADER_TAGS}}", header_tags);
    html = html.replace("{{GRID_CLASS}}", grid_class);

    html
}

/// Build a full chrome.css by taking a template copy of mdBook's chrome.css,
/// optionally prefixing an @import (user CSS) at the very top,
/// then injecting Kanagawa variables and appending extra Kanagawa-specific CSS.
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

/// Theme variables: bind Kanagawa palette to mdBook theme classes.
const KANAGAWA_VARS: &str = r#":root.navy,
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
"#;

/// Extra Kanagawa styles layered on top of mdBook's chrome.css.
const KANAGAWA_EXTRA_CSS: &str = r#"body {
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
  overflow: hidden;
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
  border-radius: 16px;
  border: 1px solid rgba(126,156,216,0.2);
  backdrop-filter: blur(10px);
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
}
"#;
