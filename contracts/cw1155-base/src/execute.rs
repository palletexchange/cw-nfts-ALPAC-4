use crate::Cw1155Contract;
use cosmwasm_std::CustomMsg;
use cw1155::execute::Cw1155Execute;
use cw721::execute::Cw721Execute;
use serde::de::DeserializeOwned;
use serde::Serialize;

impl<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
    Cw1155Execute<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
    for Cw1155Contract<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TMetadataExtensionMsg: CustomMsg,
{
}

impl<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
    Cw721Execute<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
    for Cw1155Contract<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
where
    TCustomResponseMessage: CustomMsg,
    TMetadataExtension: Clone + DeserializeOwned + Serialize,
    TMetadataExtensionMsg: CustomMsg,
{
}
