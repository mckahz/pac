import { toString } from "./Basics.js";
import * as Task from "./Task.js";
import * as Num from "./Num.js";

const load = Image.load;

export const width = (image) => (() => {
    const __expr = image;
    const __functions = [
        (_,w,_) => w
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

export const height = (image) => (() => {
    const __expr = image;
    const __functions = [
        (_,_,h) => h
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();

