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

//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/common/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for pallet_collective (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::weights::WeightInfo for WeightInfo<T> {
    // Storage: AdvisoryCommittee Members (r:1 w:1)
    // Storage: AdvisoryCommittee Proposals (r:1 w:0)
    // Storage: AdvisoryCommittee Voting (r:255 w:255)
    // Storage: AdvisoryCommittee Prime (r:0 w:1)
    fn set_members(m: u32, _n: u32, p: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 47_000
            .saturating_add(Weight::from_ref_time(27_562_000 as u64).saturating_mul(m as u64))
            // Standard Error: 18_000
            .saturating_add(Weight::from_ref_time(14_902_000 as u64).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
    }
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    fn execute(b: u32, m: u32) -> Weight {
        Weight::from_ref_time(19_830_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64))
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(16_000 as u64).saturating_mul(m as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
    }
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee ProposalOf (r:1 w:0)
    fn propose_execute(b: u32, m: u32) -> Weight {
        Weight::from_ref_time(21_646_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64))
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(25_000 as u64).saturating_mul(m as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
    }
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee ProposalOf (r:1 w:1)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    // Storage: AdvisoryCommittee ProposalCount (r:1 w:1)
    // Storage: AdvisoryCommittee Voting (r:0 w:1)
    fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
        Weight::from_ref_time(8_210_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(16_000 as u64).saturating_mul(b as u64))
            // Standard Error: 8_000
            .saturating_add(Weight::from_ref_time(37_000 as u64).saturating_mul(m as u64))
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(322_000 as u64).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    fn vote(m: u32) -> Weight {
        Weight::from_ref_time(73_664_000 as u64)
            // Standard Error: 8_000
            .saturating_add(Weight::from_ref_time(105_000 as u64).saturating_mul(m as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    // Storage: AdvisoryCommittee ProposalOf (r:0 w:1)
    fn close_early_disapproved(_m: u32, p: u32) -> Weight {
        Weight::from_ref_time(52_166_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(235_000 as u64).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee ProposalOf (r:1 w:1)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
        Weight::from_ref_time(23_154_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(11_000 as u64).saturating_mul(b as u64))
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(65_000 as u64).saturating_mul(m as u64))
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(331_000 as u64).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee Prime (r:1 w:0)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    // Storage: AdvisoryCommittee ProposalOf (r:0 w:1)
    fn close_disapproved(_m: u32, p: u32) -> Weight {
        Weight::from_ref_time(48_403_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(242_000 as u64).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee Prime (r:1 w:0)
    // Storage: AdvisoryCommittee ProposalOf (r:1 w:1)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    fn close_approved(b: u32, m: u32, p: u32) -> Weight {
        Weight::from_ref_time(24_592_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(13_000 as u64).saturating_mul(b as u64))
            // Standard Error: 10_000
            .saturating_add(Weight::from_ref_time(29_000 as u64).saturating_mul(m as u64))
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(347_000 as u64).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    // Storage: AdvisoryCommittee Voting (r:0 w:1)
    // Storage: AdvisoryCommittee ProposalOf (r:0 w:1)
    fn disapprove_proposal(p: u32) -> Weight {
        Weight::from_ref_time(20_806_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(224_000 as u64).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
}
