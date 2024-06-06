module Scene exposing (..)

import Basics.Extra exposing (..)
import Element as E exposing (Element)
import Element.Font as Font
import List.Extra as List
import Util


type alias Scene =
    { name : String
    , hierarchy : Hierarchy
    }


type Hierarchy
    = Entity String
    | Group String (List Hierarchy)


example : Scene
example =
    { name = "example"
    , hierarchy =
        Group "root"
            [ Group "backgrounds"
                [ Entity "mountains"
                , Entity "trees"
                , Entity "foliage"
                ]
            , Entity "foreground-tilemap"
            , Entity "player"
            , Group "enemies"
                [ Entity "enemy1"
                , Entity "enemy2"
                , Entity "enemy3"
                ]
            , Entity "decal-tilemap"
            ]
    }


toString : Hierarchy -> String
toString hierarchy =
    case hierarchy of
        Entity name ->
            name

        Group name children ->
            let
                lines =
                    children
                        |> List.map toString
                        |> List.concatMap (String.split "\n")

                gutters =
                    if List.isEmpty lines then
                        []

                    else
                        List.repeat (List.length lines - 1) "│" ++ [ "└" ]
            in
            List.zip gutters lines
                |> List.map (uncurry String.append)
                |> String.join "\n"
                |> String.append (name ++ "\n")


viewHierarchySelector : Scene -> Element msg
viewHierarchySelector scene =
    scene.hierarchy
        |> toString
        |> E.text
        |> E.el [ Font.family [ Font.monospace ] ]
        |> Util.window "Scene Hierarchy" [ E.width E.fill, E.height E.fill ]
