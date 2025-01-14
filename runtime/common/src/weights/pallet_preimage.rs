// Copyright 2022-2023 Forecasting Technologies LTD.
// Copyright 2021-2022 Zeitgeist PM LLC.
//
// This file is part of Zeitgeist.
//
// Zeitgeist is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// Zeitgeist is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Zeitgeist. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_preimage
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: `2023-10-25`, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `zeitgeist-benchmark`, CPU: `AMD EPYC 7601 32-Core Processor`
//! EXECUTION: `Some(Wasm)`, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_preimage
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --header=./HEADER_GPL3
// --output=./runtime/common/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for pallet_preimage (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::weights::WeightInfo for WeightInfo<T> {
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    /// The range of component `s` is `[0, 4194304]`.
    fn note_preimage(s: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `179`
        //  Estimated: `2566`
        // Minimum execution time: 38_370 nanoseconds.
        Weight::from_parts(39_390_000, 2566)
            // Standard Error: 6
            .saturating_add(Weight::from_parts(3_081, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    /// The range of component `s` is `[0, 4194304]`.
    fn note_requested_preimage(s: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `106`
        //  Estimated: `2566`
        // Minimum execution time: 23_661 nanoseconds.
        Weight::from_parts(24_500_000, 2566)
            // Standard Error: 5
            .saturating_add(Weight::from_parts(3_039, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    /// The range of component `s` is `[0, 4194304]`.
    fn note_no_deposit_preimage(s: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `106`
        //  Estimated: `2566`
        // Minimum execution time: 24_910 nanoseconds.
        Weight::from_parts(25_440_000, 2566)
            // Standard Error: 6
            .saturating_add(Weight::from_parts(3_036, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    fn unnote_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `357`
        //  Estimated: `2566`
        // Minimum execution time: 60_480 nanoseconds.
        Weight::from_parts(69_720_000, 2566)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    fn unnote_no_deposit_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `144`
        //  Estimated: `2566`
        // Minimum execution time: 43_861 nanoseconds.
        Weight::from_parts(49_840_000, 2566)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn request_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `220`
        //  Estimated: `2566`
        // Minimum execution time: 39_420 nanoseconds.
        Weight::from_parts(44_010_000, 2566)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn request_no_deposit_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `144`
        //  Estimated: `2566`
        // Minimum execution time: 26_310 nanoseconds.
        Weight::from_parts(30_330_000, 2566)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn request_unnoted_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `42`
        //  Estimated: `2566`
        // Minimum execution time: 31_400 nanoseconds.
        Weight::from_parts(34_880_000, 2566)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn request_requested_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `106`
        //  Estimated: `2566`
        // Minimum execution time: 14_320 nanoseconds.
        Weight::from_parts(17_110_000, 2566)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    fn unrequest_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `144`
        //  Estimated: `2566`
        // Minimum execution time: 40_830 nanoseconds.
        Weight::from_parts(48_820_000, 2566)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn unrequest_unnoted_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `106`
        //  Estimated: `2566`
        // Minimum execution time: 14_470 nanoseconds.
        Weight::from_parts(16_390_000, 2566)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn unrequest_multi_referenced_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `106`
        //  Estimated: `2566`
        // Minimum execution time: 15_280 nanoseconds.
        Weight::from_parts(16_370_000, 2566)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}
