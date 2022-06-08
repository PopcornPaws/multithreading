declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* Entry point invoked by `worker.js`, a bit of a hack but see the "TODO" above
	* about `worker.js` in general.
	* @param {number} ptr
	*/
	export function child_entry_point(ptr: number): void;
	/**
	*/
	export class Text {
	  free(): void;
	/**
	* @param {string} inner
	*/
	  constructor(inner: string);
	/**
	* @param {number} concurrency
	* @param {string} input
	* @param {WorkerPool} pool
	* @returns {Promise<any>}
	*/
	  process(concurrency: number, input: string, pool: WorkerPool): Promise<any>;
	}
	/**
	*/
	export class WorkerPool {
	  free(): void;
	/**
	* Creates a new `WorkerPool` which immediately creates `initial` workers.
	*
	* The pool created here can be used over a long period of time, and it
	* will be initially primed with `initial` workers. Currently workers are
	* never released or gc'd until the whole pool is destroyed.
	*
	* # Errors
	*
	* Returns any error that may happen while a JS web worker is created and a
	* message is sent to it.
	* @param {number} initial
	*/
	  constructor(initial: number);
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly __wbg_workerpool_free: (a: number) => void;
  readonly workerpool_new: (a: number, b: number) => void;
  readonly child_entry_point: (a: number, b: number) => void;
  readonly __wbg_text_free: (a: number) => void;
  readonly text_new: (a: number, b: number) => number;
  readonly text_process: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he4fe4797b3fcbeb0: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1f4ee2908696676b: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h63e8bb1af0e32b7e: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_thread_destroy: () => void;
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
