use crate::config::KanagawaConfig;

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

/// Build a complete `chrome.css` by:
/// 1. optionally inserting a user-provided `@import`,
/// 2. appending Kanagawa CSS variables,
/// 3. including the mdBook chrome template, and
/// 4. layering additional Kanagawa styles on top.
#[must_use]
pub fn build_full_chrome_css(cfg: &KanagawaConfig) -> String {
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
#[must_use]
pub fn build_code_css(cfg: &KanagawaConfig) -> String {
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
