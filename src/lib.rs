use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::Error,
    Preprocessor, PreprocessorContext,
};
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

impl Preprocessor for KanagawaTheme {
    fn name(&self) -> &str {
        "kanagawa-theme"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut landing_injected = false;

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                if chapter.path.as_ref().and_then(|p| p.file_stem()) == Some("index".as_ref())
                    && !landing_injected
                {
                    chapter.content = LANDING_PAGE.to_string();
                    landing_injected = true;
                }
            }
        });

        // Best-effort write CSS into <root>/theme/kanagawa.css
        let theme_dir: PathBuf = ctx.root.join("theme");
        if let Err(e) = fs::create_dir_all(&theme_dir) {
            log::warn!("kanagawa-theme: failed to create theme dir: {e}");
        } else if let Err(e) = fs::write(theme_dir.join("kanagawa.css"), KANAGAWA_CSS) {
            log::warn!("kanagawa-theme: failed to write kanagawa.css: {e}");
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}

// Note: Handlebars {{...}} in here are just literal HTML, not evaluated;
// the page is pure HTML + JS, which is fine for a custom landing page.
const LANDING_PAGE: &str = r#"<div class="wave-bg">
  <div class="wave"></div>
  <div class="wave"></div>
  <div class="wave"></div>
</div>

<div class="landing">
  <h1 class="title">mdTheme</h1>
  <p class="subtitle">A dope landing powered by rust</p>

  <div class="grid">
    <div class="card">
      <h2>Latest Posts</h2>
      <div id="latest-posts"><em>Loading...</em></div>
    </div>

    <div class="card">
      <h2>Recent Notes</h2>
      <div id="recent-notes"><em>Loading...</em></div>
    </div>

    <div class="card">
      <h2>Popular Tags</h2>
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
    var entries = data.entries || {};
    var collections = data.collections || {};

    var link = function (p) {
      return (p.path || "").replace(/\.md(?:own|arkdown)?$/i, ".html");
    };

    // Latest posts
    var posts = (collections.blog || collections.posts || []).slice(0, 6);
    var latestEl = document.getElementById("latest-posts");
    if (latestEl) {
      latestEl.innerHTML = posts.length
        ? posts.map(function (p) {
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
      tagEl.innerHTML = tags.map(function (pair) {
        var tag = pair[0], n = pair[1];
        return '<span class="tag-pill">' + tag + " (" + n + ")</span>";
      }).join("");
    }
  })();
</script>

<style>
  .post-preview { margin-bottom: 1.5rem; padding-bottom: 1rem; border-bottom: 1px solid rgba(126,156,216,0.2); }
  .post-preview:last-child { border-bottom: none; }
  .preview { margin-top: 0.5rem; opacity: 0.9; font-size: 0.95em; }
</style>"#;

const KANAGAWA_CSS: &str = r#":root {
  --bg: #181616;
  --bg-alt: #1f1f1f;
  --fg: #c5c9c5;
  --fg-light: #d7dae0;
  --wave-1: #1f1f28;
  --wave-2: #2a2a37;
  --wave-3: #223249;
  --accent: #7e9cd8;
  --red: #c4746e;
  --blue: #7fb4ca;
}

@media (prefers-color-scheme: light) {
  :root {
    --bg: #f5f5f5;
    --bg-alt: #e8e8e8;
    --fg: #283548;
    --fg-light: #4c5a6e;
    --wave-1: #e0e8f0;
    --wave-2: #c8d8e8;
    --wave-3: #a8c8e0;
    --accent: #345e8f;
  }
}

body { background: var(--bg); color: var(--fg); margin: 0; font-family: system-ui, sans-serif; }
a { color: var(--accent); text-decoration: none; }
a:hover { text-decoration: underline; }

.wave-bg { position: fixed; inset: 0; z-index: -1; overflow: hidden; }
.wave {
  position: absolute;
  bottom: 0; left: -50%;
  width: 200%; height: 40vh;
  background: var(--wave-1);
  border-radius: 45%;
  animation: wave 20s linear infinite;
}
.wave:nth-child(2) { background: var(--wave-2); animation-duration: 25s; opacity: 0.7; }
.wave:nth-child(3) { background: var(--wave-3); animation-duration: 30s; opacity: 0.5; }
@keyframes wave { from { transform: translateX(0); } to { transform: translateX(-50%); } }

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
}

.tag-cloud .tag-pill:hover {
  background: var(--accent);
  color: var(--bg);
}"#;
