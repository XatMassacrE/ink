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

#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::{
    memory::format,
    storage,
};
use ink_lang::contract;

contract! {
    #![env = ink_core::env::DefaultSrmlTypes]

    /// Decrements the accumulator's value.
    struct Subber {
        /// The accumulator to store values.
        accumulator: storage::Value<accumulator::Accumulator>,
    }

    impl Deploy for Subber {
        fn deploy(&mut self, accumulator: AccountId) {
            self.accumulator.set(accumulator::Accumulator::from_account_id(accumulator));
        }
    }

    impl Subber {
        pub(external) fn dec(&mut self, by: i32) {
            self.accumulator.inc(-by);
        }
    }
}
