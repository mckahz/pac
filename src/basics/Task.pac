module Task [Task, println];

import String;

let Task a = extern "Task";

let println : String.String -> Task ();
let println = extern "println";
