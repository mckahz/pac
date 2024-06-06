module FS exposing (..)

import Basics.Extra exposing (..)
import Element as E exposing (Element)
import Element.Font as Font
import List.Extra as List
import Util


type FS
    = Dir String (List FS)
    | File String


selector : Element msg
selector =
    example
        |> toString
        |> E.text
        |> E.el [ Font.family [ Font.monospace ] ]
        |> Util.window "File System" [ E.width E.fill, E.height E.fill ]


example : FS
example =
    Dir "project"
        [ Dir "src"
            [ File "main.vc" ]
        , Dir "res"
            [ File "character.png"
            , File "enemy.png"
            , File "bg.png"
            , File "ts.png"
            ]
        , File "icon.png"
        , File "settings.vc"
        ]


toString : FS -> String
toString fs =
    case fs of
        File name ->
            name

        Dir name subdirs ->
            let
                subdirLines =
                    subdirs
                        |> List.map toString
                        |> List.concatMap (String.split "\n")

                gutters =
                    if List.isEmpty subdirLines then
                        []

                    else
                        List.repeat (List.length subdirLines - 1) "│" ++ [ "└" ]
            in
            List.zip gutters subdirLines
                |> List.map (uncurry String.append)
                |> String.join "\n"
                |> String.append (name ++ "\n")
