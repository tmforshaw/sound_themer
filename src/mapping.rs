use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use serde::{Deserialize, de::IntoDeserializer};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, EnumIter, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum MappingKey {
    AudioChange,
    Login,
    Logout,
    Message,
    PowerPlug,
    PowerUnplug,
    DialogInfo,
    DialogWarning,
    DialogError,
    ScreenCapture,
    DeviceAdded,
    DeviceRemoved,
    CameraShutter,
    TrashEmpty,
    Complete,
}

impl FromStr for MappingKey {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

fn get_default_mapping_key_value(key: MappingKey) -> String {
    match key {
        MappingKey::AudioChange => "audio-volume-change",
        MappingKey::Login => "service-login",
        MappingKey::Logout => "service-logout",
        MappingKey::Message => "message",
        MappingKey::PowerPlug => "power-plug",
        MappingKey::PowerUnplug => "power-unplug",
        MappingKey::DialogInfo => "dialog-information",
        MappingKey::DialogWarning => "dialog-warning",
        MappingKey::DialogError => "dialog-error",
        MappingKey::ScreenCapture => "screen-capture",
        MappingKey::DeviceAdded => "device-added",
        MappingKey::DeviceRemoved => "device-removed",
        MappingKey::CameraShutter => "camera-shutter",
        MappingKey::TrashEmpty => "trash-empty",
        MappingKey::Complete => "complete",
    }
    .to_string()
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Mapping(HashMap<MappingKey, String>);

impl Deref for Mapping {
    type Target = HashMap<MappingKey, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Mapping {
    fn default() -> Self {
        Self(
            MappingKey::iter()
                .map(|key| (key, get_default_mapping_key_value(key)))
                .collect(),
        )
    }
}
