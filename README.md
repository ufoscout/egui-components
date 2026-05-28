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

## Status

This is an initial subset of the upstream library. The components shipped here have working visuals, variants/sizes, hover/active states, and interaction states. Many upstream components (`Dock`, `Table`, `CodeEditor`, `Chart`, `Calendar`, `Form`, `Menu`, …) are not yet ported because they require substantial framework-specific work (virtualization, focus traps, complex layout primitives) that has no direct egui equivalent and would each take significant additional effort.

## License

Apache-2.0, matching the upstream project.
