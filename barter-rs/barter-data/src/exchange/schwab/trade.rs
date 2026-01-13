use barter_integration::subscription::SubscriptionId;
use serde::{Deserialize, Serialize};

use crate::{Identifier, exchange::subscription::ExchangeSub};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct SchwabMessage<T> {
    #[serde(
        rename = "arg",
        deserialize_with = "de_schwab_message_arg_as_subscription_id"
    )]
    pub subscription_id: SubscriptionId,
    pub data: Vec<T>,
}

fn de_schwab_message_arg_as_subscription_id<'de, D>(
    deserializer: D,
) -> Result<SubscriptionId, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Arg<'a> {
        channel: &'a str,
        inst_id: &'a str,
    }

    Deserialize::deserialize(deserializer)
        .map(|arg: Arg<'_>| ExchangeSub::from((arg.channel, arg.inst_id)).id())
}