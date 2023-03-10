import rummy from './rust-build/wasm32-unknown-unknown/debug/rummy.wasm'

export default WebAssembly.instantiateStreaming(fetch(rummy)).then(
  (results) => results.instance.exports
)
