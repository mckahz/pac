module String [];

let join : String -> List String -> String;
let join sep strings =
    strings
    |> List.intersperse sep
    |> List.fold "" append;


let append : String -> String -> String;
let append left right = todo;
