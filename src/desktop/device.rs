//! **Note** This portal doesn't work for sandboxed applications.
//! # Examples
//!
//! Access a [`Device`](crate::desktop::device::Device)
//!
//! ```rust,no_run
//! use ashpd::desktop::device::{Device, DeviceProxy};
//!
//! async fn run() -> ashpd::Result<()> {
//!     let connection = zbus::Connection::session().await?;
//!     let proxy = DeviceProxy::new(&connection).await?;
//!     proxy.access_device(6879, &[Device::Speakers]).await?;
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize, Serializer};
use strum_macros::{AsRefStr, EnumString, IntoStaticStr, ToString};
use zvariant::Signature;
use zvariant_derive::{DeserializeDict, SerializeDict, TypeDict};

use super::{HandleToken, DESTINATION, PATH};
use crate::{helpers::call_basic_response_method, Error};

#[derive(SerializeDict, DeserializeDict, TypeDict, Clone, Debug, Default)]
/// Specified options for a [`DeviceProxy::access_device`] request.
struct AccessDeviceOptions {
    /// A string that will be used as the last element of the handle.
    handle_token: HandleToken,
}

#[derive(
    Debug, Clone, Copy, Deserialize, EnumString, AsRefStr, IntoStaticStr, ToString, PartialEq, Eq,
)]
#[strum(serialize_all = "lowercase")]
/// The possible device to request access to.
pub enum Device {
    /// A microphone.
    Microphone,
    /// Speakers.
    Speakers,
    /// A Camera.
    Camera,
}

impl zvariant::Type for Device {
    fn signature() -> Signature<'static> {
        String::signature()
    }
}

impl Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&self.to_string(), serializer)
    }
}

/// The interface lets services ask if an application should get access to
/// devices such as microphones, speakers or cameras. Not a portal in the strict
/// sense, since the API is not directly accessible to applications inside the
/// sandbox.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Device`](https://flatpak.github.io/xdg-desktop-portal/portal-docs.html#gdbus-org.freedesktop.portal.Device).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Device")]
pub struct DeviceProxy<'a>(zbus::Proxy<'a>);

impl<'a> DeviceProxy<'a> {
    /// Create a new instance of [`DeviceProxy`].
    pub async fn new(connection: &zbus::Connection) -> Result<DeviceProxy<'a>, Error> {
        let proxy = zbus::ProxyBuilder::new_bare(connection)
            .interface("org.freedesktop.portal.Device")?
            .path(PATH)?
            .destination(DESTINATION)?
            .build()
            .await?;
        Ok(Self(proxy))
    }

    /// Get a reference to the underlying Proxy.
    pub fn inner(&self) -> &zbus::Proxy<'_> {
        &self.0
    }

    /// Asks for access to a device.
    ///
    /// # Arguments
    ///
    /// * `pid` - The pid of the application on whose behalf the request is
    ///   made.
    /// * `devices` - A list of devices to request access to.
    ///
    /// *Note* Asking for multiple devices at the same time may or may not be supported
    ///
    /// # Specifications
    ///
    /// See also [`AccessDevice`](https://flatpak.github.io/xdg-desktop-portal/portal-docs.html#gdbus-method-org-freedesktop-portal-Device.AccessDevice).
    #[doc(alias = "AccessDevice")]
    pub async fn access_device(&self, pid: u32, devices: &[Device]) -> Result<(), Error> {
        let options = AccessDeviceOptions::default();
        call_basic_response_method(
            &self.0,
            &options.handle_token,
            "AccessDevice",
            &(pid, devices, &options),
        )
        .await
    }
}
