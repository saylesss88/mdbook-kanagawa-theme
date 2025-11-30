# mdbook-kanagawa-theme

`mdbook-kanagawa-theme` provides a Kanagawa-inspired **landing page** and **full
visual theme override** for the HTML renderer.  
It does not replace mdBook’s HTML backend, but it does replace the default
`theme/css/chrome.css` with a Kanagawa-flavored version.

The theme dropdown still works but the themes have a slightly different look.

The landing page replaces `index.md` with a side‑by‑side layout (or not for
small screens):

- **Latest Posts**
- **Recent Notes**
- **Popular Tags**

The content comes from:

- [`mdbook-content-collections`](https://crates.io/crates/mdbook-content-collections)
- [`mdbook-content-loader`](https://crates.io/crates/mdbook-content-loader)

Together they generate `content-collections.json` and expose a
`window.CONTENT_COLLECTIONS` global that this theme uses to render the cards.

---

## Installation

```bash
cargo install mdbook-kanagawa-theme
```

Version check:

```bash
mdbook-kanagawa-theme --version
```

Add the preprocessor to your `book.toml`. (I've included the other dependencies
for clarity, you'll still want to set your `site-url` and whatever else your
site requires):

```toml
[book]
title = "My Kanagawa Book"
authors = ["Your Name"]
description = "Docs with a configurable Kanagawa-inspired landing page"

[build]
# Optional, just to keep builds tidy
build-dir = "book"

[preprocessor.content-collections]
renderers = ["html"]

[preprocessor.content-loader]
command = "mdbook-content-loader"
renderers = ["html"]
after = ["content-collections"]
# inject_all = true

[preprocessor.kanagawa-theme]
renderers = ["html"]
before = ["content-loader", "content-collections"]
# Landing page text
landing_title    = "My Docs"
landing_subtitle = "Notes, posts, and more"

# Column headers
header_latest = "Latest posts"
header_notes  = "Recent notes"
header_tags   = "Popular tags"

# Optional: prepend an @import to the generated theme/css/chrome.css
# Path is relative to the built book root (same as other mdBook theme files).
# css_import = "theme/dracula.css"


# Optional: if true, the preprocessor will NOT write theme/css/chrome.css
# (use this if you want to maintain your own chrome.css instead)
# disable_builtin_css = true

[output.html]
# Do NOT use additional-css for the main theme override.
# default-theme still controls which theme class is set (rust, coal, navy, …)
default-theme = "rust"
preferred-dark-theme = "navy"
```

On each build, the preprocessor:

1. Overwrites `index.md` with the Kanagawa landing page HTML.

2. Writes `theme/css/chrome.css`, built from a template copy of mdBook’s own
   `chrome.css` plus Kanagawa variables and extra styles.

You do **not** need `additional-css = ["theme/kanagawa.css"]`; the theme is
injected by replacing `theme/css/chrome.css` directly. This was required for the
theme to be respected in the latest mdbook versions.

## Usage

**Latest posts**

This theme works by filtering and sorting your frontmatter. For example, to add
links/overviews/pics to the "Latest posts" card, your frontmatter would look
like this:

```yaml
---
title: Nix Pull Requests
date: 2025-11-27
author: saylesss88
collection: blog
tags: ["nixos", "nixpkgs"]
draft: false
---
```

Now the chapter with the above frontmatter will be added to the "Latest posts"
card.

---

**Recent notes**

The "Recent notes" card is tied to the `notes` collection:

```yaml
---
title: Intro to Derivations
date: 2025-11-21
author: saylesss88
collection: "notes"
tags: ["notes", "derivations"]
---
```

Now this chapter will be added to the "Recent notes" card.

---

**Popular tags**

Popular tags is automatically populated from the `tags` key in the frontmatter.

You can click the tag to list the associated chapters.

---

## Overriding the palette (Dracula, etc.)

This is still a WIP, currently the overrides aren't being respected.

<details>
<summary> ✔️ Click to Expand override Example </summary>

You can still override the color palette while keeping the Kanagawa layout.

Example `theme/dracula.css`:

```css
/* theme/dracula.css */
html.navy,
body.navy,
.navy,
html.light,
body.light,
.light {
  --bg: #282a36;
  --bg-alt: #44475a;
  --fg: #f8f8f2;
  --fg-light: #cfcfd9;

  --wave-1: #282a36;
  --wave-2: #343746;
  --wave-3: #44475a;

  --accent: #bd93f9;
  --red: #ff5555;
  --blue: #8be9fd;
}

/* Optional: tweak links/cards */
a {
  color: var(--accent);
}
a:hover {
  text-decoration: underline;
}
.card {
  box-shadow: 0 0 22px rgba(0, 0, 0, 0.45);
}
```

And in `book.toml`:

```toml
[preprocessor.kanagawa-theme]
renderers = ["html"]

landing_title = "My mdBook"
landing_subtitle = "Notes, posts, and more"

header_latest = "Latest posts"
header_notes = "Recent notes"
header_tags = "Popular tags"

css_import = "theme/dracula.css"
disable_builtin_css = false
```

With this setup:

- Kanagawa provides the layout and default palette.

- `css_import` adds `@import "theme/dracula.css";` to the top of
  `theme/css/chrome.css`.

- `dracula.css` redefines the same CSS variables (`--bg`, `--fg`, `--accent`,
  etc.), so your Dracula colors win while keeping the Kanagawa landing layout.

</details>

---

## Preprocessor pipeline

This theme is intended to sit in a small pipeline with the content
preprocessors:

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
default-theme = "rust"
preferred-dark-theme = "navy"
```

Order matters:

1. `mdbook-content-collections` builds `content-collections.json`.

2. `mdbook-content-loader` injects `window.CONTENT_COLLECTIONS`.

3. `mdbook-kanagawa-theme` overwrites `index.md` content with your landing page
   and writes `theme/css/chrome.css`.

The theme expects a blank `index.md` for the landing page, you can create one by
adding the following to your `SUMMARY.md` as the first line:

```md
[](index.md)
```

Run `mdbook build`, and the theme is automatically injected and applied.

---

## Light/Dark and default-theme behavior

Because this crate replaces `theme/css/chrome.css`, it effectively owns the
visual theme for all built‑in modes (`light`, `rust`, `coal`, `navy`). The
mdBook theme dropdown and your `default-theme` / `preferred-dark-theme` still
control which class is applied to the page, but the palette for each of those
classes is defined by the Kanagawa (or Dracula‑overridden) variables.
