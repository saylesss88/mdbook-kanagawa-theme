# mdbook-kanagawa-theme

mdbook-kanagawa-theme

mdbook-kanagawa-theme is a Kanagawa‑inspired visual theme for the mdBook HTML
renderer. It ships in two modes controlled by a Cargo feature flag:

| Mode                 | Install                                               | What you get                                                                                                                                                                                                                                                           |
| :------------------- | :---------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Theme only (default) | `cargo install mdbook-kanagawa-theme`                 | Kanagawa CSS injected into any standard mdBook. No landing page, no extra preprocessors required.                                                                                                                                                                      |
| Blog landing         | `cargo install mdbook-kanagawa-theme --features blog` | Everything above plus an interactive landing page (Latest Posts, Recent Notes, Popular Tags) powered by [mdbook-content-collections](https://crates.io/crates/mdbook-content-collections) and [mdbook-content-loader](https://crates.io/crates/mdbook-content-loader). |

In both modes the preprocessor writes `theme/css/chrome.css` (Kanagawa palette)
and `theme/css/kanagawa-code.css` (syntax highlighting). You can swap in
Dracula, Tokyo Night, or Catppuccin-Mocha palettes without changing the feature
flag.

`mdBook`'s built-in theme selector still works; each base theme is re-skinned
with Kanagawa variables.

The blog landing page replaces `index.md` with a configurable side‑by‑side or
top-over-bottom layout:

![screenshot1](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/landing1.png)
![screenshot1](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/kanagawa-example.png)

**Live Demo / Showcase**

1. [nix-book (Kanagawa default)](https://saylesss88.github.io/)
2. [privacy-book (Tokyo Night override)](https://mako088.github.io/android/RethinkDNS_Guide.html)

<details>
<summary>✔️ Dracula</summary>

![dracula](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/dracula1.png)
</details>

<details>
<summary>✔️ Tokyo Night</summary>

![tokyo1](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/jj.png)
</details>

<details>
<summary>✔️ Catppuccin Mocha</summary>

![catppuccin-mocha](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/catppuccin2.png)
</details>

The landing page cards (Latest Posts, Recent Notes, Popular Tags) are powered
by:

- [`mdbook-content-collections`](https://crates.io/crates/mdbook-content-collections):
  walks your `src/` tree, parses frontmatter, and writes
  `content-collections.json`.
- [`mdbook-content-loader`](https://crates.io/crates/mdbook-content-loader):
  injects that JSON as `window.CONTENT_COLLECTIONS` into every page.

Without `--features blog` these two preprocessors are not needed and the theme
works as a pure CSS override on any ordinary mdBook.

<details>
<summary>✔️ Popular Tags Example</summary>

> Click a tag to bring up the associated chapters.

![screenshot2](assets/popular_tags.png)
</details>

<details>
<summary>✔️ Content Example</summary>

![content](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/content1.png)
</details>

<details>
<summary>✔️ Kanagawa syntax highlighting</summary>

![kanagawa-code](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/kanagawa-code2.png)
</details>

---

## Installation

Theme only (CSS theming for any standard mdBook)

```bash
cargo install mdbook-kanagawa-theme
```

```bash
cargo install mdbook-kanagawa-theme --features blog
cargo install mdbook-content-collections
cargo install mdbook-content-loader
# Optional: strip YAML frontmatter from rendered output
# cargo install mdbook-frontmatter-strip
```

`--features` blog enables the interactive landing page that reads from
`window.CONTENT_COLLECTIONS`. The two companion preprocessors must also be
installed and configured in `book.toml` (see the Blog configuration section
below).

Version check:

```bash
mdbook-kanagawa-theme --version
```

---

## File layout

Before looking at configuration, here is where every file lives. The
`your-book/` prefix is wherever your `book.toml` sits.

```sh
your-book/
├── book.toml
├── src/
│ ├── SUMMARY.md
│ ├── index.md ← overwritten on each build
│ └── assets/ ← palette overrides (copied to book/ on build)
│ ├── dracula.css ← optional
│ ├── tokyo-night.css ← optional
│ └── catppuccin-mocha.css ← optional
└── theme/
└── css/
├── kanagawa-code.css ← generated by preprocessor (Kanagawa highlighting)
├── dracula-code.css ← optional, you create this
├── tokyo-night-code.css ← optional, you create this
└── catppuccin-mocha-code.css ← optional, you create this
```

Two kinds of CSS files are involved and they do different things:

**Palette files** (`src/assets/*.css`) redefine CSS custom properties (`--bg`,
`--fg`, `--accent`, etc.) for a specific `mdBook` theme class (always `coal`).
They are placed in `src/assets/` so `mdBook` copies them to `book/assets/`
during the build. The preprocessor prepends an `@import` of this file to the
generated `chrome.css` via `css_import` in `book.toml`. **These override the
page colors only, not code block syntax tokens.**

**Code highlight files** (`theme/css/*-code.css`) override highlight.js token
colors for code blocks. They are listed under `additional-css` in `book.toml`
and loaded after mdBook's bundled highlight theme. Because mdBook's default
highlight CSS loads last and can win the cascade, all background and color rules
in these files must use **literal hex values** (not `var(--fg)` or
`var(--code-bg)`) and `!important`.

---

## Configuration

**Theme only** (`cargo install mdbook-kanagawa-theme`)

Minimal `book.toml`: no content-collections preprocessors required:

```toml
[book]
title = "My Kanagawa Book"
authors = ["Your Name"]

[build]
build-dir = "book"

[preprocessor.kanagawa-theme]
renderers = ["html"]

# Optional: show a small footer link on every page
# support_footer = true
# support_footer_href = "https://github.com/you/your-book"
# support_footer_text = "Built with mdbook-kanagawa-theme"

# Palette override (uncomment one):
# css_import = "/assets/dracula.css"
# css_import = "/assets/tokyo-night.css"
# css_import = "/assets/catppuccin-mocha.css"

# disable_builtin_css = true  # manage chrome.css yourself

[output.html]
default-theme = "navy"
preferred-dark-theme = "navy"

additional-css = [
    "theme/css/kanagawa-code.css",
]
```
**Blog landing page** (`cargo install mdbook-kanagawa-theme --features blog`)

Requires `mdbook-content-collections` and `mdbook-content-loader` to also be installed. The preprocessor order matters:

```toml
[book]
title = "My Kanagawa Book"
authors = ["Your Name"]
description = "Docs with a configurable Kanagawa-inspired landing page"

[build]
build-dir = "book"

[preprocessor.content-collections]
renderers = ["html"]

[preprocessor.content-loader]
command = "mdbook-content-loader"
renderers = ["html"]
after = ["content-collections"]

[preprocessor.kanagawa-theme]
renderers = ["html"]
before = ["content-loader", "content-collections"]

# Landing page text
landing_title    = "My Docs"
landing_subtitle = "Notes, posts, and more"

# Column headers (change to whatever you prefer)
header_latest = "Latest posts"
header_notes  = "Recent notes"
header_tags   = "Popular tags"

# card_layout = "wide"    # wider cards on large screens (default: "compact")
# support_footer = true   # show "Made with mdbook-kanagawa-theme" in footer

# Palette override: path is relative to the built book root.
# Uncomment exactly one, or omit to use the Kanagawa default palette.
# css_import = "/assets/dracula.css"
# css_import = "/assets/tokyo-night.css"
# css_import = "/assets/catppuccin-mocha.css"

# disable_builtin_css = true  # set if you want to manage chrome.css yourself

[output.html]
# Kanagawa overrides the "navy" palette by default.
# If you are using a palette override (Dracula, Tokyo Night, Catppuccin),
# change both of these to "coal", those palettes target the coal theme class.
default-theme = "navy"
preferred-dark-theme = "navy"

# Do NOT add theme/css/chrome.css here, the preprocessor injects it directly.
# List only the code highlight file(s) you want:
additional-css = [
    "theme/css/kanagawa-code.css",
    # "theme/css/dracula-code.css",
    # "theme/css/tokyo-night-code.css",
    # "theme/css/catppuccin-mocha-code.css",
]
```

> **Kanagawa defaults to `navy`.** The palette override themes (Dracula, Tokyo
> Night, Catppuccin) all target the `coal` theme class. If you use one of those,
> set `default-theme = "coal"` and `preferred-dark-theme = "coal"`.

On each build the preprocessor:

1. Overwrites `src/index.md` with the Kanagawa landing page HTML.
2. Writes `theme/css/chrome.css` from a template plus Kanagawa variables.
3. If `css_import` is set, prepends `@import "<path>";` to that `chrome.css`.

You do **not** need `additional-css = ["theme/css/chrome.css"]`; that file is
injected by replacing it directly.

<details>
<summary>✔️ card_layout = "wide"</summary>

Only changes card width on larger screens; phones look the same either way.

![card_layout = "wide"](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/wide.png)
</details>

---

## Usage

The theme can use a blank or titled `index.md` for the landing page. Add one of
these as the first line of `SUMMARY.md`:

```md
[](index.md)
```

or

```md
[Introduction](index.md)
```

---

## Kanagawa syntax highlighting (optional)

The preprocessor also generates `theme/css/kanagawa-code.css`, Kanagawa-flavored
syntax highlighting for `highlight.js` code blocks. To apply it:

```toml
[output.html]
additional-css = ["theme/css/kanagawa-code.css"]
```

`chrome.css` is still injected automatically; only the code theme needs to be
listed in `additional-css`.

---

## Frontmatter

Adding `date` is strongly recommended but not required, entries with no `date`
fall back to file timestamps.

**Latest Posts** tied to `collection: "blog"`:

```yaml
---
title: Nix Pull Requests
date: 2025-11-27
author: saylesss88
collection: "blog"
tags: ["nixos", "nixpkgs"]
draft: false
---
```

If you place an image above the first heading it will appear in the card along
with the date and overview.

**Recent Notes** tied to `collection: "notes"`:

```yaml
---
title: Intro to Derivations
date: 2025-11-21
author: saylesss88
collection: "notes"
tags: ["notes", "derivations"]
---
```

`"notes"` must be quoted. This card lists links only, without the full overview.

**Popular Tags** populated automatically from the `tags` key. Click a tag to see
overviews for associated chapters.

Run `mdbook build` and the theme is automatically injected.

---

## Palette overrides (Dracula, Tokyo Night, Catppuccin)

All palette overrides follow the same two-file pattern:

1. **`src/assets/<theme>.css`** redefines CSS custom properties for the `coal`
   theme class. Placed in `src/assets/` so mdBook copies it to `book/assets/` on
   build. Referenced via `css_import` in `book.toml`.
2. **`theme/css/<theme>-code.css`** overrides `highlight.js` token colors.
   Listed in `additional-css`. Must use **literal hex values** (not CSS
   variables) and `!important` on all `background` and `color` rules, because
   `mdBook`'s bundled highlight CSS loads after yours and will win the cascade
   otherwise.

> The palette file only overrides the `coal` theme from the dropdown. All other
> themes (navy, rust, light) continue to use Kanagawa defaults.

---

### Dracula

<details>
<summary>✔️ Dracula Override</summary>

**Step 1.** Create `your-book/src/assets/dracula.css`:

```css
/* src/assets/dracula.css */
:root.coal,
.coal,
html.coal,
body.coal {
  --bg: #282a36;
  --bg-alt: #44475a;
  --fg: #f8f8f2;
  --fg-light: #cfcfd9;
  --code-bg: #363a4f;
  --wave-1: #282a36;
  --wave-2: #343746;
  --wave-3: #44475a;
  --accent: #bd93f9;
  --red: #ff5555;
  --blue: #8be9fd;
  --sidebar-bg: #282a36;
  --sidebar-fg: #f8f8f2;
  --sidebar-non-existant: #6272a4;
  --sidebar-active: #bd93f9;
  --sidebar-spacer: #44475a;
  --links: #8be9fd;
  --heading: #bd93f9;
  --bold: #ffb86c;
  --quote-bg: #343746;
  --quote-border: #44475a;
  --table-header-bg: #44475a;
  --table-alternate-bg: #343746;
}
```

**Step 2.** Create `your-book/theme/css/dracula-code.css`:

```css
/* theme/css/dracula-code.css */
.hljs,
pre code.hljs,
code.hljs {
  background: #282a36 !important;
  color: #f8f8f2 !important;
}
.hljs-keyword,
.hljs-selector-tag,
.hljs-type {
  color: #ff79c6 !important; /* pink */
}
.hljs-string,
.hljs-attribute,
.hljs-attr {
  color: #f1fa8c !important; /* yellow */
}
.hljs-number,
.hljs-literal {
  color: #bd93f9 !important; /* purple */
}
.hljs-variable,
.hljs-tag,
.hljs-regexp,
.hljs-symbol,
.hljs-bullet {
  color: #8be9fd !important; /* cyan */
}
.hljs-comment {
  color: #6272a4 !important;
  font-style: italic !important;
}
pre code.hljs {
  background: #282a36 !important;
  color: #f8f8f2 !important;
  border: 1px solid rgba(0, 0, 0, 0.5);
  border-radius: 6px;
}
:not(pre) > code.hljs {
  background: transparent;
  border: none;
  padding: 0;
}
```

**Step 3.** Update `book.toml`:

```toml
[preprocessor.kanagawa-theme]
# ... your other options ...
css_import = "/assets/dracula.css"

[output.html]
default-theme = "coal"
preferred-dark-theme = "coal"
additional-css = [
    # "theme/css/kanagawa-code.css",
    "theme/css/dracula-code.css",
]
```

![dracula-code](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/dracula-code2.png)
</details>

---

### Tokyo Night

<details>
<summary>✔️ Tokyo Night Override</summary>

**Step 1.** Create `your-book/src/assets/tokyo-night.css`:

```css
/* src/assets/tokyo-night.css */
:root.coal,
.coal,
html.coal,
body.coal {
  --bg: #1a1b26;
  --bg-alt: #24283b;
  --fg: #c0caf5;
  --fg-light: #a9b1d6;
  --code-bg: #292e42;
  --wave-1: #1a1b26;
  --wave-2: #24283b;
  --wave-3: #292e42;
  --accent: #7aa2f7;
  --red: #f7768e;
  --blue: #7dcfff;
  --sidebar-bg: #1a1b26;
  --sidebar-fg: #c0caf5;
  --sidebar-non-existant: #565f89;
  --sidebar-active: #7aa2f7;
  --sidebar-spacer: #24283b;
  --links: #7dcfff;
  --heading: #7aa2f7;
  --bold: #ff9e64;
  --quote-bg: #24283b;
  --quote-border: #292e42;
  --table-header-bg: #292e42;
  --table-alternate-bg: #24283b;
}
```

**Step 2.** Create `your-book/theme/css/tokyo-night-code.css`:

```css
/* theme/css/tokyo-night-code.css */
.hljs,
pre code.hljs,
code.hljs {
  background: #24283b !important;
  color: #c0caf5 !important;
}
.hljs-keyword,
.hljs-selector-tag,
.hljs-type {
  color: #bb9af7 !important; /* purple */
}
.hljs-string,
.hljs-attribute,
.hljs-attr {
  color: #9ece6a !important; /* green */
}
.hljs-number,
.hljs-literal {
  color: #ff9e64 !important; /* orange */
}
.hljs-variable,
.hljs-tag,
.hljs-regexp,
.hljs-symbol,
.hljs-bullet {
  color: #7dcfff !important; /* cyan */
}
.hljs-comment {
  color: #565f89 !important;
  font-style: italic !important;
}
pre code.hljs {
  background: #24283b !important;
  color: #c0caf5 !important;
  border: 1px solid rgba(0, 0, 0, 0.5);
  border-radius: 6px;
}
:not(pre) > code.hljs {
  background: transparent;
  border: none;
  padding: 0;
}
```

**Step 3.** Update `book.toml`:

```toml
[preprocessor.kanagawa-theme]
# ... your other options ...
css_import = "/assets/tokyo-night.css"

[output.html]
default-theme = "coal"
preferred-dark-theme = "coal"
additional-css = [
    # "theme/css/kanagawa-code.css",
    "theme/css/tokyo-night-code.css",
]
```

![tokyo-code](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/tokyo-night-syntaxhighlighting.png)
</details>

---

### Catppuccin Mocha

<details>
<summary>✔️ Catppuccin Mocha Override</summary>

**Step 1.** Create `your-book/src/assets/catppuccin-mocha.css`:

```css
/* src/assets/catppuccin-mocha.css */
:root.coal,
.coal,
html.coal,
body.coal {
  --bg: #1e1e2e;
  --bg-alt: #313244;
  --fg: #cdd6f4;
  --fg-light: #a6adc8;
  --code-bg: #45475a;
  --wave-1: #1e1e2e;
  --wave-2: #313244;
  --wave-3: #45475a;
  --accent: #89b4fa;
  --red: #f38ba8;
  --blue: #89b4fa;
  --sidebar-bg: #1e1e2e;
  --sidebar-fg: #cdd6f4;
  --sidebar-non-existant: #6c7086;
  --sidebar-active: #89b4fa;
  --sidebar-spacer: #313244;
  --links: #89b4fa;
  --heading: #cba6f7;
  --bold: #f9e2af;
  --quote-bg: #313244;
  --quote-border: #45475a;
  --table-header-bg: #45475a;
  --table-alternate-bg: #313244;
}
```

**Step 2.** Create `your-book/theme/css/catppuccin-mocha-code.css`:

```css
/* theme/css/catppuccin-mocha-code.css */
.hljs,
pre code.hljs,
code.hljs {
  background: #313244 !important;
  color: #cdd6f4 !important;
}
.hljs-keyword,
.hljs-selector-tag,
.hljs-type {
  color: #cba6f7 !important; /* mauve */
}
.hljs-string,
.hljs-attribute,
.hljs-attr {
  color: #a6e3a1 !important; /* green */
}
.hljs-number,
.hljs-literal {
  color: #fab387 !important; /* peach */
}
.hljs-variable,
.hljs-tag,
.hljs-regexp,
.hljs-symbol,
.hljs-bullet {
  color: #f38ba8 !important; /* red/pink */
}
.hljs-comment {
  color: #7f849c !important;
  font-style: italic !important;
}
pre code.hljs {
  background: #45475a !important;
  color: #cdd6f4 !important;
  border: 1px solid rgba(0, 0, 0, 0.5);
  border-radius: 6px;
}
:not(pre) > code.hljs {
  background: transparent;
  border: none;
  padding: 0;
}
```

**Step 3.** Update `book.toml`:

```toml
[preprocessor.kanagawa-theme]
# ... your other options ...
css_import = "/assets/catppuccin-mocha.css"

[output.html]
default-theme = "coal"
preferred-dark-theme = "coal"
additional-css = [
    # "theme/css/kanagawa-code.css",
    "theme/css/catppuccin-mocha-code.css",
]
```

![mocha-code](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/catppuccin-code2.png)
</details>

---

## Light/Dark and default-theme behavior

Because this crate replaces `theme/css/chrome.css`, it owns the visual theme for
all built‑in modes (`light`, `rust`, `coal`, `navy`). The `mdBook` theme
dropdown and `default-theme` / `preferred-dark-theme` still control which class
is applied to the page, but the palette for each class is defined by the
Kanagawa (or overridden) variables.

---

## Stripping frontmatter

`mdBook` does not parse or strip YAML frontmatter, so the raw block appears in
the rendered HTML. To suppress it:

- [mdbook-frontmatter-strip](https://crates.io/crates/mdbook-frontmatter-strip)

---

### License

[Apache License 2.0](https://github.com/saylesss88/mdbook-kanagawa-theme/blob/main/LICENSE)
