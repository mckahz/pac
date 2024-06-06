module Colors exposing (..)

import Element as E
import List.Extra as List
import String.Extra as String
import Basics.Extra exposing (uncurry)
import Basics.Extra exposing (flip)


fromHexChar : Char -> Int
fromHexChar char =
    case char |> String.fromChar |> String.toInt of
        Just digit ->
            digit

        Nothing ->
            case Char.toLower char of
                'a' ->
                    10

                'b' ->
                    11

                'c' ->
                    12

                'd' ->
                    13

                'e' ->
                    14

                'f' ->
                    15

                _ ->
                    0


hex : String -> E.Color
hex string =
    let
        hexDigits =
            string
                |> String.toList
                |> List.map fromHexChar

        dropEvery2 =
            List.indexedMap (\i e -> if (i |> modBy 2) == 0 then [e] else [])
            >> List.concat

        sixteens =
            hexDigits
            |> dropEvery2
            |> List.map ((*) 16)

        ones =
            hexDigits
            |> List.drop 1
            |> dropEvery2


        pairs =
            List.zip sixteens ones
            |> List.map (uncurry (+) >> toFloat >> flip (/) 255)
    in
    case pairs of
        r::g::b::[] -> E.rgb r g b
        r::g::b::a::[] -> E.rgb r g b
        _ -> E.rgb 1 1 1


bg : E.Color
bg =
    hex "2c3335"


bg2 : E.Color
bg2 =
    hex "626d71"


highlight : E.Color
highlight =
    hex "0883e2"


text : E.Color
text =
    hex "dee5e8"
