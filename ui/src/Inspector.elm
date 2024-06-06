module Inspector exposing (..)

import Element as E exposing (Element)
import Util

entity : Element msg
entity =
    E.column []
        [ E.text "Properties"
        , E.text "health"
        , E.text "position: x, y"
        , E.text "velocity: x, y"
        ]
