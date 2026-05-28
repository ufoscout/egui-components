# egui-components

A port of [longbridge/gpui-component](https://github.com/longbridge/gpui-component) to [egui](https://github.com/emilk/egui) 0.34, exposing the same component set in idiomatic immediate-mode style.

This is **not** a 1:1 translation. GPUI is retained / reactive while egui is immediate mode, so APIs are reshaped to match egui conventions (builder widgets returning `egui::Response`, `Widget` impls, etc.) while preserving the upstream visual language (Tailwind palette tokens, light + dark themes).

> Provenance: gpui-component's default theme is itself derived from [shadcn/ui](https://ui.shadcn.com)'s neutral preset (the upstream `default-theme.json` credits shadcn). The color values in `egui-components-theme` are inherited transitively from that source.

## Workspace

| Crate | What it provides |
|-------|------------------|
| `egui-components-theme` | Tailwind color palette, semantic tokens, light/dark `Theme` + helper to install it into `egui::Style` |
| `egui-components` | Components: `Button`, `Checkbox`, `Switch`, `Slider`, `Input`, `Badge`, `Label`, `Separator`, `Alert`, `Tag` |
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

| Component | Status |
|-----------|--------|
| Accordion | Pending |
| Alert | [Migrated](crates/components/src/alert.rs) |
| AlertDialog | Pending |
| Avatar | Pending |
| Badge | [Migrated](crates/components/src/badge.rs) |
| Breadcrumb | Pending |
| Button | [Migrated](crates/components/src/button.rs) |
| Calendar | Pending |
| Chart | Pending |
| Checkbox | [Migrated](crates/components/src/checkbox.rs) |
| Collapsible | Pending |
| ColorPicker | Pending |
| Combobox | Pending |
| DatePicker | Pending |
| DescriptionList | Pending |
| Dialog | Pending |
| Dock | Pending |
| Form | Pending |
| GroupBox | Pending |
| HoverCard | Pending |
| Icon | Pending |
| Input | [Migrated](crates/components/src/input.rs) |
| Kbd | Pending |
| Label | [Migrated](crates/components/src/label.rs) |
| Link | Pending |
| List | [Migrated](crates/components/src/list.rs) |
| Menu | Pending |
| Notification | Pending |
| NumberInput | Pending |
| OtpInput | Pending |
| Pagination | Pending |
| Plot | Pending |
| Popover | Pending |
| Progress | Pending |
| Radio | Pending |
| Rating | Pending |
| Resizable | Pending |
| SearchableList | Pending |
| Select | Pending |
| Separator | [Migrated](crates/components/src/separator.rs) |
| Sheet | Pending |
| Sidebar | Pending |
| Skeleton | Pending |
| Slider | [Migrated](crates/components/src/slider.rs) |
| Spinner | Pending |
| Stepper | Pending |
| Switch | [Migrated](crates/components/src/switch.rs) |
| Tab | [Migrated](crates/components/src/tabs.rs) |
| Table | Pending |
| Tag | [Migrated](crates/components/src/tag.rs) |
| Text | Pending |
| TitleBar | Pending |
| Tooltip | Pending |
| Tree | [Migrated](crates/components/src/tree.rs) |

## Status

This is an initial subset of the upstream library. The components shipped here have working visuals, variants/sizes, hover/active states, and interaction states. Many upstream components (`Dock`, `Table`, `CodeEditor`, `Chart`, `Calendar`, `Form`, `Menu`, …) are not yet ported because they require substantial framework-specific work (virtualization, focus traps, complex layout primitives) that has no direct egui equivalent and would each take significant additional effort.

## License

Apache-2.0, matching the upstream project.
