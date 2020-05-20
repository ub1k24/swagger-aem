{-
   Adobe Experience Manager (AEM) API
   Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API

   OpenAPI spec version: 3.2.0-pre.0
   Contact: opensource@shinesolutions.com

   NOTE: This file is auto generated by the openapi-generator.
   https://github.com/openapitools/openapi-generator.git
   Do not edit this file manually.
-}


module Data.TruststoreInfo exposing (TruststoreInfo, truststoreInfoDecoder, truststoreInfoEncoder)

import Data.TruststoreItems exposing (TruststoreItems, truststoreItemsDecoder, truststoreItemsEncoder)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Pipeline exposing (decode, optional, required)
import Json.Encode as Encode
import Maybe exposing (map, withDefault)


type alias TruststoreInfo =
    { aliases : Maybe (List TruststoreItems)
    , exists : Maybe Bool
    }


truststoreInfoDecoder : Decoder TruststoreInfo
truststoreInfoDecoder =
    decode TruststoreInfo
        |> optional "aliases" (Decode.nullable (Decode.list truststoreItemsDecoder)) Nothing
        |> optional "exists" (Decode.nullable Decode.bool) Nothing



truststoreInfoEncoder : TruststoreInfo -> Encode.Value
truststoreInfoEncoder model =
    Encode.object
        [ ( "aliases", withDefault Encode.null (map (Encode.list << List.map truststoreItemsEncoder) model.aliases) )
        , ( "exists", withDefault Encode.null (map Encode.bool model.exists) )
        ]

