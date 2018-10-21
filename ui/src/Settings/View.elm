module Settings.View exposing (view)

import Json.Decode as Json
import Html.Styled as Html exposing (Html, div)
import Html.Styled.Attributes as Attr exposing (..)
import Html.Styled.Events exposing (..)
import Html.Styled.Attributes exposing (..)

import Css exposing (..)
import Css.Foreign exposing (adjacentSiblings, selector)
import String.Extra exposing (toSentenceCase, dasherize)

import Shared.Form exposing (input, group, formLabel, textarea, button)
import Shared.Form.ImageInput exposing (..)

import Settings.Model exposing (Model)
import Settings.Message exposing (Msg(..))

view : Maybe Model -> Html Msg
view = sections >> div []

sections : Maybe Model -> List (Html Msg)
sections maybeModel =
    case maybeModel of
        Just model ->
          [ accountSection model
          , informationSection model
          , downloadsSection model
          , uploadsSection model
          , button "Save" [ onClick SaveButtonClicked ]
          ]
        Nothing -> []


accountSection : Model -> Html Msg
accountSection =
    (|>)
        >> (flip List.map) [ usernameInput, passwordInput ]
        >> group "Account"

usernameInput : Model -> Html Msg
usernameInput { username } =
    input [ value username, onInput UsernameChanged ] ("username", "text")

passwordInput : Model -> Html Msg
passwordInput { password } =
    input [ value password, onInput PasswordChanged ] ("password", "password")

informationSection : Model -> Html Msg
informationSection model =
    [ imageInput "Choose image" model.picture [ onPictureChange PictureChanged ]
    , [ textarea "Biography" [ onInput InfoChanged, value (Maybe.withDefault "" model.info) ] ] ]
        |> List.concat
        |> group "Personal Information"

downloadsSection :  Model -> Html Msg
downloadsSection model =
    [ ("Download rate", model.downloadRate, DownloadRateChanged)
    , ("Download slots", model.downloadSlots, DownloadSlotsChanged)
    ]
        |> List.map numericInput
        |> group "Downloads"

uploadsSection : Model -> Html Msg
uploadsSection model =
    [ ("Upload rate", model.uploadRate, UploadRateChanged)
    , ("Upload slots", model.uploadSlots, UploadSlotsChanged)
    ]
        |> List.map numericInput
        |> group "Uploads"

numericInput : (String, Int, String -> Msg) -> Html Msg
numericInput (inputName, val, msg) =
    input [ Attr.min "0", value (toString val), onInput msg] (inputName, "tel")

onPictureChange : (Json.Value -> msg) -> Html.Attribute msg
onPictureChange tagger =
  on "input" (Json.map tagger file)

file : Json.Decoder Json.Value
file = Json.at ["target", "files", "0"] Json.value
