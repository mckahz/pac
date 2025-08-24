import { toString } from "./Basics.js";

const and = (lhs) => (rhs) => (() => {
    const __expr = lhs;
    const __functions = [
        () => .False,
        () => rhs
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

const not = (bool) => (() => {
    const __expr = bool;
    const __functions = [
        () => .False,
        () => .True
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

const or = (lhs) => (rhs) => (() => {
    const __expr = lhs;
    const __functions = [
        () => .True,
        () => rhs
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

