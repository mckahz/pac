import { toString } from "./Basics.js";

export const first = (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => console.error("cannot get head of empty list"),
        (x,__arg1) => x
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const last = (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => console.error("cannot get first element of empty list"),
        (x,__arg1) => x
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const rest = (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => console.error("cannot get rest of empty list"),
        (__arg0,xs) => xs
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

const empty__hmmm = (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => (() => { return { tag: 0, arity: 0, args: [] }; })(),
        (__wildcard) => (() => { return { tag: 1, arity: 0, args: [] }; })()
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const walk = (init) => (f) => (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => init,
        (y,ys) => walk(f(y)(init))(f)(ys)
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const reverse = walk((() => { return { tag: 0, arity: 0, args: [] }; })())((x) => (acc) => ((__arg0) => (__arg1) =>  { return { tag: 1, arity: 2, args: [__arg0, __arg1] }; })(x)(acc));

export const walkBackwards = (init) => (f) => (xs) => reverse(walk(init)(f)(xs));

export const map = (f) => walkBackwards((() => { return { tag: 0, arity: 0, args: [] }; })())((x) => (acc) => ((__arg0) => (__arg1) =>  { return { tag: 1, arity: 2, args: [__arg0, __arg1] }; })(f(x))(acc));

export const keepIf = (p) => walkBackwards((() => { return { tag: 0, arity: 0, args: [] }; })())((x) => (acc) => (p(x) ? ((__arg0) => (__arg1) =>  { return { tag: 1, arity: 2, args: [__arg0, __arg1] }; })(x)(acc) : acc));

export const dropIf = (p) => keepIf((__arg) => not(p(Arg)));

export const sum = walk(0)((a) => (b) => (a + b));

export const product = walk(0)((a) => (b) => (a * b));

export const repeat = (n) => (x) => map((__wildcard) => x)(range(0)(n));

export const range = (lo) => (hi) => rangeHelp((() => { return { tag: 0, arity: 0, args: [] }; })())(lo)(hi);

const range2 = (lo) => (hi) => ((lo < hi) ? ((__arg0) => (__arg1) =>  { return { tag: 1, arity: 2, args: [__arg0, __arg1] }; })(lo)(range2((lo + 1))(hi)) : (() => { return { tag: 0, arity: 0, args: [] }; })());

const rangeHelp = (acc) => (lo) => (hi) => ((lo > hi) ? acc : rangeHelp(((__arg0) => (__arg1) =>  { return { tag: 1, arity: 2, args: [__arg0, __arg1] }; })(hi)(acc))(lo)((hi - 1)));

