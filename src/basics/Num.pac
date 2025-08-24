module Num [Nat, Int, Float, even?, odd?, to_string];

import Bool;

let Nat = extern "Nat";
let Int = extern "Int";
let Float = extern "Float";

let even? : Int -> Bool.Bool;
let even? n = n % 2 == 0;

let odd? : Int -> Bool.Bool;
let odd? = Bool.not << even?;
