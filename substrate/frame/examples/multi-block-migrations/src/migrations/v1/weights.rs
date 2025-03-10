// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_example_mbm`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Olivers-MBP`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// polkadot-omni-bencher
// v1
// benchmark
// pallet
// --runtime
// target/release/wbuild/argochain-runtime/argochain_runtime.compact.compressed.wasm
// --pallet
// pallet_example_mbm
// --extrinsic
// 
// --template
// substrate/.maintain/frame-weight-template.hbs
// --output
// substrate/frame/examples/multi-block-migrations/src/migrations/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_example_mbm`.
pub trait WeightInfo {
	fn step() -> Weight;
}

/// Weights for `pallet_example_mbm` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `PalletExampleMbms::MyMap` (r:2 w:1)
	/// Proof: `PalletExampleMbms::MyMap` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `28`
		//  Estimated: `5996`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(8_000_000, 5996)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `PalletExampleMbms::MyMap` (r:2 w:1)
	/// Proof: `PalletExampleMbms::MyMap` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `28`
		//  Estimated: `5996`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(8_000_000, 5996)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
