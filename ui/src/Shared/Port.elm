port module Shared.Port exposing (getPicture, updatePicture)

import Json.Decode exposing (Value)

port getPicture : Value -> Cmd msg
port updatePicture : (String -> msg) -> Sub msg
