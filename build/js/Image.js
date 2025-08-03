import { toString } from "./Basics.js";
import * as Task from "./Task.js";

export const width = (image) => (() => {
    const __expr = image;
    const __functions = [
        (__arg0,w,__arg2) => w
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const height = (image) => (() => {
    const __expr = image;
    const __functions = [
        (__arg0,__arg1,h) => h
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

const load = Image.load;

