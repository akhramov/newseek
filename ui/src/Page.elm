module Page exposing (layout)

import Html.Styled exposing (Html, div, text, a)
import Html.Styled.Attributes exposing (css, href, class)
import Css exposing (..)

import Shared.Css exposing (baseUnit, baseUnitRaw, buttonHoverStyle)
import User.Model as User
import Routing as Route exposing (Route(..))

layout : Bool -> Route -> Maybe (User.Model a) -> Html msg -> Html msg
layout isLoading activeRoute user content =
    div [css layoutStyle, class "App"] <|
        if isLoading then
            []
        else
            [ viewHeader user activeRoute
            , viewContent content
            ]

layoutStyle : List Style
layoutStyle =
    [ fontFamily sansSerif
    , fontSize (px 16)
    , height (pct 100)
    , property "display" "grid"
    , property "grid-template-columns" "1fr"
    , property "grid-template-rows" <| String.join " " [baseUnitRaw 40, "auto", baseUnitRaw 30]
    ]


viewHeader : Maybe (User.Model a) -> Route -> Html msg
viewHeader user activeRoute =
    div [ css headerWrapperStyle ]
        [ div [ css headerStyle ] <| headerItems activeRoute ]


headingLinks : List (Route, String)
headingLinks =
    [ (Route.Transfers "downloads", "Transfers")
    , (Route.Chats Nothing, "Chats")
    , (Route.Settings, "Settings")
    , (Route.About, "About")
    ]

headerItems : Route -> List (Html msg)
headerItems activeRoute =
    let
        foo = headerItem activeRoute
    in
        List.map foo headingLinks



headerItem : Route -> (Route, String) -> (Html msg)
headerItem activeRoute (route, heading) =
    case activeRoute == route of
        True ->
            div [ css headerActiveItem ]
                [ text heading ]
        False ->
            a [ href <| Route.toString route , css headerLink ] [ text heading ]

headerActiveItem : List Style
headerActiveItem =
    [ verticalAlignCenter
    , padding2 (px 0) (baseUnit 20)
    , height (pct 100)
    , backgroundColor (hex "#000")
    , color (hex "#fff")
    ]


headerLink : List Style
headerLink =
    [ verticalAlignCenter
    , padding2 (px 0) (baseUnit 20)
    , height (pct 100)
    , buttonHoverStyle
    ]


headerWrapperStyle: List Style
headerWrapperStyle =
    [ borderBottom3 (baseUnit 2) solid (hex "#000")
    , height <| baseUnit 40
    ]

headerStyle : List Style
headerStyle =
    [ fontSize (baseUnit 24)
    , verticalAlignCenter
    , contentWrapper
    ]

viewContent : Html msg -> Html msg
viewContent content =
    div [css contentStyle] [content]

contentStyle : List Style
contentStyle = [ contentWrapper ]

contentWrapper : Style
contentWrapper =
    Css.batch
        [ padding2 (px 0) (baseUnit 10)
        , height (pct 100)
        , maxWidth (baseUnit 1200)
        , width (pct 100)
        , marginLeft auto
        , marginRight auto
        ]

verticalAlignCenter : Style
verticalAlignCenter =
    Css.batch
        [ displayFlex
        , alignItems center
        ]
