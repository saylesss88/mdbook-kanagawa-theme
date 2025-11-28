use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct KanagawaTheme;

impl Preprocessor for KanagawaTheme {
    fn name(&self) -> &str {
        "kanagawa-theme"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut landing_injected = false;

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                // Only replace the root index.md → index.html
                if chapter.path.as_ref().and_then(|p| p.file_stem()) == Some("index".as_ref()) {
                    if !landing_injected {
                        chapter.content = LANDING_PAGE.to_string();
                        landing_injected = true;
                    }
                }
            }
        });

        // Also inject CSS into the theme if not already present
        let theme_dir = ctx.root.join("theme");
        std::fs::create_dir_all(&theme_dir)?;
        std::fs::write(theme_dir.join("kanagawa.css"), KANAGAWA_CSS)?;

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

// Full landing page with live content collections
const LANDING_PAGE: &str = r#"
<div class="wave-bg">
  <div class="wave"></div>
  <div class="wave"></div>
  <div class="wave"></div>
</div>

<div class="landing">
  <h1 class="title">{{ title }}</h1>
  <p class="subtitle">{{ description | default(value="A beautiful documentation site powered by mdBook") }}</p>

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
  if (!window.CONTENT_COLLECTIONS) {
    document.body.innerHTML += '<div style="text-align:center;padding:4rem;color:#c4746e"><h2>Warning: mdbook-content-loader not enabled!</h2><p>Add <code>[preprocessor.content-loader]</code> to your book.toml</p></div>';
  } else {
    const { entries, collections } = window.CONTENT_COLLECTIONS;
    const link = p => p.path.replace(/\.md$/, '.html');

    // Latest posts
    const posts = (collections.blog || collections.posts || []).slice(0, 6);
    document.getElementById('latest-posts').innerHTML = posts.length
      ? posts.map(p => `
          <div class="post-preview">
            <h3><a href="${link(p)}">${p.title}</a></h3>
            ${p.date ? `<time>${new Date(p.date).toLocaleDateString()}</time>` : ''}
            <div class="preview">${p.preview_html || ''}</div>
          </div>
        `).join('')
      : '<p>No posts yet.</p>';

    // Notes
    const notes = (collections.notes || []).slice(0, 8);
    document.getElementById('recent-notes').innerHTML = notes.length
      ? notes.map(p => `• <a href="${link(p)}">${p.title}</a><br>`).join('')
      : '<p>No notes yet.</p>';

    // Tag cloud
    const counts = {};
    entries.forEach(p => p.tags.forEach(t => counts[t] = (counts[t] || 0) + 1));
    const tags = Object.entries(counts)
      .sort((a,b) => b[1] - a[1])
      .slice(0, 15);

    document.getElementById('tag-cloud').innerHTML = tags.map(([tag, n]) => 
      `<a href="{{ path_to_root }}search.html?q=${encodeURIComponent(tag)}">${tag} (${n})</a>`
    ).join('');
  }
</script>

<style>
  .post-preview { margin-bottom: 1.5rem; padding-bottom: 1rem; border-bottom: 1px solid rgba(126,156,216,0.2); }
  .post-preview:last-child { border-bottom: none; }
  .preview { margin-top: 0.5rem; opacity: 0.9; font-size: 0.95em; }
</style>
"#;

// Full Kanagawa CSS (dark + light mode)
const KANAGAWA_CSS: &str = r#"
:root {
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

.tag-cloud a {
  display: inline-block;
  background: rgba(126,156,216,0.15);
  color: var(--accent);
  padding: 0.5rem 1rem;
  margin: 0.4rem;
  border-radius: 2rem;
  font-size: 0.9rem;
  transition: all 0.2s;
}

.tag-cloud a:hover {
  background: var(--accent);
  color: var(--bg);
}
"#;
