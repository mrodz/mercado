use std::collections::{BTreeMap, HashMap};

use super::{channel::SchwabChannel, market::SchwabMarket};
use crate::exchange::subscription::ExchangeSub;
use barter_integration::{Validator, error::SocketError};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use serde_json::Value;

// Implement custom Serialize to assist aesthetics of <schwab as Connector>::requests() function.
impl Serialize for ExchangeSub<SchwabChannel, SchwabMarket> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SchwabSubArg", 2)?;
        state.serialize_field("channel", self.channel.as_ref())?;
        state.serialize_field("instId", self.market.as_ref())?;
        state.end()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SchwabSubResponse {
    Event { 
        event: String,
        #[serde(flatten)]
        other: HashMap<String, Value>
    },

    Error {
        error: BTreeMap<String, String>
    },
}

impl Validator for SchwabSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        match self {
            Self::Event { ref event, .. } if event == "subscribed" => Ok(self),
            Self::Event { event, other } => Err(SocketError::Subscribe(format!("did not receive subscribe event: got {event}, {other:?}"))),
            Self::Error { error } => Err(SocketError::Subscribe(format!(
                "received failure subscription response: {error:?}",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod de {
        use super::*;

        #[test]
        fn test_schwab_subscription_response() {
            struct TestCase {
                input: &'static str,
                expected: Result<SchwabSubResponse, SocketError>,
            }

            let cases = vec![
                TestCase {
                    input: r#"
                {"event":"subscribed"}
                "#,
                    expected: Ok(SchwabSubResponse::Event { event: "subscribed".to_owned(), other: HashMap::new() }),
                },
                TestCase {
                    input: r#"
                {"error":{"MissingAuthentication":"missing authentication cookie"}}
                "#,
                    expected: Ok(SchwabSubResponse::Error {
                        error: BTreeMap::from([(
                            "MissingAuthentication".to_owned(),
                            "missing authentication cookie".to_owned(),
                        )]),
                    }),
                },
            ];

            for (index, test) in cases.into_iter().enumerate() {
                let actual = serde_json::from_str::<SchwabSubResponse>(test.input);
                match (actual, test.expected) {
                    (Ok(actual), Ok(expected)) => {
                        assert_eq!(actual, expected, "TC{} failed", index)
                    }
                    (Err(_), Err(_)) => {
                        // Test passed
                    }
                    (actual, expected) => {
                        // Test failed
                        panic!(
                            "TC{index} failed because actual != expected. \nActual: {actual:?}\nExpected: {expected:?}\n"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_validate_schwab_sub_response() {
        struct TestCase {
            input_response: SchwabSubResponse,
            is_valid: bool,
        }

        let cases = vec![
            TestCase {
                // TC0: input response is subscription success
                input_response: SchwabSubResponse::Event { event: "subscribed".to_owned(), other: HashMap::new() },
                is_valid: true,
            },
            TestCase {
                // TC1: input response is failed subscription
                input_response: SchwabSubResponse::Error {
                    error: BTreeMap::from([(
                        "MissingAuthentication".to_owned(),
                        "missing authentication cookie".to_owned(),
                    )]),
                },
                is_valid: false,
            },
        ];

        for (index, test) in cases.into_iter().enumerate() {
            let actual = test.input_response.validate().is_ok();
            assert_eq!(actual, test.is_valid, "TestCase {} failed", index);
        }
    }
}
