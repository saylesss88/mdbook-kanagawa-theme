# mdbook-kanagawa-theme

This is not a full replacement for the built-in mdBook themes.  
Instead, it provides a custom landing page that replaces `index.md` with a
Kanagawa-inspired layout: side-by-side **Latest Posts** and **Recent Notes**,
plus a **Popular Tags** card.

This theme was created with, and relies on:

- [mdbook-content-collections](https://crates.io/crates/mdbook-content-collections)
- [mdbook-content-loader](https://crates.io/crates/mdbook-content-loader)

It exists mainly to demonstrate how these preprocessors can be combined to build
a blog-like landing page.

## Installation

```bash
cargo install mdbook-kanagawa-theme
```

Add the following to your `book.toml`:

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

With this:

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
