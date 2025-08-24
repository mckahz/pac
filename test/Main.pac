module Main [main];

import Task;
import List;

let main : Task ();
let main =
    Task.println
    <| to_string
    <| List.sum
    <| List.range 0 100;
