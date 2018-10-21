module Shared.Form.ImageInput exposing (imageInput)

import Html.Styled exposing (Html, Attribute, div, label, img, input, text)
import Html.Styled.Attributes exposing (..)
import Css exposing (..)
import Css.Foreign exposing (adjacentSiblings, selector)
import String.Extra exposing (dasherize)

import Shared.Css exposing (..)


imageInput : String -> Maybe String -> List (Attribute msg) -> List (Html msg)
imageInput inputName maybeImageSource attrs =
    let
        opaqueName = dasherize inputName
        image =
            case maybeImageSource of
                Just imageSource ->
                    img [ src imageSource, alt inputName, css imageStyle ] []
                Nothing -> div [] []
    in
        [ image
        , div [ css [ position relative  ] ]
            [ hiddenFileInput opaqueName attrs
            , fileInputLabel opaqueName inputName
            ]
        ]

hiddenFileInput : String -> List (Attribute msg) -> Html msg
hiddenFileInput opaqueName attrs =
    input
      (type_ "file" :: name opaqueName :: id opaqueName :: css imageInputStyle :: attrs) []

fileInputLabel : String -> String -> Html msg
fileInputLabel opaqueName inputName =
    label [ for opaqueName, css imageInputLabelStyle ] [ text inputName ]

imageStyle : List Style
imageStyle =
    [ maxWidth (pct 100)
    , border2 (baseUnit 2) solid
    , padding (baseUnit 10)
    ]

imageInputLabelStyle : List Style
imageInputLabelStyle =
    [ margin2 (baseUnit 20) (baseUnit 10)
    , Shared.Css.buttonStyle
    ]

imageInputStyle : List Style
imageInputStyle =
    [ accessibleText
    , focus
        [ adjacentSiblings
              [ selector "label"
                    [ color (hex "fff")
                    , backgroundColor (hex "#000")
                    ]
              ]
        ]
    ]

accessibleText : Style
accessibleText =
    Css.batch
        [ Css.property "clip" "rect(0 0 0 0)"
        , position absolute
        ]
