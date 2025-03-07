// Source code for the Substrate Telemetry Server.
// Copyright (C) 2021 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! These types are partly used in [`crate::node_message`], but also stored and used
//! more generally through the application.

use serde::ser::{SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};

use crate::{time, MeanList};

pub type BlockNumber = u64;
pub type Timestamp = u64;
pub use primitive_types::H256 as BlockHash;

/// Basic node details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeDetails {
    pub chain: Box<str>,
    pub name: Box<str>,
    pub implementation: Box<str>,
    pub version: Box<str>,
    pub validator: Option<Box<str>>,
    pub network_id: Option<Box<str>>,
    pub startup_time: Option<Box<str>>,
}

/// A couple of node statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NodeStats {
    pub peers: u64,
    pub txcount: u64,
}

// # A note about serialization/deserialization of types in this file:
//
// Some of the types here are sent to UI feeds. In an effort to keep the
// amount of bytes sent to a minimum, we have written custom serializers
// for those types.
//
// For testing purposes, it's useful to be able to deserialize from some
// of these types so that we can test message feed things, so custom
// deserializers exist to undo the work of the custom serializers.
impl Serialize for NodeStats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&self.peers)?;
        tup.serialize_element(&self.txcount)?;
        tup.end()
    }
}

impl<'de> Deserialize<'de> for NodeStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (peers, txcount) = <(u64, u64)>::deserialize(deserializer)?;
        Ok(NodeStats { peers, txcount })
    }
}

/// Node IO details.
#[derive(Default)]
pub struct NodeIO {
    pub used_state_cache_size: MeanList<f32>,
}

impl Serialize for NodeIO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(1)?;
        // This is "one-way": we can't deserialize again from this to a MeanList:
        tup.serialize_element(self.used_state_cache_size.slice())?;
        tup.end()
    }
}

/// Concise block details
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub hash: BlockHash,
    pub height: BlockNumber,
}

impl Block {
    pub fn zero() -> Self {
        Block {
            hash: BlockHash::from([0; 32]),
            height: 0,
        }
    }
}

/// Node hardware details.
#[derive(Default)]
pub struct NodeHardware {
    /// Upload uses means
    pub upload: MeanList<f64>,
    /// Download uses means
    pub download: MeanList<f64>,
    /// Stampchange uses means
    pub chart_stamps: MeanList<f64>,
}

impl Serialize for NodeHardware {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(3)?;
        // These are "one-way": we can't deserialize again from them to MeanLists:
        tup.serialize_element(self.upload.slice())?;
        tup.serialize_element(self.download.slice())?;
        tup.serialize_element(self.chart_stamps.slice())?;
        tup.end()
    }
}

/// Node location details
#[derive(Debug, Clone, PartialEq)]
pub struct NodeLocation {
    pub latitude: f32,
    pub longitude: f32,
    pub city: Box<str>,
}

impl Serialize for NodeLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(3)?;
        tup.serialize_element(&self.latitude)?;
        tup.serialize_element(&self.longitude)?;
        tup.serialize_element(&&*self.city)?;
        tup.end()
    }
}

impl<'de> Deserialize<'de> for NodeLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (latitude, longitude, city) = <(f32, f32, Box<str>)>::deserialize(deserializer)?;
        Ok(NodeLocation {
            latitude,
            longitude,
            city,
        })
    }
}

/// Verbose block details
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockDetails {
    pub block: Block,
    pub block_time: u64,
    pub block_timestamp: u64,
    pub propagation_time: Option<u64>,
}

impl Default for BlockDetails {
    fn default() -> Self {
        BlockDetails {
            block: Block::zero(),
            block_timestamp: time::now(),
            block_time: 0,
            propagation_time: None,
        }
    }
}

impl Serialize for BlockDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(5)?;
        tup.serialize_element(&self.block.height)?;
        tup.serialize_element(&self.block.hash)?;
        tup.serialize_element(&self.block_time)?;
        tup.serialize_element(&self.block_timestamp)?;
        tup.serialize_element(&self.propagation_time)?;
        tup.end()
    }
}

impl<'de> Deserialize<'de> for BlockDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tup = <(u64, BlockHash, u64, u64, Option<u64>)>::deserialize(deserializer)?;
        Ok(BlockDetails {
            block: Block {
                height: tup.0,
                hash: tup.1,
            },
            block_time: tup.2,
            block_timestamp: tup.3,
            propagation_time: tup.4,
        })
    }
}
