# Meal Manager

A Makepad-based desktop and web app for meal order statistics and employee registry.

## Features
- Paste meal order text and generate lunch/dinner summaries plus exceptions.
- Maintain employee registry with aliases for name matching.
- Local persistence:
  - Desktop: Redb file `meal_manager.redb`.
  - Web: `localStorage` key `meal_manager:employees`.

## Data Format
Input lines are parsed as:

```
name: 11
name 01
name：10
```

Rules:
- The parser extracts two digits; the first is lunch count, the second is dinner count.
- Unknown names and malformed lines are listed in the exception panel.
- Name matching supports aliases from the employee registry.

## Build and Run

### Desktop
```
cargo run
```

### Web (Wasm)
Install wasm toolchain (only needed once):
```
cargo makepad wasm install-toolchain
```

Build wasm package:
```
cargo makepad wasm --bindgen build -p meal_manager --release
```

Run local server (optional):
```
cargo makepad wasm --bindgen run -p meal_manager --release
```

The build output is placed in:
`target/makepad-wasm-app/release/meal_manager`

When serving, configure headers:
- `Cross-Origin-Embedder-Policy: require-corp`
- `Cross-Origin-Opener-Policy: same-origin`

And set mime types:
- `.html` -> `text/html`
- `.wasm` -> `application/wasm`
- `.css` -> `text/css`
- `.js` -> `text/javascript`
- `.ttf` -> `application/ttf`
- `.png` -> `image/png`
- `.jpg` -> `image/jpg`
- `.svg` -> `image/svg+xml`
- `.md` -> `text/markdown`
