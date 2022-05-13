# speclang-rs

## WebAssembly

### adapter/javascript (todo: organize docs)

#### build (todo: automate)

1. `$ wasm-pack build --target=bundler`
2. edit `pkg/speclang_bg.js`

| line                       | before                                        | after                                                  |
| -------------------------- | --------------------------------------------- | ------------------------------------------------------ |
| 1                          | `import * as wasm from './speclang_bg.wasm';` | `import initWasm from './speclang_bg.wasm'; let wasm;` |
| wasm_bindgen method export | `export function parse(input) {`              | `export async function parse(input) {`                 |
| wasm_bindgen method body   | insert at start of each method body           | `if (!wasm) { wasm = await initWasm(); }`              |

3. edit `pkg/speclang.d.ts` to make every wasm_bindgen method `async`

4. `$ cd adapter/javascript && npm run build`
5. Tests
   5a. From `test/node`, run `$ npm t`
   5b. From `test/browser`, run `$ npm run dev`

#### release

6. `$ npm publish`
