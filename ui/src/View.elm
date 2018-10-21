module View exposing (view)

import Html exposing (Html)
import Html.Styled as Styled

import Model exposing (Model)
import Message exposing (Msg(..))
import Routing as Route exposing (Route(..))

import Page exposing (..)
import Settings.View as Settings

view : Model a -> Html Msg
view model =
    chooseView model
        |> layout model.loading model.route model.settings
        |> Styled.toUnstyled

chooseView : Model a -> Styled.Html Msg
chooseView { settings, route } =
    case route of
        Route.Settings -> Styled.map SettingsMessage (Settings.view settings)
        _ -> Styled.map SettingsMessage (Settings.view settings)
