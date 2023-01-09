# speclang

## test

- `$ cargo test`

### options

|variable|value|reason|
|-|-|-|
|test command|`cargo test -- --nocapture`|print logs|
|env:RUST_TEST_THREADS|1|run tests sequentially|
|env:RUST_BACKTRACE|1|output full stack trace|

<!-- ## build wasm

- `$ wasm-pack build --target=bundler`
- (optional) build preferred speclang adapter

### JS build steps (todo: automate or move into adapter/javascript)

1. edit `pkg/speclang_bg.js`

| line                       | before                                        | after                                                  |
| -------------------------- | --------------------------------------------- | ------------------------------------------------------ |
| 1                          | `import * as wasm from './speclang_bg.wasm';` | `import initWasm from './speclang_bg.wasm'; let wasm;` |
| wasm_bindgen method export | `export function parse(input) {`              | `export async function parse(input) {`                 |
| wasm_bindgen method body   | insert at start of each method body           | `if (!wasm) { wasm = await initWasm(); }`              |

2. edit `pkg/speclang.d.ts` to make every wasm_bindgen method return a `Promise` result -->
