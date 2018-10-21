module Subscriptions exposing (subscriptions)

import Model exposing (..)
import Message exposing (Msg(..))
import Settings.Subscriptions as Settings

subscriptions : Model a -> Sub Msg
subscriptions model =
    Sub.batch
        [ Sub.map SettingsMessage Settings.subscriptions
        ]
