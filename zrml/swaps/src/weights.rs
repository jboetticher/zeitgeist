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

//! Autogenerated weights for zrml_swaps
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-21, STEPS: `10`, REPEAT: 1000, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=1000
// --pallet=zrml_swaps
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/weight_template.hbs
// --output=./zrml/swaps/src/weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use core::marker::PhantomData;
use frame_support::{traits::Get, weights::Weight};

///  Trait containing the required functions for weight retrival within
/// zrml_swaps (automatically generated)
pub trait WeightInfoZeitgeist {
    fn admin_clean_up_pool_cpmm_categorical(a: u32) -> Weight;
    fn admin_clean_up_pool_cpmm_scalar() -> Weight;
    fn apply_to_cached_pools_execute_arbitrage(a: u32) -> Weight;
    fn apply_to_cached_pools_noop(a: u32) -> Weight;
    fn destroy_pool_in_subsidy_phase(a: u32) -> Weight;
    fn distribute_pool_share_rewards(a: u32, b: u32) -> Weight;
    fn end_subsidy_phase(a: u32, b: u32) -> Weight;
    fn execute_arbitrage_buy_burn(a: u32) -> Weight;
    fn execute_arbitrage_mint_sell(a: u32) -> Weight;
    fn execute_arbitrage_skipped(a: u32) -> Weight;
    fn pool_exit(a: u32) -> Weight;
    fn pool_exit_subsidy() -> Weight;
    fn pool_exit_with_exact_asset_amount() -> Weight;
    fn pool_exit_with_exact_pool_amount() -> Weight;
    fn pool_join(a: u32) -> Weight;
    fn pool_join_subsidy() -> Weight;
    fn pool_join_with_exact_asset_amount() -> Weight;
    fn pool_join_with_exact_pool_amount() -> Weight;
    fn clean_up_pool_categorical_without_reward_distribution(a: u32) -> Weight;
    fn swap_exact_amount_in_cpmm() -> Weight;
    fn swap_exact_amount_in_rikiddo(a: u32) -> Weight;
    fn swap_exact_amount_out_cpmm() -> Weight;
    fn swap_exact_amount_out_rikiddo(a: u32) -> Weight;
    fn open_pool(a: u32) -> Weight;
    fn close_pool(a: u32) -> Weight;
    fn destroy_pool(a: u32) -> Weight;
}

