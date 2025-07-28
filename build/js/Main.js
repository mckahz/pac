import { toString } from "./Basics.js";
import * as Task from "./Task.js";
import * as List from "./List.js";

export const main = Task.println(toString(List.sum(List.range(0)(100))));

