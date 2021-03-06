//! [GET /_matrix/federation/v1/user/devices/](https://matrix.org/docs/spec/server_server/r0.1.4#get-matrix-federation-v1-user-devices-userid)

use js_int::UInt;
use ruma_api::ruma_api;
use ruma_common::encryption::DeviceKeys;
use ruma_identifiers::{DeviceId, UserId};
use serde::{Deserialize, Serialize};

ruma_api! {
    metadata: {
        description: "Gets information on all of the user's devices.",
        name: "get_devices",
        method: GET,
        path: "/_matrix/federation/v1/user/devices",
        rate_limited: false,
        requires_authentication: true,
    }

    request: {
        /// The user ID to retrieve devices for. Must be a user local to the receiving homeserver.
        #[ruma_api(query)]
        pub user_id: &'a UserId,
    }

    response: {
        /// The user ID devices were requested for.
        pub user_id: UserId,

        /// A unique ID for a given user_id which describes the version of the returned device
        /// list. This is matched with the `stream_id` field in `m.device_list_update` EDUs in
        /// order to incrementally update the returned device_list.
        pub stream_id: UInt,

        /// The user's devices. May be empty.
        pub devices: Vec<UserDevice>,
    }
}

/// Information about a user's device.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDevice {
    /// The device ID.
    pub device_id: Box<DeviceId>,

    /// Identity keys for the device.
    pub keys: DeviceKeys,

    /// Optional display name for the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_display_name: Option<String>,
}

impl<'a> Request<'a> {
    /// Creates a new `Request` with the given user id
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a new `Response` with the given user id, stream id and devices.
    pub fn new(user_id: UserId, stream_id: UInt, devices: Vec<UserDevice>) -> Self {
        Self { user_id, stream_id, devices }
    }
}
