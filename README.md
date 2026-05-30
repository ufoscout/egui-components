# egui-components

> ## 🤖 This code is mostly AI-generated
> The bulk of this repository was written by AI agents. Treat it as a draft:
> review, compile, and test before relying on any part of it. Some components
> marked "Migrated" below may still be incomplete. See [CLAUDE.md](CLAUDE.md)
> for the migration workflow and the pinned upstream baseline commit.

A port of [longbridge/gpui-component](https://github.com/longbridge/gpui-component) to [egui](https://github.com/emilk/egui) 0.34, exposing the same component set in idiomatic immediate-mode style.

This is **not** a 1:1 translation. GPUI is retained / reactive while egui is immediate mode, so APIs are reshaped to match egui conventions (builder widgets returning `egui::Response`, `Widget` impls, etc.) while preserving the upstream visual language (Tailwind palette tokens, light + dark themes).

> Provenance: gpui-component's default theme is itself derived from [shadcn/ui](https://ui.shadcn.com)'s neutral preset (the upstream `default-theme.json` credits shadcn). The color values in `egui-components-theme` are inherited transitively from that source.

## Workspace

| Crate | What it provides |
|-------|------------------|
| `egui-components-theme` | Tailwind color palette, semantic tokens, light/dark `Theme` + helper to install it into `egui::Style` |
| `egui-components` | Components: `Button`, `Checkbox`, `Radio`, `Switch`, `Slider`, `Input`, `NumberInput`, `OtpInput`, `Select`/Combobox, `Avatar`, `Card`, `Tooltip`, `HoverCard`, `Popover`, `Badge`, `Label`, `Link`, `Separator`, `Alert`, `Tag`, `Icon`, `Progress`, `Rating`, `Accordion`, `Collapsible`, `Breadcrumb`, `Pagination`, `DescriptionList`, `Form`, `Menu`, `Notification`/`Toasts`, `Dialog`, `AlertDialog`, `Resizable`, `ScrollArea`, `Heading`, `Sidebar`, `Rail`, `TitleBar` |
| `examples/demo` | eframe app demonstrating all components with a light/dark toggle |

## Run the demo

```bash
cargo run -p demo --release
```

## Bundled themes

The upstream gpui-component repo ships 21 JSON theme files (Catppuccin, Tokyo Night, Gruvbox, Solarized, …). All of them are vendored under [crates/theme/themes/](crates/theme/themes/) and converted into Rust `const Theme` values at compile time by [crates/theme/build.rs](crates/theme/build.rs), so consumers pay no runtime parsing cost.

```rust
use egui_components_theme::{presets, Theme};

# let ctx = egui::Context::default();
// Use a named constant directly:
presets::CATPPUCCIN_MOCHA.install(&ctx);

// Or look one up by display name at runtime:
if let Some(theme) = presets::by_name("Tokyo Night") {
    theme.install(&ctx);
}

// Or iterate every bundled theme (36 in total across the 21 files):
for preset in presets::ALL {
    println!("{} (family: {}): {:?}", preset.name, preset.family, preset.theme.mode);
}
```

Any color key the JSON doesn't define falls back to the matching field from `ThemeColor::light()` / `ThemeColor::dark()` based on the theme's `mode`.

## Migration status

The table below lists every component exposed by upstream [`gpui-component`](https://github.com/longbridge/gpui-component/tree/main/crates/ui/src) and whether it has been ported to `egui-components`.

A few rows cover a type we expose under a different (often shadcn-style) name: `Card` for `GroupBox`, `Toasts` for `Notification`, and `Select::combobox` for `Combobox`. Those alternate names are noted in the relevant rows.

| Component | Status |
|-----------|--------|
| Accordion | [Migrated](crates/components/src/accordion.rs) |
| Alert | [Migrated](crates/components/src/alert.rs) |
| AlertDialog | [Migrated](crates/components/src/dialog.rs) |
| Avatar | [Migrated](crates/components/src/avatar.rs) |
| Badge | [Migrated](crates/components/src/badge.rs) |
| Breadcrumb | [Migrated](crates/components/src/breadcrumb.rs) |
| Button | [Migrated](crates/components/src/button.rs) |
| Calendar | Pending |
| Card | [Migrated](crates/components/src/card.rs) (shadcn-style surface; upstream's nearest equivalent is `GroupBox`) |
| Chart | Pending |
| Checkbox | [Migrated](crates/components/src/checkbox.rs) |
| Collapsible | [Migrated](crates/components/src/collapsible.rs) |
| ColorPicker | Pending |
| Combobox | [Migrated](crates/components/src/select.rs) (as `Select::combobox`) |
| DatePicker | Pending |
| DescriptionList | [Migrated](crates/components/src/description_list.rs) |
| Dialog | [Migrated](crates/components/src/dialog.rs) |
| Dock | Pending |
| Form | [Migrated](crates/components/src/form.rs) |
| GroupBox | [Migrated](crates/components/src/card.rs) (as `Card`) |
| Heading | [Migrated](crates/components/src/heading.rs) (shadcn-style section title; upstream's nearest equivalent is `Text`) |
| HoverCard | [Migrated](crates/components/src/hover_card.rs) |
| Icon | [Migrated](crates/components/src/icon.rs) |
| Input | [Migrated](crates/components/src/input.rs) |
| Kbd | Pending |
| Label | [Migrated](crates/components/src/label.rs) |
| Link | [Migrated](crates/components/src/link.rs) |
| List | [Migrated](crates/components/src/list.rs) |
| Menu | [Migrated](crates/components/src/menu.rs) |
| Notification | [Migrated](crates/components/src/notification.rs) |
| NumberInput | [Migrated](crates/components/src/number_input.rs) |
| OtpInput | [Migrated](crates/components/src/otp_input.rs) |
| Pagination | [Migrated](crates/components/src/pagination.rs) |
| Plot | Pending |
| Popover | [Migrated](crates/components/src/popover.rs) |
| Progress | [Migrated](crates/components/src/progress.rs) |
| Radio | [Migrated](crates/components/src/radio.rs) |
| Rail | [Migrated](crates/components/src/sidebar.rs) (icon-only navigation rail; upstream models it as a collapsed `Sidebar`) |
| Rating | [Migrated](crates/components/src/rating.rs) |
| Resizable | [Migrated](crates/components/src/resizable.rs) |
| ScrollArea | [Migrated](crates/components/src/scroll_area.rs) (themed wrapper over `egui::ScrollArea`; upstream's `scroll` module) |
| SearchableList | Pending |
| Select | [Migrated](crates/components/src/select.rs) |
| Separator | [Migrated](crates/components/src/separator.rs) |
| Sheet | Pending |
| Sidebar | [Migrated](crates/components/src/sidebar.rs) |
| Skeleton | Pending |
| Slider | [Migrated](crates/components/src/slider.rs) |
| Spinner | Pending |
| Stepper | Pending |
| Switch | [Migrated](crates/components/src/switch.rs) |
| Tab | [Migrated](crates/components/src/tabs.rs) |
| Table | Pending |
| Tag | [Migrated](crates/components/src/tag.rs) |
| Text | Pending |
| TitleBar | [Migrated](crates/components/src/titlebar.rs) |
| Tooltip | [Migrated](crates/components/src/tooltip.rs) |
| Tree | Done by using [egui_ltreeview](https://crates.io/crates/egui_ltreeview)|

## Status

This is an initial subset of the upstream library. The components shipped here have working visuals, variants/sizes, hover/active states, and interaction states. Many upstream components (`Dock`, `Table`, `CodeEditor`, `Chart`, `Calendar`, …) are not yet ported because they require substantial framework-specific work (virtualization, focus traps, complex layout primitives) that has no direct egui equivalent and would each take significant additional effort.

## License

Apache-2.0, matching the upstream project.
