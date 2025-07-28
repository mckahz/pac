module Bool [];

let Bool = True | False;

let and : Bool -> Bool -> Bool;
let and lhs rhs =
    when lhs is
    | False -> False
    | True -> rhs;;


let or : Bool -> Bool -> Bool;
let or lhs rhs =
    when lhs is
    | True -> True
    | False -> rhs;;
