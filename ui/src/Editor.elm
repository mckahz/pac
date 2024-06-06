module Editor exposing (..)

import Element as E exposing (Element)
import Util
import Element.Input as Input
import Msg exposing (Msg(..))


type Editor
    = Scene
    | Tileset
    | Animation
    | Dialogue
    | Debugger


toString : Editor -> String
toString editor =
    case editor of
        Scene ->
            "Scene Editor"

        Tileset ->
            "Tileset / Tilemap Editor"

        Animation ->
            "Animation Editor"

        Dialogue ->
            "Dialogue Editor"

        Debugger ->
            "Debugger"

viewEditorButton : Editor -> Element Msg -> Element Msg
viewEditorButton edit icon =
    Input.button []
        { onPress = Just (EditorChanged edit)
        , label =
            E.el
                [ if model.editor == edit then
                    Font.color Colors.highlight

                    else
                    Font.color Colors.text
                ]
                icon
        }

view : Editor -> Element msg
view editor =
    Util.window (toString editor) [ E.width <| E.fillPortion 2, E.height E.fill ] <|
        E.column [ E.width E.fill, E.height E.fill ]
            [ E.row [ E.width E.fill, E.spacing 10 ] <|
                [ viewEditorButton Editor.Scene Icons.scene
                , viewEditorButton Editor.Tileset Icons.tileset
                , viewEditorButton Editor.Animation Icons.animation
                , viewEditorButton Editor.Dialogue Icons.dialogue
                , viewEditorButton Editor.Debugger Icons.debug
                ]
            , E.el [ E.height E.fill ] E.none
            ]
