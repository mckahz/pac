module Util exposing (..)

import Colors
import Element as E exposing (Element)
import Element.Background as Background
import Element.Border as Border


hr : Element msg
hr =
    E.el
        [ E.width E.fill
        , E.height <| E.px 3
        , Background.color <| E.rgb 0.0 0.0 0.0
        ]
        E.none



-- TODO: unify window aesthetic


window : String -> List (E.Attribute msg) -> Element msg -> Element msg
window heading attrs body =
    E.column attrs
        [ E.text heading
            |> E.el [ Border.width 3, Border.color Colors.bg2 ]
            |> E.el
                [ Border.widthEach { top = 3, bottom = 0, left = 0, right = 0 }
                , Border.color Colors.highlight
                , Background.color Colors.bg2
                ]
        , body
            |> E.el
                [ E.width E.fill
                , E.height E.fill
                , Background.color Colors.bg
                , Border.widthEach { top = 3, bottom = 0, left = 0, right = 0 }
                , Border.color Colors.bg2
                ]
        ]
