# mdbook-kanagawa-theme

This is not a full replacement for the built-in mdBook themes.  
Instead, it provides a custom landing page that replaces `index.md` with a
Kanagawa-inspired layout: side-by-side **Latest Posts** and **Recent Notes**,
plus a **Popular Tags** card.

The landing page is driven entirely by metadata from
`mdbook-content-collections` and `mdbook-content-loader`, which together provide
a `content-collections.json` file and a `window.CONTENT_COLLECTIONS` global in
the rendered HTML. The goal of this crate is to demonstrate how those
preprocessors can be combined to build a blog‑like front page for your book.​

This theme was created with, and relies on:

- [mdbook-content-collections](https://crates.io/crates/mdbook-content-collections)
- [mdbook-content-loader](https://crates.io/crates/mdbook-content-loader)

It exists mainly to demonstrate how these preprocessors can be combined to build
a blog-like landing page.

## Installation

```bash
cargo install mdbook-kanagawa-theme
```

Add the preprocessor and CSS to your `book.toml`:

```toml
[book]
title = "My Kanagawa Book"
authors = ["Your Name"]
description = "Docs with a configurable Kanagawa-inspired landing page"

[build]
# Optional, just to keep builds tidy
build-dir = "book"

[preprocessor.kanagawa-theme]
renderers = ["html"]

# Landing page text
landing_title    = "My Docs"
landing_subtitle = "Notes, posts, and more"

# Column headers
header_latest = "Latest posts"
header_notes  = "Recent notes"
header_tags   = "Popular tags"

# Optional: have the generated theme/kanagawa.css import another palette
# This path is relative to the compiled book root, same as additional-css.
# css_import = "theme/dracula.css"

# Optional: if true, the preprocessor will NOT write theme/kanagawa.css
# disable_builtin_css = true

[output.html]
# Tell mdBook to load the CSS file that the preprocessor writes
additional-css = ["theme/kanagawa.css"]
default-theme = "coal"
```

As you can see above, all of these fields can be overridden. I'm still working
on the custom `css_import` being rendered correctly, the flags work but with no
change to appearance as of right now.

<details>
<summary> ✔️ Click for example `dracula.css` </summary>

```css
/* theme/dracula.css */

/* Dark variants (coal/navy) use classic Dracula colors */
html.coal,
html.navy {
  --bg: #282a36; /* Dracula background */
  --bg-alt: #44475a; /* current line */
  --fg: #f8f8f2; /* foreground */
  --fg-light: #cfcfd9; /* slightly dimmer foreground */

  --wave-1: #282a36;
  --wave-2: #343746; /* UI background lighter */
  --wave-3: #44475a;

  --accent: #bd93f9; /* purple */
  --red: #ff5555;
  --blue: #8be9fd; /* cyan */
}

/* Light variants can be a softer “inverted Dracula” feel */
html.light,
html.rust {
  --bg: #f8f8f2;
  --bg-alt: #e6e6df;
  --fg: #282a36;
  --fg-light: #44475a;

  --wave-1: #f0efe6;
  --wave-2: #e6e5dc;
  --wave-3: #dcdad3;

  --accent: #6272a4; /* comment/secondary tone */
  --red: #ff5555;
  --blue: #8be9fd;
}

/* Optional: tighten how links and cards look under Dracula */
a {
  color: var(--accent);
}

a:hover {
  text-decoration: underline;
}

.card {
  box-shadow: 0 0 22px rgba(0, 0, 0, 0.4);
}
```

Again, this is a WIP.

</details>

### Preprocessor pipeline

`mdbook-kanagawa-theme` is meant to be part of a small preprocessor pipeline.

A typical `book.toml` looks like this:

```toml
[preprocessor.content-collections]
renderers = ["html"]

[preprocessor.content-loader]
renderers = ["html"]
after = ["content-collections"]

[preprocessor.kanagawa-theme]
renderers = ["html"]
before = ["content-loader", "content-collections"]

[output.html]
additional-css = ["theme/kanagawa.css"]
```

With this ordering:

1. `mdbook-content-collections` builds `content-collections.json`.

2. `mdbook-content-loader` injects `window.CONTENT_COLLECTIONS`.

3. `mdbook-kanagawa-theme` overwrites `index.md` content with your landing page
   and writes `theme/kanagawa.css`.

4. The default HTML backend picks up `theme/kanagawa.css` via `additional-css`.

The theme expects a blank `index.md` for the landing page, you can create one by
adding the following to your `SUMMARY.md` as the first line:

```md
[](index.md)
```

Run `mdbook build`, and the theme is automatically injected and applied.

## Light/Dark behavior

`kanagawa.css` hooks into mdBook’s existing theme classes (`html.light`,
`html.rust`, `html.coal`, `html.navy`), so the built-in theme dropdown still
works.
