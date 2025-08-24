import { toString } from "./Basics.js";
import * as List from "./List.js";

const join = (sep) => (strings) => List.fold("")(append)(List.intersperse(sep)(strings));

const append = (left) => (right) => todo;

