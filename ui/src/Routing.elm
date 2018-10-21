module Routing exposing (..)

import Navigation exposing (Location)
import UrlParser exposing (..)

type Route
    = Main
    | Transfers String
    | Chats (Maybe String)
    | Settings
    | About
    | NotFound


toString : Route -> String
toString route =
    case route of
        Settings -> "#settings"
        _ -> ""

matchers : Parser (Route -> a) a
matchers =
    oneOf
    [ map Main top
    , map Settings (s "settings")
    ]

parseLocation : Location -> Route
parseLocation location =
    case (parseHash matchers location) of
        Just route ->
            route

        Nothing ->
            NotFound
