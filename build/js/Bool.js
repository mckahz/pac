import { toString } from "./Basics.js";

const and = (lhs) => (rhs) => (() => {
    const __expr = lhs;
    const __functions = [
        () => (() => { return { tag: 1, arity: 0, args: [] }; })(),
        () => rhs
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

const or = (lhs) => (rhs) => (() => {
    const __expr = lhs;
    const __functions = [
        () => (() => { return { tag: 0, arity: 0, args: [] }; })(),
        () => rhs
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

