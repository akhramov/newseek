module Update exposing (update)

import Navigation exposing (Location)

import Message exposing (Msg(..))
import Model exposing (..)
import Routing exposing (parseLocation)

import Settings.Update as Settings
import Settings.Message as SettingsMessage

update : Msg -> Model a -> ( Model a, Cmd Msg )
update msg model =
    case msg of
        NoOp ->
            ( model, Cmd.none )
        OnLocationChange location ->
            setRoute location model
        SettingsMessage message ->
            let
                isLoading = case message of
                                SettingsMessage.SettingsUpdated _ -> False
                                SettingsMessage.SaveButtonClicked -> True
                                _ -> model.loading

                (updatedSettings, cmd) = Settings.update message model.settings
            in
                 ( { model | settings = updatedSettings, loading = isLoading }
                 , Cmd.map SettingsMessage cmd
                 )


setRoute : Location -> Model b -> (Model b, Cmd Msg)
setRoute location model =
    let
        route = parseLocation location

        cmd = case route of
                  Routing.Settings -> Cmd.map SettingsMessage (Settings.loadSettings)
                  _ -> Cmd.none
    in
        ({ model | route = route }, cmd)
