module Shared.Css exposing (..)

import Css exposing (..)
import Css.Transitions as Transitions exposing (transition)


baseUnit : Float -> Rem
baseUnit number =
    number / 16 |> Css.rem

baseUnitRaw : Float -> String
baseUnitRaw number =
    (number / 16 |> toString) ++ "rem"

buttonStyle : Style
buttonStyle =
    Css.batch
        [ Css.height (baseUnit 40)
        , Css.width (baseUnit 180)
        , padding2 (px 0) (baseUnit 15)
        , fontSize (baseUnit 18)
        , border2 (baseUnit 2) solid
        , backgroundColor transparent
        , borderColor (hex "#000")
        , color (hex "#000")
        , cursor pointer
        , displayFlex
        , justifyContent center
        , alignItems center
        , buttonHoverStyle
        ]

buttonHoverStyle : Style
buttonHoverStyle =
    Css.batch
        [ color (hex "#000")
        , textDecoration none
        , transition [ Transitions.backgroundColor 200
                     , Transitions.color 200
                     ]
        , hover [ backgroundColor (hex "#000")
                , color (hex "#fff")
                ]
        , focus [ backgroundColor (hex "#000")
                , color (hex "#fff")
                ]
        ]
