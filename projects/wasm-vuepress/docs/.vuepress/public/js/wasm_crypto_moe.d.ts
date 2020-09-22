/* tslint:disable */
/* eslint-disable */
/**
* @param {string} text
* @returns {string}
*/
export function marysue_encode(text: string): string;
/**
* @param {string} text
* @returns {string}
*/
export function marysue_decode(text: string): string;
/**
* @param {string} text
* @returns {string}
*/
export function aaencode_encode(text: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly marysue_encode: (a: number, b: number, c: number) => void;
  readonly marysue_decode: (a: number, b: number, c: number) => void;
  readonly aaencode_encode: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        