The repository provides strongly-typed Rust bindings for ASC X12 EDI versions. These notes help an AI coding agent become productive quickly by describing the project's architecture, conventions, and common workflows.

1) Big picture
- Purpose: each `vNNNN` module contains types and parsing/serialization helpers for a specific X12 version (e.g. `src/v004010`, `src/v005010`). The crate exposes those modules behind Cargo feature flags declared in `Cargo.toml`.
- Data flow: callers build or parse `Transmission` values (e.g. `Transmission::<_835>`) which contain `isa`, `functional_group` and `iea`. Parsing is implemented with `nom`; formatting uses `DisplaySegment`/`ParseSegment` derives from `x12-types-macros`.

2) Key files & locations (use these as entry points)
- `src/lib.rs` — top-level modules and feature flags. Enable/disable versions via Cargo features.
- `src/vNNNN/segment.rs` — the canonical place for segment structs (fields named `_01`, `_02`, ... and annotated with `serde(rename = "NN")`). Example: `src/v004010/segment.rs`.
- `src/util/*` — parsing helpers (e.g. `parse_line`) and small helpers used across versions.
- `examples/` — runnable examples (e.g. `examples/parse_x12.rs` uses `Transmission::<_835>::parse`). Good for integration-level behavior.
- `docs/` — human docs and templates (e.g. `docs/segment-template.md`) useful when adding new segments.
- `test-data/` — sample EDI files used by tests and examples.

3) Project-specific conventions (important to follow)
- Naming: segments are uppercase types (AK1, AK2, ...). Fields are numbered `_01`, `_02`, ... and have `serde(rename = "01")` style annotations.
- Derives/macros: segment structs routinely derive `Serialize`, `Deserialize`, `Default`, `Debug`, `PartialEq`, `Eq` and the macro derives `DisplaySegment` and `ParseSegment` from `x12-types-macros`. New structs should follow the same derives.
- Tests: tests live inside the version folders (files named `test_*.rs`). When adding behavior for a version, add tests adjacent to that version's code.
- Feature flags: each supported X12 version is a Cargo feature (see `[features]` in `Cargo.toml`). To add a new version, add a `vNNNN` feature, add the module to `src/lib.rs`, and follow the code layout in existing `vNNNN` folders.

4) Build / run / test (concrete commands)
- Build default (all default versions): `cargo build` (default features are enabled per `Cargo.toml`).
- Build with specific version(s): `cargo build --no-default-features --features "v005010"` (or include multiple features space-separated). This is useful to speed CI or local iteration.
- Run example (uses whatever features the example depends on): `cargo run --example parse_x12` (if needed, add `--no-default-features --features "v005010"`).
- Run tests (all): `cargo test` (runs version tests enabled by features). To test a single version: `cargo test --no-default-features --features "v004010"`.

5) Typical editing tasks and where to change things
- Add/modify a segment: edit `src/vNNNN/segment.rs` for the appropriate version. Use the same `_NN` field pattern and derive attributes. Consult `docs/segment-template.md` to match formatting.
- Add a new version: add `src/vYYYYY/mod.rs` & `segment.rs`, export it in `src/lib.rs` behind a new `vYYYYY` feature, and add the feature to `Cargo.toml`.
- Update parsing/formatting behavior: `x12-types-macros` centralizes a lot of parse/display logic. Prefer using the existing derives and adjust macros only if a change is cross-cutting and covered by tests.

6) Libraries and integration points to be aware of
- nom (parsing) — used in `src/util` and in generated parsers.
- serde (serialize/deserialize) — used on all segment structs.
- validator — present as a dependency; some types or helper code may use `Validate`.
- x12-types-macros — custom derive macros that implement `DisplaySegment` and `ParseSegment`. Changing parse/display logic usually involves these macros.

7) Examples to cite when creating edits
- `examples/parse_x12.rs` — parsing a sample 005010_835 into `Transmission::<_835>`.
- `examples/render_x12.rs` & `examples/render_transmission.rs` — show how `DisplaySegment`/`format` are used to serialize a `Transmission`.

8) Safety checks for changes
- Run `cargo test` (or run tests limited to the affected version via features). Many segments have exhaustive structural tests in their version folder; keep these green.
- Preserve `serde(rename)` indices and numbered field names — changing these will break (de)serialization and parsing.

If anything above is unclear or you want the instructions adjusted (e.g. add rules for commit message style, test matrix commands, or CI specifics), tell me which area to expand and I'll iterate.
