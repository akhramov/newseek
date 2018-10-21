module Settings.Update exposing (update, loadSettings)

import Json.Encode exposing (object, encode)
import Json.Decode exposing (int, string, nullable, decodeString, Decoder)
import Json.Decode.Pipeline exposing (decode, required, optional, hardcoded)
import Http

import Settings.Message exposing (Msg(..))
import Settings.Model exposing (..)

import Shared.Port exposing (getPicture)

update : Msg -> Maybe Model -> ( Maybe Model, Cmd Msg )
update msg maybeModel =
    case msg of
        SettingsUpdated (Ok newModel) ->
            (Just newModel, Cmd.none)
        _ ->
            case maybeModel of
                Nothing -> (maybeModel, Cmd.none)
                Just model -> updateFunc msg model |> Tuple.mapFirst Just


updateFunc : Msg -> Model -> ( Model, Cmd Msg )
updateFunc msg model =
    case msg of
        UsernameChanged username ->
            ({ model | username = username }, Cmd.none)
        PasswordChanged password ->
            ({ model | password = password }, Cmd.none)
        InfoChanged info ->
            ({ model | info = Just info }, Cmd.none)
        PictureChanged value ->
            (model, getPicture value)
        PictureUpdated picture ->
            ({ model | picture = Just picture }, Cmd.none)
        DownloadRateChanged rate ->
            let
                updatedRate = parseString rate model.downloadRate
            in
                ({ model | downloadRate = updatedRate }, Cmd.none)
        DownloadSlotsChanged slots ->
            let
                updatedSlots = parseString slots model.downloadSlots
            in
                ({ model | downloadSlots = updatedSlots }, Cmd.none)
        UploadRateChanged rate ->
            let
                updatedRate = parseString rate model.uploadRate
            in
                ({ model | uploadRate = updatedRate }, Cmd.none)
        UploadSlotsChanged slots ->
            let
                updatedSlots = parseString slots model.uploadSlots
            in
                ({ model | uploadSlots = updatedSlots }, Cmd.none)
        SaveButtonClicked ->
            (model, updateSettings model)
        SettingsUpdated (Ok newModel) ->
            (newModel, Cmd.none)
        SettingsUpdated (Err error) ->
            (model, Cmd.none) -- TODO: Error handling

parseString : String -> Int -> Int
parseString string default =
    case String.isEmpty string of
        True -> 0
        False ->
            Result.withDefault default (String.toInt string)

updateSettings : Model -> Cmd Msg
updateSettings model =
    encodeSettings model
        |> Http.stringBody "application/json"
        |> flip (Http.post "/api/settings") settingsDecoder
        |> Http.send SettingsUpdated

loadSettings : Cmd Msg
loadSettings =
    Http.get "/api/settings" settingsDecoder
        |> Http.send SettingsUpdated


settingsDecoder : Decoder Model
settingsDecoder =
    decode userCtor
        |> required "username" string
        |> required "picture" (nullable string)
        |> required "info" (nullable string)
        |> required "password" string
        |> required "uploadRate" int
        |> required "uploadSlots" int
        |> required "downloadRate" int
        |> required "downloadSlots" int

encodeSettings : Model -> String
encodeSettings model =
    encode 0
        <| object
            [ ("id", Json.Encode.null)
            , ("username", Json.Encode.string model.username)
            , ("password", Json.Encode.string model.password)
            , ("info", Json.Encode.string <| Maybe.withDefault "" model.info)
            , ("picture", Json.Encode.string <| Maybe.withDefault "" model.picture)
            , ("uploadSlots", Json.Encode.int model.uploadSlots)
            , ("uploadRate", Json.Encode.int model.uploadRate)
            , ("downloadSlots", Json.Encode.int model.downloadSlots)
            , ("downloadRate", Json.Encode.int model.downloadRate)
            ]
