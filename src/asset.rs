//! Module to deal with Scratch asset

use std::default;

use crate::prelude::*;

/// Costume Asset.
/// Is considered backdrop if stage.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    /// The x-coordinate of the rotation center.
    pub rotation_center_x: Number,

    /// The y-coordinate of the rotation center.
    pub rotation_center_y: Number,

    /// The reciprocal of a costume scaling factor for bitmap costumes.
    /// This may be absent. In Scratch 3.0, all bitmap costumes are double-resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitmap_resolution: Option<u64>,

    /// The MD5 hash of the asset file.
    pub asset_id: Uid,

    /// The name.
    pub name: Name,

    /// The name of the asset file.
    /// None if using the default asset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub md5ext: Option<String>,

    /// The name of the format of the asset file.
    pub data_format: String,
}

impl Default for Costume {
    fn default() -> Self {
        Costume {
            rotation_center_x: Number::Int(0),
            rotation_center_y: Number::Int(0),
            bitmap_resolution: None,
            md5ext: None,
            asset_id: Default::default(),
            data_format: Default::default(),
            name: Default::default(),
        }
    }
}

/// Sound Asset.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sound {
    /// The sampling rate of the sound in Hertz.
    pub rate: u64,

    /// The number of samples.
    pub sample_count: u64,

    /// This is for some reason exists in the file but is not documented on the wiki.
    /// I'm not exactly sure what they do since this is always empty.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub format: Option<String>,

    /// The MD5 hash of the asset file.
    pub asset_id: Uid,

    /// The name.
    pub name: Name,

    /// The name of the asset file.
    /// None if using the default asset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub md5ext: Option<String>,

    /// The name of the format of the asset file.
    pub data_format: String,
}