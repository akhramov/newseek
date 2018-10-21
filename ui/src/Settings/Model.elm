module Settings.Model exposing (Model, userCtor)

import User.Model as User

type alias Model =
    User.Model { password : String
               , uploadRate : Int
               , uploadSlots : Int
               , downloadRate : Int
               , downloadSlots : Int
               }

userCtor : String -> Maybe String -> Maybe String -> String -> Int -> Int -> Int -> Int -> Model
userCtor username picture info password uploadRate uploadSlots downloadRate downloadSlots =
    { username = username
    , picture = picture
    , info = info
    , password = password
    , uploadRate = uploadRate
    , uploadSlots = uploadSlots
    , downloadRate = downloadRate
    , downloadSlots = downloadSlots
    }
