# speclang

## outputs

### wasm (todo: automate build)

1. `$ wasm-pack build --target=bundler`
2. edit `pkg/speclang_bg.js`

| line                       | before                                        | after                                                  |
| -------------------------- | --------------------------------------------- | ------------------------------------------------------ |
| 1                          | `import * as wasm from './speclang_bg.wasm';` | `import initWasm from './speclang_bg.wasm'; let wasm;` |
| wasm_bindgen method export | `export function parse(input) {`              | `export async function parse(input) {`                 |
| wasm_bindgen method body   | insert at start of each method body           | `if (!wasm) { wasm = await initWasm(); }`              |

3. edit `pkg/speclang.d.ts` to make every wasm_bindgen method return a `Promise` result
4. build preferred speclang adapter(s)
