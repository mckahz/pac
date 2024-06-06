module Main exposing (..)

import Basics.Extra exposing (..)
import Browser exposing (UrlRequest(..))
import Browser.Navigation as Nav
import Collage as C
import Collage.Render as R
import Color
import Colors
import Element as E exposing (Element)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font exposing (Font)
import Element.Input as Input
import Element.Region as Region
import FS
import FeatherIcons as Icon
import Html
import Icons
import Inspector
import List.Extra as List
import Scene
import String.Extra as String
import Url exposing (Url)
import Util
import Editor exposing (Editor(..))
import Msg exposing (Msg(..))



type alias Model =
    { editor : Editor
    }


main : Program () Model Msg
main =
    Browser.application
        { subscriptions = \_ -> Sub.none
        , onUrlChange = UrlChanged
        , onUrlRequest = LinkClicked
        , init = init
        , view =
            \model ->
                { title =
                    "Virtual Console"
                , body =
                    [ E.layoutWith
                        { options =
                            [ E.focusStyle
                                { borderColor = Nothing
                                , backgroundColor = Nothing
                                , shadow = Nothing
                                }
                            ]
                        }
                        []
                      <|
                        view model
                    ]
                }
        , update = update
        }


init : flags -> Url -> Nav.Key -> ( Model, Cmd msg )
init _ url key =
    ( { editor = Tileset }, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        LinkClicked _ ->
            ( model, Cmd.none )

        UrlChanged _ ->
            ( model, Cmd.none )

        EditorChanged editor ->
            ( { model | editor = editor }, Cmd.none )


view : Model -> Element Msg
view model =
    let
        selector =
            E.column [ E.width <| E.fillPortion 1, E.height E.fill, E.spacing 10 ]
                [ Scene.viewHierarchySelector Scene.example
                , FS.selector
                ]

        inspector =
            Util.window "Inspector" [ E.width <| E.fillPortion 1, E.height E.fill ] <|
                E.column [ E.width E.fill, E.height E.fill ] [ Inspector.entity ]

    in
    E.column [ E.width E.fill, E.height E.fill, Background.color Colors.bg, Font.color Colors.text ]
        -- NOTE: Navbar
        [ E.row [ E.width E.fill, E.padding 10 ]
            [ [Icons.settings, Icons.play]
                |> E.row [E.alignRight]
            ]
            --|> List.map (E.el [ E.alignRight ])

        -- NOTE: Main Panel
        , E.row [ E.width E.fill, E.height E.fill, E.spacing 10 ]
            [ selector
            , Editor.view model.editor
            , inspector
            ]
        ]

