module Num [Nat, Int, Float, even?, odd?, to_string];

let Nat = extern "Nat";
let Int = extern "Int";
let Float = extern "Float";

let even? : Int -> Bool;
let even? n = n % 2 == 0;

let odd? : Int -> Bool;
let odd? = not << even?;

let to_string : Int -> String;
let to_string n = n;
