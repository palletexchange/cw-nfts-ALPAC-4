use cosmwasm_std::CustomMsg;
use cw1155::state::Cw1155Config;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Cw1155Contract<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TMetadataExtension,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMessage,
    // Message passed for updating metadata.
    TMetadataExtensionMsg,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: CustomMsg,
{
    pub config: Cw1155Config<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>,
}

impl<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg> Default
    for Cw1155Contract<'static, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg>
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: CustomMsg,
{
    fn default() -> Self {
        Self {
            config: Cw1155Config::default(),
        }
    }
}