/// Weight functions for zrml_swaps (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfoZeitgeist for WeightInfo<T> {
    // Storage: MarketCommons Markets (r:1 w:0)
    // Storage: MarketCommons MarketPool (r:1 w:0)
    // Storage: Swaps Pools (r:1 w:1)
    fn admin_clean_up_pool_cpmm_categorical(a: u32) -> Weight {
        Weight::from_ref_time(31_098_000 as u64)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(731_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: MarketCommons Markets (r:1 w:0)
    // Storage: MarketCommons MarketPool (r:1 w:0)
    // Storage: Swaps Pools (r:1 w:1)
    fn admin_clean_up_pool_cpmm_scalar() -> Weight {
        Weight::from_ref_time(32_642_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Swaps PoolsCachedForArbitrage (r:8 w:7)
    // Storage: Swaps Pools (r:7 w:0)
    // Storage: Tokens Accounts (r:462 w:462)
    // Storage: System Account (r:7 w:0)
    // Storage: Tokens TotalIssuance (r:64 w:64)
    fn apply_to_cached_pools_execute_arbitrage(a: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 1_469_000
            .saturating_add(Weight::from_ref_time(1_675_452_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(43 as u64))
            .saturating_add(T::DbWeight::get().reads((70 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes(42 as u64))
            .saturating_add(T::DbWeight::get().writes((67 as u64).saturating_mul(a as u64)))
    }
    // Storage: Swaps PoolsCachedForArbitrage (r:8 w:7)
    fn apply_to_cached_pools_noop(a: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(5_608_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(a as u64)))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:1 w:0)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:1)
    // Storage: Tokens Accounts (r:1 w:1)
    fn destroy_pool_in_subsidy_phase(a: u32) -> Weight {
        Weight::from_ref_time(29_566_000 as u64)
            // Standard Error: 8_000
            .saturating_add(Weight::from_ref_time(13_399_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(a as u64)))
    }
    // Storage: Tokens TotalIssuance (r:2 w:1)
    // Storage: Tokens Accounts (r:46 w:22)
    // Storage: System Account (r:2 w:1)
    fn distribute_pool_share_rewards(a: u32, b: u32) -> Weight {
        Weight::from_ref_time(35_420_000 as u64)
            // Standard Error: 35_000
            .saturating_add(Weight::from_ref_time(17_211_000 as u64).saturating_mul(a as u64))
            // Standard Error: 35_000
            .saturating_add(Weight::from_ref_time(26_355_000 as u64).saturating_mul(b as u64))
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(b as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(b as u64)))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:11 w:10)
    // Storage: Tokens Accounts (r:22 w:22)
    // Storage: System Account (r:11 w:11)
    // Storage: Tokens TotalIssuance (r:2 w:2)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:0)
    fn end_subsidy_phase(a: u32, b: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 112_000
            .saturating_add(Weight::from_ref_time(18_230_000 as u64).saturating_mul(a as u64))
            // Standard Error: 688_000
            .saturating_add(Weight::from_ref_time(84_266_000 as u64).saturating_mul(b as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().reads((9 as u64).saturating_mul(b as u64)))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes((9 as u64).saturating_mul(b as u64)))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: System Account (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    fn execute_arbitrage_buy_burn(a: u32) -> Weight {
        Weight::from_ref_time(41_088_000 as u64)
            // Standard Error: 15_000
            .saturating_add(Weight::from_ref_time(24_575_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(a as u64)))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: System Account (r:2 w:1)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    fn execute_arbitrage_mint_sell(a: u32) -> Weight {
        Weight::from_ref_time(42_289_000 as u64)
            // Standard Error: 15_000
            .saturating_add(Weight::from_ref_time(23_012_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(a as u64)))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:2 w:0)
    fn execute_arbitrage_skipped(a: u32) -> Weight {
        Weight::from_ref_time(19_470_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(3_448_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(a as u64)))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:5 w:5)
    // Storage: System Account (r:1 w:0)
    fn pool_exit(a: u32) -> Weight {
        Weight::from_ref_time(45_823_000 as u64)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(18_287_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(a as u64)))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:1 w:1)
    // Storage: Tokens Accounts (r:1 w:1)
    fn pool_exit_subsidy() -> Weight {
        Weight::from_ref_time(48_230_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_exit_with_exact_asset_amount() -> Weight {
        Weight::from_ref_time(96_892_000 as u64)
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_exit_with_exact_pool_amount() -> Weight {
        Weight::from_ref_time(99_737_000 as u64)
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:5 w:5)
    fn pool_join(a: u32) -> Weight {
        Weight::from_ref_time(37_826_000 as u64)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(16_946_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(a as u64)))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Tokens Accounts (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:1 w:1)
    fn pool_join_subsidy() -> Weight {
        Weight::from_ref_time(49_883_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_join_with_exact_asset_amount() -> Weight {
        Weight::from_ref_time(87_144_000 as u64)
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_join_with_exact_pool_amount() -> Weight {
        Weight::from_ref_time(86_612_000 as u64)
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Swaps Pools (r:1 w:1)
    fn clean_up_pool_categorical_without_reward_distribution(a: u32) -> Weight {
        Weight::from_ref_time(8_025_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(272_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:4 w:4)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn swap_exact_amount_in_cpmm() -> Weight {
        Weight::from_ref_time(117_070_000 as u64)
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Tokens TotalIssuance (r:2 w:1)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    fn swap_exact_amount_in_rikiddo(a: u32) -> Weight {
        Weight::from_ref_time(93_123_000 as u64)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(12_678_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:4 w:4)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn swap_exact_amount_out_cpmm() -> Weight {
        Weight::from_ref_time(112_852_000 as u64)
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:4 w:3)
    // Storage: Tokens TotalIssuance (r:2 w:1)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn swap_exact_amount_out_rikiddo(a: u32) -> Weight {
        Weight::from_ref_time(63_407_000 as u64)
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(22_008_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Swaps Pools (r:1 w:1)
    fn open_pool(a: u32) -> Weight {
        Weight::from_ref_time(17_686_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(378_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Swaps Pools (r:1 w:1)
    fn close_pool(a: u32) -> Weight {
        Weight::from_ref_time(16_582_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(280_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Tokens Accounts (r:2 w:2)
    // Storage: Tokens TotalIssuance (r:2 w:2)
    fn destroy_pool(a: u32) -> Weight {
        Weight::from_ref_time(16_846_000 as u64)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(16_174_000 as u64).saturating_mul(a as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(a as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(a as u64)))
    }
}
