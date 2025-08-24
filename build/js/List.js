import { toString } from "./Basics.js";
import * as Bool from "./Bool.js";

export const repeat = (n) => (x) => map((__wildcard) => x)(range(0)(n));

export const walkBackwards = (init) => (f) => (xs) => reverse(walk(init)(f)(xs));

export const rest = (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => crash("cannot get rest of empty list"),
        (_,xs) => xs
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const dropIf = (p) => keepIf((__arg) => not(p(Arg)));

export const keepIf = (p) => walkBackwards((() => { return { tag: 0, arity: 0, args: [] }; })())((x) => (acc) => (p(x) ? ((__arg0) => (__arg1) =>  { return { tag: 1, arity: 2, args: [__arg0, __arg1] }; })(x)(acc) : acc));

export const reverse = walk((() => { return { tag: 0, arity: 0, args: [] }; })())((x) => (acc) => ((__arg0) => (__arg1) =>  { return { tag: 1, arity: 2, args: [__arg0, __arg1] }; })(x)(acc));

const rangeHelp = (acc) => (lo) => (hi) => ((lo > hi) ? acc : rangeHelp(((__arg0) => (__arg1) =>  { return { tag: 1, arity: 2, args: [__arg0, __arg1] }; })(hi)(acc))(lo)((hi - 1)));

const empty__hmmm = (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => Bool.True,
        (_,_) => Bool.False
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const range = (lo) => (hi) => rangeHelp((() => { return { tag: 0, arity: 0, args: [] }; })())(lo)(hi);

export const sum = walk(0)((a) => (b) => (a + b));

export const walk = (init) => (f) => (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => init,
        (y,ys) => walk(f(y)(init))(f)(ys)
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const first = (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => crash("cannot get head of empty list"),
        (x,_) => x
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const map = (f) => walkBackwards((() => { return { tag: 0, arity: 0, args: [] }; })())((x) => (acc) => ((__arg0) => (__arg1) =>  { return { tag: 1, arity: 2, args: [__arg0, __arg1] }; })(f(x))(acc));

export const last = (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => crash("cannot get first element of empty list"),
        (x,_) => x
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const product = walk(0)((a) => (b) => (a * b));

