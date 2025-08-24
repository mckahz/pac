module Basics [to_string];

import String;

let to_string : a -> String.String;
let to_string = extern "to_string";
