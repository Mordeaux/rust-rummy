import sys

import wasmtime.loader

sys.path.append('./src/rust-build/wasm32-unknown-unknown/debug')
import rummy as rummy_wasm

print(rummy_wasm.add(3,3))
