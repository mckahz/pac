module Icons exposing (..)

import Element exposing (Element)
import FeatherIcons as F


icon : F.Icon -> Element msg
icon =
    F.toHtml [] >> Element.html

settings : Element msg
settings = icon F.settings

play : Element msg
play = icon F.play

scene : Element msg
scene = icon F.film

tileset : Element msg
tileset = icon F.map

animation : Element msg
animation = icon F.fastForward

dialogue : Element msg
dialogue = icon F.messageSquare

console : Element msg
console = icon F.clipboard

debug : Element msg
debug = icon F.cpu
