/* tslint:disable */
/* eslint-disable */
declare module "*.wasm" {
  export const memory: WebAssembly.Memory;
}

declare module "*.wasm?init" {
  const initWASM: (options: WebAssembly.Imports) => Promise<WebAssembly.Instance>;
  export default initWASM;
}
