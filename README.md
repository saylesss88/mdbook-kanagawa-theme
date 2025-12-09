# mdbook-kanagawa-theme

`mdbook-kanagawa-theme` provides an interactive, **blog‑like landing page** and
a **full visual theme override** for the HTML renderer. It does not replace
mdBook’s HTML backend, but it does replace the default `theme/css/chrome.css`
with a Kanagawa‑inspired variant. You can easily swap in any colorscheme you
like, with Dracula, Tokyo Night, and Catppuccin-Mocha example palettes included.

By tweaking per‑chapter frontmatter date, you can choose which pages are
featured on the landing (for example, as “Latest Posts” or “Recent Notes”). Your
books structure stays the same, only your landing page is replaced. (This also
works with no date listed by using timestamps).

The theme dropdown still works but the themes have a slightly different look.

The landing page replaces `index.md` with a configurable side‑by‑side or
top-over-bottom layout:

![screenshot1](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/landing1.png)

<details>
<summary> ✔️ Dracula </summary>

![dracula](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/dracula1.png)

</details>

<details>
<summary> ✔️ Tokyo-Night </summary>

![tokyo1](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/jj.png)

</details>

<details>
<summary> ✔️ Catppuccin-Mocha </summary>

![catppuccin-mocha](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/catppuccin2.png)

</details>

- **Latest Posts**
- **Recent Notes**
- **Popular Tags**

The content comes from:

