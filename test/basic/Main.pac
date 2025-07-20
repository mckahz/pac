module Main [];

import List;
import Task;

let main : Task ();
let main =
    (if 1 == 2 then List.range 0 100 else [7, 8, 9])
    |> List.filter Num.odd?
    |> Task.println;
