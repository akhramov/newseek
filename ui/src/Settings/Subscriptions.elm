module Settings.Subscriptions exposing (subscriptions)

import Settings.Message exposing (Msg(..))
import Shared.Port exposing (updatePicture)

subscriptions : Sub Msg
subscriptions =
    updatePicture PictureUpdated