- [`mdbook-content-collections`](https://crates.io/crates/mdbook-content-collections)
- [`mdbook-content-loader`](https://crates.io/crates/mdbook-content-loader)

Together they generate `content-collections.json` and expose a
`window.CONTENT_COLLECTIONS` global that this theme uses to render the cards.

<details>
<summary> ✔️ Popular tags Example </summary>

> You can click on the tag to bring up the associated chapters.

![screenshot2](assets/popular_tags.png)

</details>

<details>
<summary> ✔️ Content Example </summary>

![content](assets/content1.png)

</details>

---

## Installation

This section assumes that you already have `mdbook` installed.

```bash
cargo install mdbook-kanagawa-theme
# Install the themes dependencies
cargo install mdbook-content-collections
cargo install mdbook-content-loader
# You'll probably want to strip the frontmatter
# cargo install mdbook-frontmatter-strip
```

Version check:

```bash
mdbook-kanagawa-theme --version
mdbook-kanagawa-theme -V
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

# Use bigger cards and top-over-bottom layout in full-screen view
# card_layout = "wide"  # or "compact" # side-by-side layout

# Optional: prepend an @import to the generated theme/css/chrome.css
# Path is relative to the built book root (same as other mdBook theme files).
# css_import = "/assets/dracula.css"
# css_import = "/assets/tokyo-night.css"
# css_import = "/assets/catppuccin-mocha.css"

# Optional: if true, the preprocessor will NOT write theme/css/chrome.css
# (use this if you want to maintain your own chrome.css instead)
# disable_builtin_css = true

[output.html]
# Do NOT use additional-css for the main theme override.
# default-theme still controls which theme class is set (rust, coal, navy, …)
default-theme = "navy"
preferred-dark-theme = "navy"
```

You can change the header names to whatever you prefer.

On each build, the preprocessor:

1. Overwrites `index.md` with the Kanagawa landing page HTML.

2. Writes `theme/css/chrome.css`, built from a template copy of mdBook’s own
   `chrome.css` plus Kanagawa variables and extra styles.

You do **not** need `additional-css = ["theme/kanagawa.css"]`; the theme is
injected by replacing `theme/css/chrome.css` directly. This was required for the
theme to be respected in the latest mdbook versions.

<details>
<summary> ✔️ card_layout = "wide" </summary>

This only changes the cards width on bigger screens, the look is the same for
phones:

![card_layout = "wide"](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/wide.png)

</details>

---

## Usage

**Latest posts**

The "Latest posts" card is tied to the `blog` collection.

This theme works by filtering and sorting your frontmatter. For example, to add
links/overviews/pics to the "Latest posts" card, your frontmatter would look
like this:

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

Now the chapter with the above frontmatter will be added to the "Latest posts"
card.

If you place an image above the first header, it will be shown in this card
along with the date and overview. I may add the author to each chapter shown in
"Latest posts" in the future but currently the author isn't listed.

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

Now this chapter will be added to the "Recent notes" card. This card only lists
the links rather than the complete overview like "Latest posts" does. The
frontmatter is fairly forgiving but in this case, `"notes"` must be quoted.

---

**Popular tags**

Popular tags is automatically populated from the `tags` key in the frontmatter.

You can click the tag to bring up the overviews of the chapters associated with
the said tag.

---

## Create an index.md

The theme can use a blank or titled `index.md` for the landing page, you can
create one by adding the following to your `SUMMARY.md` as the first line:

```md
[](index.md)
```

Or

```md
[Introduction](index.md)
```

Run `mdbook build`, and the theme is automatically injected and applied.

---

## Overriding the palette (Dracula, Tokyo-Night, etc.)

Dracula is just an example, the idea is to allow overriding to whatever you
prefer, just change the values in the provided `dracula.css` to what you like.

<details>
<summary> ✔️ Dracula Override </summary>

You can still override the color palette while keeping the Kanagawa layout.

Create `your-book/src/assets/dracula.css`:

```css
/* src/assets/dracula.css */

:root.coal,
.coal,
html.coal,
body.coal {
  /* Core page colors */
  --bg: #282a36;
  --bg-alt: #44475a;
  --fg: #f8f8f2;
  --fg-light: #cfcfd9;

  /* Waves / accent palette */
  --wave-1: #282a36;
  --wave-2: #343746;
  --wave-3: #44475a;

  --accent: #bd93f9; /* main accent (purple) */
  --red: #ff5555;
  --blue: #8be9fd;

  /* Sidebar */
  --sidebar-bg: #282a36;
  --sidebar-fg: #f8f8f2;
  --sidebar-non-existant: #6272a4;
  --sidebar-active: #bd93f9;
  --sidebar-spacer: #44475a;

  /* Content links / headings / bold */
  --links: #8be9fd; /* cyan links */
  --heading: #bd93f9; /* purple headings */
  --bold: #ffb86c; /* warm emphasis */

  /* Optional extras */
  --quote-bg: #343746;
  --quote-border: #44475a;
  --table-header-bg: #44475a;
  --table-alternate-bg: #343746;
}
```

When you run `mdbook build` this file will be placed in
`your-book/book/assets/dracula.css`.

**This only overrides the coal color-scheme** from the dropdown. The other
themes will still use the Kanagawa defaults.

And in `book.toml`:

```toml
[preprocessor.kanagawa-theme]
renderers = ["html"]
before = ["content-loader", "content-collections"]

landing_title = "My mdBook"
landing_subtitle = "Notes, posts, and more"

# These strings can be changed to whatever you prefer
header_latest = "Latest posts"
header_notes = "Recent notes"
header_tags = "Popular tags"

css_import = "/assets/dracula.css"
disable_builtin_css = false

[output.html]
# automatically switch to the override
default-theme = "coal"
```

With this setup:

- mdBook still builds the book as usual, and `mdbook-kanagawa-theme` generates
  `theme/css/chrome.css` with the Kanagawa layout and variable hooks.​

- The preprocessor adds an `@import "/assets/dracula.css";` (from `css_import`)
  at the top of that generated `chrome.css`, so your Dracula file runs after the
  built‑in variable defaults.​

- Because `dracula.css` redefines the same CSS custom properties used by the
  Kanagawa theme (`--bg`, `--fg`, `--accent`, sidebar colors, etc.), the page
  keeps the Kanagawa layout and landing page, but all colors for the `coal`
  theme class come from your Dracula palette instead. Kanagawa provides the
  layout and default palette.

- The dracula theme only overrides the `coal` theme from the dropdown, this way
  you can easily switch between the normal themes with dracula and kanagawa
  added.

</details>

### Tokyo-Night

<details>
<summary> ✔️ Tokyo-Night Override </summary>

Create `your-book/src/assets/tokyo-night.css`:

```css
/* src/assets/tokyo-night.css */

:root.coal,
.coal,
html.coal,
body.coal {
  /* Core page colors (Tokyo Night-ish) */
  --bg: #1a1b26;
  --bg-alt: #24283b;
  --fg: #c0caf5;
  --fg-light: #a9b1d6; /* lighter foreground */

  /* Waves / accent palette */
  --wave-1: #1a1b26; /* deepest background */
  --wave-2: #24283b; /* normal background */
  --wave-3: #292e42; /* slightly raised panels */

  --accent: #7aa2f7; /* main accent (blue) */
  --red: #f7768e; /* error / destructive */
  --blue: #7dcfff; /* secondary accent / info */

  /* Sidebar */
  --sidebar-bg: #1a1b26;
  --sidebar-fg: #c0caf5;
  --sidebar-non-existant: #565f89; /* dimmed items */
  --sidebar-active: #7aa2f7;
  --sidebar-spacer: #24283b;

  /* Content links / headings / bold */
  --links: #7dcfff; /* cyan-ish links */
  --heading: #7aa2f7; /* blue headings */
  --bold: #ff9e64; /* warm emphasis */

  /* Optional extras */
  --quote-bg: #24283b;
  --quote-border: #292e42;
  --table-header-bg: #292e42;
  --table-alternate-bg: #24283b;
}
```

Add this to your `book.toml` along with what's been shown so far. The overrides
only apply to the `coal` theme from the dropdown:

```toml
css_import = "/assets/tokyo-night.css"
disable_builtin_css = false

[output.html]
# automatically switch to the override
default-theme = "coal"
```

- The other themes will still use the Kanagawa defaults.

![tokyo2](https://raw.githubusercontent.com/saylesss88/mdbook-kanagawa-theme/main/assets/tokyo.png)

</details>

---

## Catppuccin Mocha

<details>
<summary> ✔️ Catppuccin-Mocha Override </summary>

Create `your-book/src/assets/catppuccin-mocha.css`

```css
/* src/assets/catppuccin-mocha.css */

:root.coal,
.coal,
html.coal,
body.coal {
  /* Core page colors (Catppuccin Mocha-inspired) */
  --bg: #1e1e2e; /* base */
  --bg-alt: #313244; /* surface0 */
  --fg: #cdd6f4; /* text */
  --fg-light: #a6adc8; /* subtext1 */

  /* Waves / accent palette */
  --wave-1: #1e1e2e; /* base */
  --wave-2: #313244; /* surface0 */
  --wave-3: #45475a; /* surface1 */

  --accent: #89b4fa; /* blue */
  --red: #f38ba8; /* red */
  --blue: #89b4fa; /* blue */

  /* Sidebar */
  --sidebar-bg: #1e1e2e; /* base */
  --sidebar-fg: #cdd6f4; /* text */
  --sidebar-non-existant: #6c7086; /* overlay1 */
  --sidebar-active: #89b4fa; /* blue */
  --sidebar-spacer: #313244; /* surface0 */

  /* Content links / headings / bold */
  --links: #89b4fa; /* blue links */
  --heading: #cba6f7; /* mauve headings */
  --bold: #f9e2af; /* yellow emphasis */

  /* Optional extras */
  --quote-bg: #313244; /* surface0 */
  --quote-border: #45475a; /* surface1 */
  --table-header-bg: #45475a; /* surface1 */
  --table-alternate-bg: #313244; /* surface0 */
}
```

Add this to your `book.toml` along with what's been shown so far. The overrides
only apply to the `coal` theme from the dropdown:

```toml
css_import = "/assets/catppuccin-mocha.css"
disable_builtin_css = false

[output.html]
# automatically switch to the override
default-theme = "coal"
```

- The other themes continue to use the Kanagawa palette

</details>

## Light/Dark and default-theme behavior

Because this crate replaces `theme/css/chrome.css`, it effectively owns the
visual theme for all built‑in modes (`light`, `rust`, `coal`, `navy`). The
mdBook theme dropdown and your `default-theme` / `preferred-dark-theme` still
control which class is applied to the page, but the palette for each of those
classes is defined by the Kanagawa (or Dracula/Tokyo-Night overridden)
variables.

---

## Stripping the frontmatter

mdBook does not parse or strip YAML frontmatter, so the raw block (e.g. any YAML
keys like `title:`, `date:`, etc.) appears in the HTML.

To avoid this, you can use:

- [mdbook-frontmatter-strip](https://crates.io/crates/mdbook-frontmatter-strip)

### License

[Apache License 2.0](https://github.com/saylesss88/mdbook-kanagawa-theme/blob/main/LICENSE)
