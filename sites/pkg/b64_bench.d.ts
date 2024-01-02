declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	*/
	export function start(): void;
	/**
	* @param {number} iterations
	* @param {number} b64_length
	* @returns {number}
	*/
	export function generate_b64(iterations: number, b64_length: number): number;
	/**
	* @param {number} iterations
	* @param {number} b64_length
	* @returns {number}
	*/
	export function bench(iterations: number, b64_length: number): number;
	/**
	* Entry point for web workers
	* @param {number} ptr
	*/
	export function wasm_thread_entry_point(ptr: number): void;
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly start: () => void;
  readonly generate_b64: (a: number, b: number) => number;
  readonly bench: (a: number, b: number) => number;
  readonly wasm_thread_entry_point: (a: number) => void;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly wasm_bindgen__convert__closures__invoke1_mut_ref__he39045b41b0f04f3: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_thread_destroy: (a?: number, b?: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>, maybe_memory?: WebAssembly.Memory): Promise<InitOutput>;
