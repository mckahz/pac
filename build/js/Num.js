import { toString } from "./Basics.js";
import * as Bool from "./Bool.js";

export const even__hmmm = (n) => ((n % 2) === 0);

export const odd__hmmm = (__arg) => Bool.not(even__hmmm(Arg));

