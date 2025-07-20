const head = (xs) => (() => {
                    const expr = xs;
                    const f = (x) => x;
                    return f(expr)
                })();
const tail = (xs) => (() => {
                    const expr = xs;
                    const f = (x) => x;
                    return f(expr)
                })();
const length = ((walk)(0))((_) => (s) => (s + 1));
const emptyHmm = (xs) => (() => {
                    const expr = xs;
                    const f = (x) => x;
                    return f(expr)
                })();
const walk = (init) => (f) => (xs) => (() => {
                    const expr = xs;
                    const f = (x) => x;
                    return f(expr)
                })();
const reverse = ((walk)([]))((x) => (acc) => [x] + acc);
const walkBackwards = (init) => (f) => (xs) => (reverse)((((walk)(init))(f))(xs));
const map = (f) => ((walkBackwards)([]))((x) => (acc) => [(f)(x)] + acc);
const filter = (p) => ((walkBackwards)([]))((x) => (acc) => ((p)(x) ? [x] + acc : acc));
const sum = ((walk)(0))((a) => (b) => (a + b));
