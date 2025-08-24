module Image [Image, width, height];

import Task;
import Num;

let Image = Image String Int Int;

let width : Image -> Num.Int;
let width image =
    when image is
    | Image _ w _ -> w;;

let height : Image -> Num.Int;
let height image =
    when image is
    | Image _ _ h -> h;;

let load : a -> Task.Task String -> Task.Task Image;
let load = extern "Image.load";
