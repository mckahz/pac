module List
    [ List
    , map
    , keep_if
    , drop_if
    , walk
    , walk_backwards
    , reverse
    , sum
    , product
    , first
    , last
    , rest
    , init
    , repeat
    , range
    ];

let List a =
    | Empty
    | Cons a (List a);

let first : List a -> a;
let first xs =
    when xs is
    | [] -> crash "cannot get head of empty list"
    | x::_ -> x;;

let last : List a -> a;
let last xs =
    when xs is
    | [] -> crash "cannot get first element of empty list"
    | x::_ -> x;;


let rest : List a -> List a;
let rest xs =
    when xs is
    | [] -> crash "cannot get rest of empty list"
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

let repeat : Int -> a -> List a;
let repeat n x = range 0 n |> map (\_ -> x);

let range : Int -> Int -> List Int;
let range lo hi = range_help [] lo hi;

let range_help : List Int -> Int -> Int -> List Int;
let range_help acc lo hi =
    if lo > hi then
        acc
    else
        range_help (hi :: acc) lo (hi - 1);
