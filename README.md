# mdbook-kanagawa-theme

Still enables you to use whichever theme you prefer.

This theme was created with and relies on:
[mdbook-content-collections](https://crates.io/crates/mdbook-content-collections),
and [mdbook-content-loader](https://crates.io/crates/mdbook-content-loader). And
is a project to show some of their functionality.

I'm still working out a bug that displays a random code block in a random place
on the landing.

## Installation

```bash
cargo install mdbook-kanagawa-theme
```

Then in your `book.toml` add the following:

```toml
[preprocessor.content-collections]
renderers = ["html"]

[preprocessor.content-loader]
renderers = ["html"]
after = ["content-collections"]

[preprocessor.kanagawa-theme]
renderers = ["html"]

[output.html]
additional-css = ["theme/kanagawa.css"]
```

With this:

1. `mdbook-content-collections` builds `content-collections.json`.

2. `mdbook-content-loader` injects `window.CONTENT_COLLECTIONS`.

3. `mdbook-kanagawa-theme` overwrites index.md content with your landing page
   and writes `theme/kanagawa.css`.

4. The default HTML backend picks up `theme/kanagawa.css` via `additional-css`.

The theme expects a blank `index.md` for the landing page, you can create one by
adding the following to your `SUMMARY.md` as the first line:

```md
[](index.md)
```
