# Laperm Plugin template

A minimal [Laperm](https://github.com/k-ymmt/Laperm) Plugin in Rust: one Command, **Insert Date**, that
inserts today's date at the caret using a format chosen in the Plugin's settings.

Use this repository as a template ("Use this template" on GitHub), then:

1. Rename the crate in `Cargo.toml` and describe your Plugin in `manifest.json` (`id`, `name`, `author`,
   `permissions`, `commands`, `settings`). The rules are in the SDK's
   [`docs/plugin-api.md`](https://github.com/k-ymmt/laperm-plugin-sdk/blob/main/docs/plugin-api.md).
2. Implement the [`Plugin`](https://github.com/k-ymmt/laperm-plugin-sdk) trait in `src/lib.rs`.
3. Build and try it locally:

   ```sh
   rustup target add wasm32-unknown-unknown
   cargo build --release --target wasm32-unknown-unknown
   mkdir -p "<your Vault>/.laperm/plugins/insert-date"
   cp manifest.json "<your Vault>/.laperm/plugins/insert-date/"
   cp target/wasm32-unknown-unknown/release/laperm_plugin_template.wasm "<your Vault>/.laperm/plugins/insert-date/plugin.wasm"
   ```

   Then in Laperm: File → Manage Plugins… → enable it. A hand-placed Plugin gets no update checks.
4. Release: bump `version` in both `Cargo.toml` and `manifest.json`, commit, tag `v<version>` and push the tag.
   The `Release` workflow builds `plugin.wasm` and publishes a GitHub Release with `manifest.json` and
   `plugin.wasm`. Users install it in Laperm with your `owner/repository`.

A Plugin is a `wasm32-unknown-unknown` core module with no WASI: no clock, files or network except through
Laperm's host API (`laperm_plugin::{editor, vault, storage, clock, settings, notice, log}`).
