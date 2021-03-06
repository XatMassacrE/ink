// Copyright 2018-2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Provides low-level primitives to operate on chunks of cells.

mod raw_chunk;
mod sync_chunk;
mod typed_chunk;

pub(crate) use self::raw_chunk::RawChunkCell;

pub use self::{
    raw_chunk::RawChunk,
    sync_chunk::SyncChunk,
    typed_chunk::TypedChunk,
};
