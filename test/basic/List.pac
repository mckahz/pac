module List [List, walk, walk_backwards, map, keep_if, sum, head, tail];

let List a =
    | Empty
    | Cons a (List a);

let head : List a -> a;
let head xs =
    when xs is
    | [] -> crash "cannot get head of empty list"
    | x::_ -> x;;

let tail : List a -> List a;
let tail xs =
    when xs is
    | [] -> crash "cannot get tail of empty list"
    | _::xs -> xs;;

let empty? : List a -> Bool;
let empty? xs =
    when xs is
    | [] -> True
    | _ -> False;;

let walk : acc -> (a -> acc -> acc) -> List a -> acc;
let walk init f xs =
    when xs is
    | [] -> init
    | y::ys -> walk (f y init) f ys;;

let reverse : List a -> List a;
let reverse = walk [] (\x acc -> x :: acc);

let walk_backwards : acc -> (a -> acc -> acc) -> List a -> acc;
let walk_backwards init f xs = reverse (walk init f xs);

let map : (a -> b) -> List a -> List b;
let map f =
    walk_backwards [] (\x acc -> f x :: acc);

let keep_if : (a -> Bool) -> List a -> List a;
let keep_if p = walk_backwards [] (\x acc ->
        if p x then
            x :: acc
        else
            acc
    );

let drop_if : (a -> Bool) -> List a -> List a;
let drop_if p = keep_if (not << p);

let sum : List Int -> Int;
let sum = walk 0 (\a b -> a + b);

let product : List Int -> Int;
let product = walk 0 (\a b -> a * b);
