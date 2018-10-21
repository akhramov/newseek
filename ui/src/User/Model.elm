module User.Model exposing (Model)

type alias Model a =
    { a |
      username: String
    , picture: Maybe String
    , info: Maybe String
    }
