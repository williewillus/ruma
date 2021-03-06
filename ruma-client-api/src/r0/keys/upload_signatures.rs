//! [POST /_matrix/client/r0/keys/signatures/upload](https://13301-24998719-gh.circle-artifacts.com/0/scripts/gen/client_server/unstable.html#post-matrix-client-r0-keys-signatures-upload)

use std::collections::BTreeMap;

use ruma_api::ruma_api;
use ruma_identifiers::UserId;

ruma_api! {
    metadata: {
        description: "Publishes cross-signing signatures for the user.",
        method: POST,
        name: "upload_signatures",
        path: "/_matrix/client/r0/keys/signatures/upload",
        rate_limited: false,
        requires_authentication: true,
    }

    request: {
        /// Signed keys.
        #[ruma_api(body)]
        pub signed_keys: BTreeMap<UserId, BTreeMap<String, serde_json::Value>>,
    }

    response: {}

    error: crate::Error
}
