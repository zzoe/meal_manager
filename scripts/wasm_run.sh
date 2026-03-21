#!/bin/bash
set -e

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT_DIR/target/makepad-wasm-app/release/meal_manager"
PORT="${1:-8010}"
PORT_ARG=""
if [ -n "${1:-}" ]; then
  PORT_ARG="--port=$PORT"
fi

get_mtime() {
  stat -f %m "$1" 2>/dev/null || stat -c %Y "$1"
}

get_size() {
  stat -f %z "$1" 2>/dev/null || stat -c %s "$1"
}

# Clear stale outputs so we don't patch old files.
rm -f "$OUT_DIR/bindgen.js" "$OUT_DIR/index.html" "$OUT_DIR/env.js"

START_TS=$(date +%s)

cleanup() {
  if [ -n "${RUN_PGID:-}" ]; then
    kill -- "-$RUN_PGID" 2>/dev/null || true
  fi
  if command -v lsof >/dev/null 2>&1; then
    PIDS=$(lsof -ti :"$PORT" 2>/dev/null || true)
    if [ -n "$PIDS" ]; then
      kill $PIDS 2>/dev/null || true
    fi
  fi
}

trap cleanup INT TERM EXIT

if command -v setsid >/dev/null 2>&1; then
  setsid cargo makepad wasm $PORT_ARG --bindgen run -p meal_manager --release &
  RUN_PID=$!
  RUN_PGID=$RUN_PID
else
  cargo makepad wasm $PORT_ARG --bindgen run -p meal_manager --release &
  RUN_PID=$!
  RUN_PGID=$(ps -o pgid= "$RUN_PID" | tr -d ' ' || true)
fi

apply_wasm_patch() {
  # Ensure env.js exists and provides the imports expected by wasm.
  cat > "$OUT_DIR/env.js" <<'EOF'
let _wasm = null;

export function set_wasm(wasm) {
    _wasm = wasm;
}

function bridge() {
    return _wasm && _wasm._bridge;
}

export function js_console_log(u8_ptr, len) {
    const b = bridge();
    if (!b) return;
    b.js_console_log(u8_ptr, len);
}

export function js_console_error(u8_ptr, len) {
    const b = bridge();
    if (!b) return;
    b.js_console_error(u8_ptr, len);
}

export function js_time_now() {
    return Date.now() / 1000.0;
}

export function js_open_web_socket(id, url_ptr, url_len) {
    const b = bridge();
    if (!b) {
        console.error("js_open_web_socket out of context");
        return;
    }
    b.js_open_web_socket(id, url_ptr, url_len);
}

export function js_web_socket_send_string(id, str_ptr, str_len) {
    const b = bridge();
    if (!b) {
        console.error("js_web_socket_send_string out of context");
        return;
    }
    b.js_web_socket_send_string(id, str_ptr, str_len);
}

export function js_web_socket_send_binary(id, bin_ptr, bin_len) {
    const b = bridge();
    if (!b) {
        console.error("js_web_socket_send_binary out of context");
        return;
    }
    b.js_web_socket_send_binary(id, bin_ptr, bin_len);
}
EOF

  if [ -f "$OUT_DIR/bindgen.js" ]; then
    perl -0pi -e 's#from "env"#from "./env.js"#g' "$OUT_DIR/bindgen.js"
  fi

  cat > "$OUT_DIR/index.html" <<'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset='utf-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0, user-scalable=no'>
    <title>meal_manager</title>
    <script type='module'>
        import {init_env} from './makepad_wasm_bridge/wasm_bridge.js'
        import {set_wasm as set_bindgen_wasm} from './env.js'
        import init from './bindgen.js';

        let env = {};
        let set_wasm = init_env(env);
        let module = await WebAssembly.compileStreaming(fetch('./meal_manager.wasm'))
        let wasm = await init({module_or_path: module}, env);
        set_wasm(wasm);
        set_bindgen_wasm(wasm);

        wasm._has_thread_support = wasm.exports.memory.buffer instanceof SharedArrayBuffer;
        wasm._memory = wasm.exports.memory;
        wasm._module = module;
        import {WasmWebGL} from './makepad_platform/web_gl.js'

        class MyWasmApp {
            constructor(wasm) {
                let canvas = document.getElementsByClassName('full_canvas')[0];
                this.webgl = new WasmWebGL (wasm, this, canvas);
            }
        }
        let app = new MyWasmApp(wasm);
    </script>
    <link rel='stylesheet' type='text/css' href='./makepad_platform/full_canvas.css'>
</head>
<body>
    <canvas class='full_canvas'></canvas>
    <div class='canvas_loader'>
        <div>Loading..</div>
    </div>
</body>
</html>
EOF
}

# Wait for freshly generated outputs then patch once.
for _ in $(seq 1 120); do
  if [ -f "$OUT_DIR/bindgen.js" ] && [ -f "$OUT_DIR/index.html" ]; then
    BINDGEN_MTIME=$(get_mtime "$OUT_DIR/bindgen.js")
    INDEX_MTIME=$(get_mtime "$OUT_DIR/index.html")
    BINDGEN_SIZE=$(get_size "$OUT_DIR/bindgen.js")
    INDEX_SIZE=$(get_size "$OUT_DIR/index.html")
    if [ "$BINDGEN_MTIME" -ge "$START_TS" ] && [ "$INDEX_MTIME" -ge "$START_TS" ] \
      && [ "$BINDGEN_SIZE" -gt 0 ] && [ "$INDEX_SIZE" -gt 0 ]; then
      apply_wasm_patch
      break
    fi
  fi
  sleep 0.5
done

wait "$RUN_PID"
