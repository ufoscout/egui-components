# CLAUDE.md

Guidance for Claude (and other AI agents) working in this repository.

> ⚠️ **This codebase is mostly AI-generated.** Treat existing code as a draft:
> read the real source and verify it compiles and behaves correctly before
> trusting it, rather than assuming a component is complete because the README
> marks it "Migrated".

## What this project is

`egui-components` is a **port of [longbridge/gpui-component](https://github.com/longbridge/gpui-component)
to [egui](https://github.com/emilk/egui) 0.34**. The goal is to expose the same
component set as gpui-component, but in idiomatic egui immediate-mode style.

It is **not** a 1:1 translation:

- gpui-component is built on GPUI, a **retained / reactive** framework.
- egui is **immediate-mode**.

So APIs are reshaped to egui conventions while the **visual language is preserved**
(Tailwind / shadcn palette tokens, light + dark themes).

## Upstream baseline commit (READ THIS BEFORE MIGRATING)

The port currently tracks gpui-component at:

```
upstream: https://github.com/longbridge/gpui-component
commit:   2a2fa72d14fa9a4bae63b01ab58ed3fe7fb8ba0b
```

> **Note:** this SHA was recorded as the upstream `main` HEAD at the time the
> migration guidance was written. If you know the migration was actually based on
> a different commit, correct this value.

**Why this matters:** future upstream changes should be migrated *incrementally*.
To find what changed upstream since the baseline:

```bash
# Inspect upstream diffs without cloning everything into the repo
git clone --filter=blob:none https://github.com/longbridge/gpui-component /tmp/gpui-component
cd /tmp/gpui-component
git log --oneline 2a2fa72d14fa9a4bae63b01ab58ed3fe7fb8ba0b..HEAD -- crates/ui/src
git diff   2a2fa72d14fa9a4bae63b01ab58ed3fe7fb8ba0b..HEAD -- crates/ui/src/button.rs
```

The upstream component sources live under
[`crates/ui/src`](https://github.com/longbridge/gpui-component/tree/main/crates/ui/src).

**After you migrate upstream changes, update the baseline SHA above** to the new
upstream commit so the next session has an accurate diff base.

## Workspace layout

| Path | What it provides |
|------|------------------|
| [crates/theme/](crates/theme/) (`egui-components-theme`) | Tailwind color palette, semantic tokens, light/dark `Theme`, and a helper to install it into `egui::Style`. Bundled themes are vendored JSON under [crates/theme/themes/](crates/theme/themes/) and compiled to `const Theme` values by [crates/theme/build.rs](crates/theme/build.rs). |
| [crates/components/](crates/components/) (`egui-components`) | The component widgets. One file per component in [crates/components/src/](crates/components/src/), declared and re-exported from [lib.rs](crates/components/src/lib.rs). Shared helpers (the `Size` / `Variant` enums) live in [common.rs](crates/components/src/common.rs). |
| [examples/demo/](examples/demo/) (`demo`) | An eframe app demonstrating every component with a light/dark toggle. |

Build / run:

```bash
cargo build
cargo run -p demo --release   # visual smoke test
cargo test                     # doc-tests double as compile checks
```

## What to migrate

Two sources of work:

1. **Components not yet ported.** The migration-status table in
   [README.md](README.md) lists every upstream component and its status. Anything
   marked **Pending** is unported. As of writing: `Calendar`, `Chart`,
   `ColorPicker`, `DatePicker`, `Dock`, `Kbd`, `Plot`, `SearchableList`, `Sheet`,
   `Skeleton`, `Spinner`, `Stepper`, `Table`, `Text`.
   - Some Pending components (`Dock`, `Table`, `Chart`, `Calendar`,
     `CodeEditor`) need framework-specific machinery (virtualization, focus
     traps, complex layout) with no direct egui equivalent — they are large
     efforts, not quick ports.
   - Some components are intentionally exposed under a different (shadcn-style)
     name: `Card` ⇐ upstream `GroupBox`, `Toasts` ⇐ `Notification`,
     `Select::combobox` ⇐ `Combobox`, `Rail` ⇐ collapsed `Sidebar`. Keep these
     mappings in mind so you don't double-port.
   - `Tree` is provided via the external [`egui_ltreeview`](https://crates.io/crates/egui_ltreeview)
     crate (re-exported from [tree.rs](crates/components/src/tree.rs)) rather than
     a hand-written port.

2. **Upstream changes since the baseline commit.** See the section above — diff
   upstream against the recorded SHA and port the deltas into the matching
   `crates/components/src/<component>.rs` file.

## How to migrate a component

1. **Read the upstream source** for the component under
   `crates/ui/src/<component>.rs` in gpui-component (at or after the baseline SHA).
2. **Map it to egui conventions** (see "Conventions" below). Preserve the
   visual design (variants, sizes, hover/active/disabled states, colors) but
   express interaction in immediate mode.
3. **Create / update** `crates/components/src/<component>.rs`, declare the module
   in [lib.rs](crates/components/src/lib.rs) (`pub mod x;` + `pub use x::*;`), and
   keep both lists alphabetically ordered to match the existing entries.
4. **Pull colors from the `Theme`**, never hard-code palette values — that keeps
   every bundled theme working.
5. **Add a doc example** in the rustdoc; `cargo test` compiles these so they act
   as a compile check (see existing files for the pattern).
6. **Demo it** — add the component to [examples/demo/src/main.rs](examples/demo/src/main.rs)
   and run `cargo run -p demo --release` to eyeball it in both light and dark mode.
7. **Update the migration-status table** in [README.md](README.md), linking the
   new source file.
8. **If you migrated upstream changes, bump the baseline SHA** in this file.

## Conventions

- **Widgets return `egui::Response`.** Simple, click-style components implement
  `egui::Widget` (e.g. `Button`) so they work with `ui.add(...)`; container-style
  components (e.g. `Card`) expose a `.show(ui, |ui| { ... })` method that returns
  an `InnerResponse`.
- **Builder pattern.** Components are constructed with `::new(...)` and configured
  with chained setters (`.variant(...)`, `.size(...)`, convenience ctors like
  `Button::primary(...)`, etc.).
- **Theming.** Components read the installed theme from the egui context via
  `egui_components_theme::Theme::get(ui.ctx())` — install it once at startup with
  `Theme::dark().install(&ctx)` (or a bundled preset). Do not hard-code Tailwind
  hex values in component code; read `theme.colors` / `theme.metrics`.
- **Sizing.** Use the shared `Size` scale (`Size::Small` / `Size::Medium` /
  `Size::Large`, `Medium` is the default) from [common.rs](crates/components/src/common.rs),
  which maps to per-size metrics on `ThemeMetrics`, rather than inventing
  per-component sizes.
- **Variants.** Reuse the shared `Variant` enum (`Primary`, `Secondary`, `Ghost`,
  `Outline`, `Link`, `Danger`, `Success`, `Warning`, `Info`) where applicable.
- **Naming.** Mirror upstream component and variant names where reasonable; only
  diverge for the documented shadcn-style aliases above, and note the alias in
  both the rustdoc and the README.
- **No runtime theme parsing.** Bundled JSON themes are converted to `const`
  values at compile time via [build.rs](crates/theme/build.rs); keep that pattern.

## Provenance / licensing

- License is **Apache-2.0**, matching upstream.
- gpui-component's default theme derives from [shadcn/ui](https://ui.shadcn.com)'s
  neutral preset; those color values are inherited transitively here.
- The bundled JSON theme files under [crates/theme/themes/](crates/theme/themes/)
  follow gpui-component's theme schema (`.theme-schema.json`).
