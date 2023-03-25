import wasm from './rummy-wasm/rummy_bg.wasm'
import { __wbg_set_wasm } from './rummy-wasm/rummy_bg.js'

// @ts-ignore
export default WebAssembly.instantiateStreaming(fetch(wasm))
  .then((results) => __wbg_set_wasm(results.instance.exports))
  .then(() => import('./rummy-wasm/rummy_bg.js'))
