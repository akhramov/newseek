module Message exposing (Msg(..))

import Navigation exposing (Location)

import Settings.Message as Settings

type Msg
    = NoOp
    | OnLocationChange Location
    | SettingsMessage Settings.Msg
