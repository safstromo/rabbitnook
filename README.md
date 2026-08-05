# rabbitnook

[rabbitnook.com](https://rabbitnook.com) — my personal site and blog.

A [Leptos](https://github.com/leptos-rs/leptos) full-stack Rust app: server-rendered by Axum,
hydrated in the browser as WebAssembly, styled with Tailwind v4. The home page has a small
interactive terminal; the blog posts are written as Leptos components with client-side syntax
highlighting.

## Development

The dev shell provides the Rust toolchain and every binary `cargo-leptos` shells out to
(`sass`, `tailwindcss`, `wasm-opt`, `wasm-bindgen`). With
[devenv](https://devenv.sh) installed:

```bash
devenv shell
```

Then:

| Command | What it does |
|---|---|
| `dev` | dev server with live reload on <http://127.0.0.1:3000> |
| `release` | production build: binary in `target/release`, site in `target/site` |
| `check` | leptosfmt + rustfmt + clippy, for both the server and WASM builds |
| `fmt` | apply both formatters |
| `cargo test` | unit tests |

`devenv up` runs the watcher as a managed process.

The `wasm-bindgen` CLI version in `devenv.nix` must match the `wasm-bindgen` crate version in
`Cargo.lock` exactly — see the comments in that file before changing either.

## Adding a blog post

1. Add `src/components/blogpages/<slug>.rs` and declare it in that directory's `mod.rs`.
2. Wrap the body in `<BlogLayout>`; it supplies highlight.js, the footer links and the ko-fi
   button. Use `<CodeBlock class="rust" code=my_snippet />` for code.
3. Add the route in `src/app.rs` (keep `ssr=SsrMode::Async`).
4. Add the post to `POSTS` in `src/components/blog.rs` so it appears on the index.

Images go in `public/blog/<slug>/`.

## Deployment

`Dockerfile` builds a static site plus server binary and ships them on a bare Alpine image.
`.github/workflows/docker-publish.yml` builds, pushes to ghcr.io and signs the image with
cosign when a GitHub release is published.

The server binds the address from `[package.metadata.leptos] site-addr` (127.0.0.1:3000).
Set `LEPTOS_SITE_ADDR` at runtime to override it — a container published to other hosts needs
`0.0.0.0:3000`.

## License

MIT — see [LICENSE](LICENSE).
