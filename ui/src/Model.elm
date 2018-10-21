module Model exposing (..)

import Settings.Model as Settings
import User.Model as User

import Message exposing (Msg(..))

import Routing exposing (Route(..))

type alias Model a =
    { settings : Maybe Settings.Model
    , users : List (User.Model a)
    , route : Route
    , loading : Bool
    }

init : Model a
init = { settings = Nothing, users = [], route = Main, loading = False }
