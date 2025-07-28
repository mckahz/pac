import { toString } from "./Basics.js";

export const evenHmm = (n) => ((n % 2) === 0);

export const oddHmm = (x) => not(evenHmm(x));

export const toString = (n) => n;

