module Msg exposing (Msg(..))
import Browser exposing (UrlRequest)
import Url exposing (Url)
import Editor exposing (Editor)

type Msg
    = EditorChanged Editor
    | LinkClicked UrlRequest
    | UrlChanged Url


