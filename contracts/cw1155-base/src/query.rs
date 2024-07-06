use crate::Cw1155Contract;
use cosmwasm_std::CustomMsg;
use cw1155::query::Cw1155Query;
use cw721::query::Cw721Query;
use serde::de::DeserializeOwned;
use serde::Serialize;

impl<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
    Cw1155Query<TMetadataExtension>
    for Cw1155Contract<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TMetadataExtensionMsg: CustomMsg,
{
}

impl<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
    Cw721Query<TMetadataExtension>
    for Cw1155Contract<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
where
    TCustomResponseMessage: CustomMsg,
    TMetadataExtension: Clone + DeserializeOwned + Serialize,
    TMetadataExtensionMsg: CustomMsg,
{
}
