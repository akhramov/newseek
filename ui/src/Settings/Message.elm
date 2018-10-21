module Settings.Message exposing (Msg(..))

import Json.Decode exposing (Value)
import Settings.Model exposing(..)
import Http

type Msg
    = UsernameChanged String
    | PasswordChanged String
    | PictureChanged Value
    | PictureUpdated String
    | InfoChanged String
    | DownloadRateChanged String
    | DownloadSlotsChanged String
    | UploadRateChanged String
    | UploadSlotsChanged String
    | SaveButtonClicked
    | SettingsUpdated (Result Http.Error Model)
