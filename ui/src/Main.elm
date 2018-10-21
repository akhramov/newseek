module Main exposing (main)

import Html exposing (Html, program)

import Navigation exposing (Location)

import Model exposing (..)
import View exposing (view)
import Update exposing (update)
import Subscriptions exposing (subscriptions)
import Message exposing (Msg(..))

-- init
init : Location -> ( Model a, Cmd Msg )
init location =
    update (OnLocationChange location) Model.init

-- MAIN

main : Program Never (Model a) Msg
main =
    Navigation.program OnLocationChange
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
