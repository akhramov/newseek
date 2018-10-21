module Shared.Form exposing (group, input, formLabel, textarea, button)

import Html.Styled as Html exposing (Html, Attribute, a, div, text, label, fieldset, img)
import Html.Styled.Attributes as Attr exposing (..)
import Css exposing (..)
import Css.Foreign exposing (adjacentSiblings, selector)
import Css.Transitions as Transitions exposing (transition)
import String.Extra exposing (toSentenceCase, dasherize)

import Shared.Css exposing (..)


group : String -> List (Html msg) -> Html msg
group title content  =
    fieldset [ css fieldsetStyle ]
        <| legend title :: content

legend : String -> Html msg
legend title =
    Html.legend [ css legendStyle ] [ text title ]

input : List (Attribute msg) -> (String, String) -> Html msg
input attrs (inputName, inputType) =
    Html.input
        |> floatingLabelElement inputName (css inputStyle :: type_ inputType :: attrs)
        |> div [ css inputWrapperStyle ]

textarea : String -> List (Attribute msg) -> Html msg
textarea title attrs =
    Html.textarea
        |> floatingLabelElement title (css textareaStyle :: attrs)
        |> div [ css textareaWrapperStyle ]


floatingLabelElement : String -> List (Attribute msg)
                     -> (List (Attribute msg) -> List (Html msg) -> Html msg)
                     -> List (Html msg)
floatingLabelElement elementName attributes ctor =
    let
        opaqueName =
            dasherize elementName
        attrs =
            List.append attributes [ id opaqueName
                                   , name opaqueName
                                   , Attr.required True
                                   ]
    in
        [ ctor attrs [], formLabel opaqueName elementName ]


formLabel : String -> String -> Html msg
formLabel opaqueName inputName =
    label [ for opaqueName, css labelStyle ] [ text <| toSentenceCase inputName ]

button : String -> List (Attribute msg) -> Html msg
button title attributes =
    let
        attrs = List.append attributes [ css buttonStyle ]
    in
        Html.button attrs [ text title ]

textareaWrapperStyle : List Style
textareaWrapperStyle =
    [ position relative
    , margin2 (baseUnit 20) (baseUnit 10)
    ]

buttonStyle : List Style
buttonStyle =
    [ margin4 (px 0) (px 0) (baseUnit 40) auto
    , Shared.Css.buttonStyle
    ]

labelStyle : List Style
labelStyle =
    [ position absolute
    , top (baseUnit 15.75)
    , left (baseUnit 17)
    , fontSize (baseUnit 18)
    , color (hex "#777")
    , transition [ Transitions.top 200, Transitions.fontSize 200 ]
    , zIndex (int -1)
    ]

inputStyle : List Style
inputStyle =
    [ Css.height (baseUnit 50)
    , Css.width (baseUnit 240)
    , padding3 (baseUnit 10) (baseUnit 15) (px 0)
    , textEntryStyle
    ]


inputWrapperStyle : List Style
inputWrapperStyle =
    [ position relative
    , margin2 (baseUnit 20) (baseUnit 10)
    ]


textareaStyle : List Style
textareaStyle =
    [ Css.width (baseUnit 400)
    , minWidth (baseUnit 240)
    , minHeight (baseUnit 80)
    , padding3 (baseUnit 20) (baseUnit 15) (px 0)
    , textEntryStyle
    ]

textEntryStyle : Style
textEntryStyle =
    Css.batch [ fontSize (baseUnit 16)
              , border2 (baseUnit 2) solid
              , borderColor (hex "#777")
              , backgroundColor transparent
              , outline none
              , boxShadow none
              , valid [ labelActiveStyle ]
              , focus
                    [ borderColor (hex "#000")
                    , labelActiveStyle
                    ]
              ]

labelActiveStyle : Style
labelActiveStyle =
    adjacentSiblings [ selector "label"
                           [ fontSize (baseUnit 14)
                           , top (baseUnit 5)
                           ]
                     ]


fieldsetStyle : List Style
fieldsetStyle =
    [ border3 (baseUnit 2) solid (hex "#000")
    , margin2 (baseUnit 30) (px 0)
    ]

legendStyle : List Style
legendStyle =
    [ fontSize <| baseUnit 30 ]
