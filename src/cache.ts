import { Key } from "../src-tauri/bindings/Key";

export const preferences = new Map();
export const paths = new Map();
export const icons = new Map();
export let keyDatas: Key[] = [];

export const FALLBACK_ICON_SYMBOL = Symbol();
