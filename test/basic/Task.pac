module Task [Task, println];

let Task a = extern "Task";

let println : String -> Task ();
let println = extern "println";
