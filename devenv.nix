{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # ---------------------------------------------------------------------------
  # Toolchain
  #
  # rust-toolchain.toml stays the single source of truth (nightly +
  # wasm32-unknown-unknown, both required by leptos_router/nightly and the
  # hydrate bundle). devenv hands the file to rust-overlay, so `channel`,
  # `version` and `targets` must NOT also be set here - the module asserts on
  # that combination.
  # ---------------------------------------------------------------------------
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  # ---------------------------------------------------------------------------
  # Packages
  #
  # nixpkgs builds cargo-leptos with the `no_downloads` feature, so - unlike the
  # Dockerfile's upstream binary - it never fetches its own helper executables
  # into ~/.cache/cargo-leptos. It resolves each one by name from $PATH and
  # hard-errors if one is missing ("X is required but was not found"). Every
  # entry below is therefore load-bearing, under exactly these binary names.
  # ---------------------------------------------------------------------------
  packages = with pkgs; [
    git
    cargo-leptos

    dart-sass # -> `sass`
    binaryen # -> `wasm-opt`, release builds only

    # -> `tailwindcss`. MUST be the v4 package: pkgs.tailwindcss is still 3.4.x
    # and cannot parse style/tailwind.css, which is v4 CSS-first config
    # (`@import "tailwindcss"`, `@theme`).
    tailwindcss_4

    # -> `wasm-bindgen`. The CLI version has to match the wasm-bindgen *crate*
    # version in Cargo.lock exactly or the build fails with a schema mismatch.
    # If you bump the crate, bump this attribute to match (nixpkgs carries
    # wasm-bindgen-cli_0_2_<patch> for each supported patch).
    wasm-bindgen-cli_0_2_126

    # Formats the `view!` macros; plain rustfmt leaves them alone.
    leptosfmt
  ];

  # ---------------------------------------------------------------------------
  # Scripts
  # ---------------------------------------------------------------------------

  scripts.dev.exec = "cargo leptos watch";
  scripts.release.exec = "cargo leptos build --release";

  # ssr and hydrate are mutually exclusive - `--all-features` turns on both and
  # fails to compile - so each side is checked separately, the client one
  # against the wasm target it actually ships to.
  #
  # NB: the tree does not pass this yet. `cargo fmt` wants edition-2024 import
  # ordering in 4 files (run `fmt`), and clippy flags a `let _ =` on a unit value
  # in terminal.rs. leptosfmt is clean.
  scripts.check.exec = ''
    set -e
    leptosfmt --check src
    cargo fmt --all -- --check
    cargo clippy --features ssr -- -D warnings
    cargo clippy --features hydrate --target wasm32-unknown-unknown -- -D warnings
  '';

  scripts.fmt.exec = ''
    set -e
    leptosfmt src
    cargo fmt --all
  '';

  # `devenv up`
  processes.dev.exec = "cargo leptos watch";

  enterShell = ''
    echo "rabbitnook  |  dev  release  check  fmt   (site: http://127.0.0.1:3000)"
  '';

  # There is no browser/e2e tooling here on purpose: the Playwright suite was
  # deleted (it only ever held the unmodified Leptos template test). Adding one
  # back on NixOS means pinning the npm package to pkgs.playwright-driver's
  # version and pointing PLAYWRIGHT_BROWSERS_PATH at pkgs.playwright-driver.browsers,
  # since downloaded browsers do not run here.

  # No git-hooks block: hooks are intentionally not configured for this project.
  # Use `check` manually.
}
