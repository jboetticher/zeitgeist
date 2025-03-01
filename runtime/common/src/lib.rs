// Copyright 2022-2024 Forecasting Technologies LTD.
// Copyright 2021-2022 Zeitgeist PM LLC.
// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//     Copyright (C) 2020-2022 Acala Foundation.
//     SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]
#![allow(clippy::crate_in_macro_def)]

pub mod fees;
pub mod weights;

#[macro_export]
macro_rules! decl_common_types {
    () => {
        use core::marker::PhantomData;
        use frame_support::traits::{
            Currency, Imbalance, NeverEnsureOrigin, OnRuntimeUpgrade, OnUnbalanced,
        };
        #[cfg(feature = "try-runtime")]
        use frame_try_runtime::{TryStateSelect, UpgradeCheckSelect};
        use orml_traits::MultiCurrency;
        use sp_runtime::{generic, DispatchError, DispatchResult, SaturatedConversion};
        use zeitgeist_primitives::traits::{DeployPoolApi, DistributeFees, MarketCommonsPalletApi};

        pub type Block = generic::Block<Header, UncheckedExtrinsic>;

        type Address = sp_runtime::MultiAddress<AccountId, ()>;

        type Migrations = ();

        pub type Executive = frame_executive::Executive<
            Runtime,
            Block,
            frame_system::ChainContext<Runtime>,
            Runtime,
            AllPalletsWithSystem,
            Migrations,
        >;

        pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
        pub(crate) type NodeBlock = generic::Block<Header, sp_runtime::OpaqueExtrinsic>;
        type RikiddoSigmoidFeeMarketVolumeEma = zrml_rikiddo::Instance1;
        pub type SignedExtra = (
            CheckNonZeroSender<Runtime>,
            CheckSpecVersion<Runtime>,
            CheckTxVersion<Runtime>,
            CheckGenesis<Runtime>,
            CheckEra<Runtime>,
            CheckNonce<Runtime>,
            CheckWeight<Runtime>,
            // https://docs.rs/pallet-asset-tx-payment/latest/src/pallet_asset_tx_payment/lib.rs.html#32-34
            pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>,
        );
        pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
        pub type UncheckedExtrinsic =
            generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

        // Governance
        type AdvisoryCommitteeInstance = pallet_collective::Instance1;
        type AdvisoryCommitteeMembershipInstance = pallet_membership::Instance1;
        type CouncilInstance = pallet_collective::Instance2;
        type CouncilMembershipInstance = pallet_membership::Instance2;
        type TechnicalCommitteeInstance = pallet_collective::Instance3;
        type TechnicalCommitteeMembershipInstance = pallet_membership::Instance3;

        // Council vote proportions
        // At least 50%
        type EnsureRootOrHalfCouncil = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionAtLeast<AccountId, CouncilInstance, 1, 2>,
        >;

        // At least 66%
        type EnsureRootOrTwoThirdsCouncil = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionAtLeast<AccountId, CouncilInstance, 2, 3>,
        >;

        // At least 75%
        type EnsureRootOrThreeFourthsCouncil = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionAtLeast<AccountId, CouncilInstance, 3, 4>,
        >;

        // At least 100%
        type EnsureRootOrAllCouncil = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionAtLeast<AccountId, CouncilInstance, 1, 1>,
        >;

        // Technical committee vote proportions
        // At least 50%
        #[cfg(feature = "parachain")]
        type EnsureRootOrHalfTechnicalCommittee = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 2>,
        >;

        // At least 66%
        type EnsureRootOrTwoThirdsTechnicalCommittee = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 2, 3>,
        >;

        // At least 100%
        type EnsureRootOrAllTechnicalCommittee = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 1>,
        >;

        // Advisory Committee vote proportions
        // More than 33%
        type EnsureRootOrMoreThanOneThirdAdvisoryCommittee = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionMoreThan<AccountId, AdvisoryCommitteeInstance, 1, 3>,
        >;

        // More than 50%
        type EnsureRootOrMoreThanHalfAdvisoryCommittee = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionMoreThan<AccountId, AdvisoryCommitteeInstance, 1, 2>,
        >;

        // More than 66%
        type EnsureRootOrMoreThanTwoThirdsAdvisoryCommittee = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionMoreThan<AccountId, AdvisoryCommitteeInstance, 2, 3>,
        >;

        // At least 66%
        type EnsureRootOrTwoThirdsAdvisoryCommittee = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionAtLeast<AccountId, AdvisoryCommitteeInstance, 2, 3>,
        >;

        // At least 100%
        type EnsureRootOrAllAdvisoryCommittee = EitherOfDiverse<
            EnsureRoot<AccountId>,
            EnsureProportionAtLeast<AccountId, AdvisoryCommitteeInstance, 1, 1>,
        >;

        #[cfg(feature = "std")]
        /// The version information used to identify this runtime when compiled natively.
        pub fn native_version() -> NativeVersion {
            NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
        }

        // Accounts protected from being deleted due to a too low amount of funds.
        pub struct DustRemovalWhitelist;

        impl Contains<AccountId> for DustRemovalWhitelist
        where
            frame_support::PalletId: AccountIdConversion<AccountId>,
        {
            fn contains(ai: &AccountId) -> bool {
                let mut pallets = vec![
                    AuthorizedPalletId::get(),
                    CourtPalletId::get(),
                    GlobalDisputesPalletId::get(),
                    LiquidityMiningPalletId::get(),
                    OrderbookPalletId::get(),
                    ParimutuelPalletId::get(),
                    PmPalletId::get(),
                    SimpleDisputesPalletId::get(),
                    SwapsPalletId::get(),
                    TreasuryPalletId::get(),
                ];

                if let Some(pallet_id) = frame_support::PalletId::try_from_sub_account::<u128>(ai) {
                    return pallets.contains(&pallet_id.0);
                }

                for pallet_id in pallets {
                    let pallet_acc: AccountId = pallet_id.into_account_truncating();

                    if pallet_acc == *ai {
                        return true;
                    }
                }

                false
            }
        }

        common_runtime::impl_fee_types!();

        pub mod opaque {
            //! Opaque types. These are used by the CLI to instantiate machinery that don't need to
            //! know the specifics of the runtime. They can then be made to be agnostic over
            //! specific formats of data like extrinsics, allowing for them to continue syncing the
            //! network through upgrades to even the core data structures.

            use super::Header;
            use alloc::vec::Vec;
            use sp_runtime::{generic, impl_opaque_keys};

            pub type Block = generic::Block<Header, sp_runtime::OpaqueExtrinsic>;

            #[cfg(feature = "parachain")]
            impl_opaque_keys! {
                pub struct SessionKeys {
                    pub nimbus: crate::AuthorInherent,
                    pub vrf: session_keys_primitives::VrfSessionKey,
                }
            }

            #[cfg(not(feature = "parachain"))]
            impl_opaque_keys! {
                pub struct SessionKeys {
                    pub aura: crate::Aura,
                    pub grandpa: crate::Grandpa,
                }
            }
        }
    };
}

// Construct runtime
#[macro_export]
macro_rules! create_runtime {
    ($($additional_pallets:tt)*) => {
        use alloc::{boxed::Box, vec::Vec};
        // Pallets are enumerated based on the dependency graph.
        //
        // For example, `PredictionMarkets` is pĺaced after `SimpleDisputes` because
        // `PredictionMarkets` depends on `SimpleDisputes`.

        construct_runtime!(
            pub enum Runtime where
                Block = crate::Block,
                NodeBlock = crate::NodeBlock,
                UncheckedExtrinsic = crate::UncheckedExtrinsic,
            {
                // System
                System: frame_system::{Call, Config, Event<T>, Pallet, Storage} = 0,
                Timestamp: pallet_timestamp::{Call, Pallet, Storage, Inherent} = 1,
                RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} = 2,
                Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 3,
                Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>} = 4,

                // Money
                Balances: pallet_balances::{Call, Config<T>, Event<T>, Pallet, Storage} = 10,
                TransactionPayment: pallet_transaction_payment::{Config, Event<T>, Pallet, Storage} = 11,
                Treasury: pallet_treasury::{Call, Config, Event<T>, Pallet, Storage} = 12,
                Vesting: pallet_vesting::{Call, Config<T>, Event<T>, Pallet, Storage} = 13,
                Multisig: pallet_multisig::{Call, Event<T>, Pallet, Storage} = 14,
                Bounties: pallet_bounties::{Call, Event<T>, Pallet, Storage} =  15,
                AssetTxPayment: pallet_asset_tx_payment::{Event<T>, Pallet} = 16,

                // Governance
                Democracy: pallet_democracy::{Pallet, Call, Storage, Config<T>, Event<T>} = 20,
                AdvisoryCommittee: pallet_collective::<Instance1>::{Call, Config<T>, Event<T>, Origin<T>, Pallet, Storage} = 21,
                AdvisoryCommitteeMembership: pallet_membership::<Instance1>::{Call, Config<T>, Event<T>, Pallet, Storage} = 22,
                Council: pallet_collective::<Instance2>::{Call, Config<T>, Event<T>, Origin<T>, Pallet, Storage} = 23,
                CouncilMembership: pallet_membership::<Instance2>::{Call, Config<T>, Event<T>, Pallet, Storage} = 24,
                TechnicalCommittee: pallet_collective::<Instance3>::{Call, Config<T>, Event<T>, Origin<T>, Pallet, Storage} = 25,
                TechnicalCommitteeMembership: pallet_membership::<Instance3>::{Call, Config<T>, Event<T>, Pallet, Storage} = 26,

                // Other Parity pallets
                Identity: pallet_identity::{Call, Event<T>, Pallet, Storage} = 30,
                Utility: pallet_utility::{Call, Event, Pallet, Storage} = 31,
                Proxy: pallet_proxy::{Call, Event<T>, Pallet, Storage} = 32,
                Contracts: pallet_contracts = 33,

                // Third-party
                AssetManager: orml_currencies::{Call, Pallet, Storage} = 40,
                Tokens: orml_tokens::{Config<T>, Event<T>, Pallet, Storage} = 41,

                // Zeitgeist
                MarketCommons: zrml_market_commons::{Pallet, Storage} = 50,
                Authorized: zrml_authorized::{Call, Event<T>, Pallet, Storage} = 51,
                Court: zrml_court::{Call, Event<T>, Pallet, Storage} = 52,
                LiquidityMining: zrml_liquidity_mining::{Call, Config<T>, Event<T>, Pallet, Storage} = 53,
                RikiddoSigmoidFeeMarketEma: zrml_rikiddo::<Instance1>::{Pallet, Storage} = 54,
                SimpleDisputes: zrml_simple_disputes::{Call, Event<T>, Pallet, Storage} = 55,
                Swaps: zrml_swaps::{Call, Event<T>, Pallet, Storage} = 56,
                PredictionMarkets: zrml_prediction_markets::{Call, Event<T>, Pallet, Storage} = 57,
                Styx: zrml_styx::{Call, Event<T>, Pallet, Storage} = 58,
                GlobalDisputes: zrml_global_disputes::{Call, Event<T>, Pallet, Storage} = 59,
                NeoSwaps: zrml_neo_swaps::{Call, Event<T>, Pallet, Storage} = 60,
                Orderbook: zrml_orderbook::{Call, Event<T>, Pallet, Storage} = 61,
                Parimutuel: zrml_parimutuel::{Call, Event<T>, Pallet, Storage} = 62,

                $($additional_pallets)*
            }
        );
    }
}

#[macro_export]
macro_rules! create_runtime_with_additional_pallets {
    ($($additional_pallets:tt)*) => {
        #[cfg(feature = "parachain")]
        create_runtime!(
            // System
            ParachainSystem: cumulus_pallet_parachain_system::{Call, Config, Event<T>, Inherent, Pallet, Storage, ValidateUnsigned} = 100,
            ParachainInfo: parachain_info::{Config, Pallet, Storage} = 101,

            // Consensus
            ParachainStaking: pallet_parachain_staking::{Call, Config<T>, Event<T>, Pallet, Storage} = 110,
            AuthorInherent: pallet_author_inherent::{Call, Inherent, Pallet, Storage} = 111,
            AuthorFilter: pallet_author_slot_filter::{Call, Config, Event, Pallet, Storage} = 112,
            AuthorMapping: pallet_author_mapping::{Call, Config<T>, Event<T>, Pallet, Storage} = 113,

            // XCM
            CumulusXcm: cumulus_pallet_xcm::{Event<T>, Origin, Pallet} = 120,
            DmpQueue: cumulus_pallet_dmp_queue::{Call, Event<T>, Pallet, Storage} = 121,
            PolkadotXcm: pallet_xcm::{Call, Config, Event<T>, Origin, Pallet, Storage} = 122,
            XcmpQueue: cumulus_pallet_xcmp_queue::{Call, Event<T>, Pallet, Storage} = 123,
            AssetRegistry: orml_asset_registry::{Call, Config<T>, Event<T>, Pallet, Storage} = 124,
            UnknownTokens: orml_unknown_tokens::{Pallet, Storage, Event} = 125,
            XTokens: orml_xtokens::{Pallet, Storage, Call, Event<T>} = 126,

            // Others
            $($additional_pallets)*
        );

        #[cfg(not(feature = "parachain"))]
        create_runtime!(
            // Consensus
            Aura: pallet_aura::{Config<T>, Pallet, Storage} = 100,
            Grandpa: pallet_grandpa::{Call, Config, Event, Pallet, Storage} = 101,

            // Others
            $($additional_pallets)*
        );
    }
}

#[macro_export]
macro_rules! impl_config_traits {
    () => {
        use common_runtime::weights;
        #[cfg(feature = "parachain")]
        use xcm_config::config::*;

        // Configure Pallets
        #[cfg(feature = "parachain")]
        impl cumulus_pallet_dmp_queue::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type ExecuteOverweightOrigin = EnsureRootOrHalfTechnicalCommittee;
            type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
        }

        #[cfg(feature = "parachain")]
        impl cumulus_pallet_parachain_system::Config for Runtime {
            type CheckAssociatedRelayNumber =
                cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
            type DmpMessageHandler = DmpQueue;
            type RuntimeEvent = RuntimeEvent;
            type OnSystemEvent = ();
            type OutboundXcmpMessageSource = XcmpQueue;
            type ReservedDmpWeight = crate::parachain_params::ReservedDmpWeight;
            type ReservedXcmpWeight = crate::parachain_params::ReservedXcmpWeight;
            type SelfParaId = parachain_info::Pallet<Runtime>;
            type XcmpMessageHandler = XcmpQueue;
        }

        #[cfg(feature = "parachain")]
        impl cumulus_pallet_xcm::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
        }

        #[cfg(feature = "parachain")]
        impl cumulus_pallet_xcmp_queue::Config for Runtime {
            type ChannelInfo = ParachainSystem;
            type ControllerOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
            type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
            type ExecuteOverweightOrigin = EnsureRootOrHalfTechnicalCommittee;
            type PriceForSiblingDelivery = ();
            type RuntimeEvent = RuntimeEvent;
            type VersionWrapper = ();
            type WeightInfo = weights::cumulus_pallet_xcmp_queue::WeightInfo<Runtime>;
            type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
        }

        impl frame_system::Config for Runtime {
            type AccountData = pallet_balances::AccountData<Balance>;
            type AccountId = AccountId;
            type BaseCallFilter = IsCallable;
            type BlockHashCount = BlockHashCount;
            type BlockLength = RuntimeBlockLength;
            type BlockNumber = BlockNumber;
            type BlockWeights = RuntimeBlockWeights;
            type RuntimeCall = RuntimeCall;
            type DbWeight = RocksDbWeight;
            type RuntimeEvent = RuntimeEvent;
            type Hash = Hash;
            type Hashing = BlakeTwo256;
            type Header = generic::Header<BlockNumber, BlakeTwo256>;
            type Index = Index;
            type Lookup = AccountIdLookup<AccountId, ()>;
            type MaxConsumers = ConstU32<16>;
            type OnKilledAccount = ();
            type OnNewAccount = ();
            #[cfg(feature = "parachain")]
            type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
            #[cfg(not(feature = "parachain"))]
            type OnSetCode = ();
            type RuntimeOrigin = RuntimeOrigin;
            type PalletInfo = PalletInfo;
            type SS58Prefix = SS58Prefix;
            type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
            type Version = Version;
        }

        #[cfg(not(feature = "parachain"))]
        impl pallet_aura::Config for Runtime {
            type AuthorityId = sp_consensus_aura::sr25519::AuthorityId;
            type DisabledValidators = ();
            type MaxAuthorities = MaxAuthorities;
        }

        #[cfg(feature = "parachain")]
        impl pallet_author_inherent::Config for Runtime {
            type AccountLookup = AuthorMapping;
            type CanAuthor = AuthorFilter;
            type SlotBeacon = cumulus_pallet_parachain_system::RelaychainBlockNumberProvider<Self>;
            type WeightInfo = weights::pallet_author_inherent::WeightInfo<Runtime>;
        }

        #[cfg(feature = "parachain")]
        impl pallet_author_mapping::Config for Runtime {
            type DepositAmount = CollatorDeposit;
            type DepositCurrency = Balances;
            type RuntimeEvent = RuntimeEvent;
            type Keys = session_keys_primitives::VrfId;
            type WeightInfo = weights::pallet_author_mapping::WeightInfo<Runtime>;
        }

        #[cfg(feature = "parachain")]
        impl pallet_author_slot_filter::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type RandomnessSource = RandomnessCollectiveFlip;
            type PotentialAuthors = ParachainStaking;
            type WeightInfo = weights::pallet_author_slot_filter::WeightInfo<Runtime>;
        }

        frame_support::parameter_types! {
            pub const MaxSetIdSessionEntries: u32 = 12;
        }

        #[cfg(not(feature = "parachain"))]
        impl pallet_grandpa::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type KeyOwnerProofSystem = ();
            type KeyOwnerProof =
                <Self::KeyOwnerProofSystem as frame_support::traits::KeyOwnerProofSystem<(
                    KeyTypeId,
                    pallet_grandpa::AuthorityId,
                )>>::Proof;
            type KeyOwnerIdentification =
                <Self::KeyOwnerProofSystem as frame_support::traits::KeyOwnerProofSystem<(
                    KeyTypeId,
                    pallet_grandpa::AuthorityId,
                )>>::IdentificationTuple;
            type HandleEquivocation = ();
            type MaxAuthorities = MaxAuthorities;
            type MaxSetIdSessionEntries = MaxSetIdSessionEntries;
            // Currently the benchmark does yield an invalid weight implementation
            // type WeightInfo = weights::pallet_grandpa::WeightInfo<Runtime>;
            type WeightInfo = ();
        }

        #[cfg(feature = "parachain")]
        impl pallet_xcm::Config for Runtime {
            type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type RuntimeOrigin = RuntimeOrigin;
            type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
            type UniversalLocation = UniversalLocation;
            type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
            type XcmExecuteFilter = Nothing;
            // ^ Disable dispatchable execute on the XCM pallet.
            // Needs to be `Everything` for local testing.
            type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
            type XcmTeleportFilter = Everything;
            type XcmReserveTransferFilter = Nothing;
            type XcmRouter = XcmRouter;

            type Currency = Balances;
            type CurrencyMatcher = ();
            type TrustedLockers = ();
            type SovereignAccountOf = LocationToAccountId;
            type MaxLockers = ConstU32<8>;
            type WeightInfo = pallet_xcm::TestWeightInfo;
            #[cfg(feature = "runtime-benchmarks")]
            type ReachableDest = ReachableDest;

            const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
            // ^ Override for AdvertisedXcmVersion default
            type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
        }

        #[cfg(feature = "parachain")]
        impl pallet_parachain_staking::Config for Runtime {
            type BlockAuthor = AuthorInherent;
            type CandidateBondLessDelay = CandidateBondLessDelay;
            type Currency = Balances;
            type DelegationBondLessDelay = DelegationBondLessDelay;
            type RuntimeEvent = RuntimeEvent;
            type LeaveCandidatesDelay = LeaveCandidatesDelay;
            type LeaveDelegatorsDelay = LeaveDelegatorsDelay;
            type MaxBottomDelegationsPerCandidate = MaxBottomDelegationsPerCandidate;
            type MaxTopDelegationsPerCandidate = MaxTopDelegationsPerCandidate;
            type MaxDelegationsPerDelegator = MaxDelegationsPerDelegator;
            type MinBlocksPerRound = MinBlocksPerRound;
            type MinCandidateStk = MinCollatorStk;
            type MinCollatorStk = MinCollatorStk;
            type MinDelegation = MinDelegatorStk;
            type MinDelegatorStk = MinDelegatorStk;
            type MinSelectedCandidates = MinSelectedCandidates;
            type MonetaryGovernanceOrigin = EnsureRoot<AccountId>;
            type OnCollatorPayout = ();
            type PayoutCollatorReward = ();
            type OnNewRound = ();
            type RevokeDelegationDelay = RevokeDelegationDelay;
            type RewardPaymentDelay = RewardPaymentDelay;
            type WeightInfo = weights::pallet_parachain_staking::WeightInfo<Runtime>;
        }

        #[cfg(feature = "parachain")]
        impl orml_asset_registry::Config for Runtime {
            type AssetId = CurrencyId;
            type AssetProcessor = CustomAssetProcessor;
            type AuthorityOrigin = AsEnsureOriginWithArg<EnsureRootOrTwoThirdsCouncil>;
            type Balance = Balance;
            type CustomMetadata = CustomMetadata;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = ();
        }

        impl orml_currencies::Config for Runtime {
            type GetNativeCurrencyId = GetNativeCurrencyId;
            type MultiCurrency = Tokens;
            type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances>;
            type WeightInfo = weights::orml_currencies::WeightInfo<Runtime>;
        }

        pub struct CurrencyHooks<R>(sp_std::marker::PhantomData<R>);
        impl<C: orml_tokens::Config>
            orml_traits::currency::MutationHooks<AccountId, CurrencyId, Balance>
            for CurrencyHooks<C>
        {
            type OnDust = orml_tokens::TransferDust<Runtime, ZeitgeistTreasuryAccount>;
            type OnKilledTokenAccount = ();
            type OnNewTokenAccount = ();
            type OnSlash = ();
            type PostDeposit = ();
            type PostTransfer = ();
            type PreDeposit = ();
            type PreTransfer = ();
        }

        impl orml_tokens::Config for Runtime {
            type Amount = Amount;
            type Balance = Balance;
            type CurrencyHooks = CurrencyHooks<Runtime>;
            type CurrencyId = CurrencyId;
            type DustRemovalWhitelist = DustRemovalWhitelist;
            type RuntimeEvent = RuntimeEvent;
            type ExistentialDeposits = ExistentialDeposits;
            type MaxLocks = MaxLocks;
            type MaxReserves = MaxReserves;
            type ReserveIdentifier = [u8; 8];
            type WeightInfo = weights::orml_tokens::WeightInfo<Runtime>;
        }

        #[cfg(feature = "parachain")]
        impl orml_unknown_tokens::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
        }

        #[cfg(feature = "parachain")]
        impl orml_xtokens::Config for Runtime {
            type AccountIdToMultiLocation = AccountIdToMultiLocation;
            type Balance = Balance;
            type BaseXcmWeight = BaseXcmWeight;
            type CurrencyId = CurrencyId;
            type CurrencyIdConvert = AssetConvert;
            type RuntimeEvent = RuntimeEvent;
            type MaxAssetsForTransfer = MaxAssetsForTransfer;
            type MinXcmFee = ParachainMinFee;
            type MultiLocationsFilter = Everything;
            type ReserveProvider = orml_traits::location::AbsoluteReserveProvider;
            type SelfLocation = SelfLocation;
            type UniversalLocation = UniversalLocation;
            type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
            type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
        }

        impl pallet_balances::Config for Runtime {
            type AccountStore = System;
            type Balance = Balance;
            type DustRemoval = Treasury;
            type RuntimeEvent = RuntimeEvent;
            type ExistentialDeposit = ExistentialDeposit;
            type MaxLocks = MaxLocks;
            type MaxReserves = MaxReserves;
            type ReserveIdentifier = [u8; 8];
            type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
        }

        impl pallet_collective::Config<AdvisoryCommitteeInstance> for Runtime {
            type DefaultVote = PrimeDefaultVote;
            type RuntimeEvent = RuntimeEvent;
            type MaxMembers = AdvisoryCommitteeMaxMembers;
            type MaxProposals = AdvisoryCommitteeMaxProposals;
            type MotionDuration = AdvisoryCommitteeMotionDuration;
            type RuntimeOrigin = RuntimeOrigin;
            type Proposal = RuntimeCall;
            type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
        }

        impl pallet_collective::Config<CouncilInstance> for Runtime {
            type DefaultVote = PrimeDefaultVote;
            type RuntimeEvent = RuntimeEvent;
            type MaxMembers = CouncilMaxMembers;
            type MaxProposals = CouncilMaxProposals;
            type MotionDuration = CouncilMotionDuration;
            type RuntimeOrigin = RuntimeOrigin;
            type Proposal = RuntimeCall;
            type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
        }

        impl pallet_collective::Config<TechnicalCommitteeInstance> for Runtime {
            type DefaultVote = PrimeDefaultVote;
            type RuntimeEvent = RuntimeEvent;
            type MaxMembers = TechnicalCommitteeMaxMembers;
            type MaxProposals = TechnicalCommitteeMaxProposals;
            type MotionDuration = TechnicalCommitteeMotionDuration;
            type RuntimeOrigin = RuntimeOrigin;
            type Proposal = RuntimeCall;
            type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
        }

        impl pallet_contracts::Config for Runtime {
            type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
            type CallFilter = ContractsCallfilter;
            type CallStack = [pallet_contracts::Frame<Runtime>; 5];
            type ChainExtension = ();
            type Currency = Balances;
            type DeletionQueueDepth = ContractsDeletionQueueDepth;
            type DeletionWeightLimit = ContractsDeletionWeightLimit;
            type DepositPerItem = ContractsDepositPerItem;
            type DepositPerByte = ContractsDepositPerByte;
            type MaxCodeLen = ContractsMaxCodeLen;
            type MaxDebugBufferLen = ContractsMaxDebugBufferLen;
            type MaxStorageKeyLen = ContractsMaxStorageKeyLen;
            type Randomness = RandomnessCollectiveFlip;
            type RuntimeEvent = RuntimeEvent;
            type RuntimeCall = RuntimeCall;
            type Schedule = ContractsSchedule;
            type Time = Timestamp;
            type UnsafeUnstableInterface = ContractsUnsafeUnstableInterface;
            type WeightPrice = pallet_transaction_payment::Pallet<Runtime>;
            type WeightInfo = weights::pallet_contracts::WeightInfo<Runtime>;
        }

        impl pallet_democracy::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            type EnactmentPeriod = EnactmentPeriod;
            type LaunchPeriod = LaunchPeriod;
            type VotingPeriod = VotingPeriod;
            type VoteLockingPeriod = VoteLockingPeriod;
            type MinimumDeposit = MinimumDeposit;
            /// Origin that can decide what their next motion is.
            type ExternalOrigin = EnsureRootOrHalfCouncil;
            /// Origin that can have the next scheduled referendum be a straight majority-carries vote.
            type ExternalMajorityOrigin = EnsureRootOrHalfCouncil;
            /// Origina that can have the next scheduled referendum be a straight default-carries
            /// (NTB) vote.
            type ExternalDefaultOrigin = EnsureRootOrAllCouncil;
            /// Origin that can have an ExternalMajority/ExternalDefault vote
            /// be tabled immediately and with a shorter voting/enactment period.
            type FastTrackOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
            /// Origin from which the next majority-carries (or more permissive) referendum may be tabled
            /// to vote immediately and asynchronously in a similar manner to the emergency origin.
            type InstantOrigin = EnsureRootOrAllTechnicalCommittee;
            type InstantAllowed = InstantAllowed;
            type FastTrackVotingPeriod = FastTrackVotingPeriod;
            /// Origin from which any referendum may be cancelled in an emergency.
            type CancellationOrigin = EnsureRootOrThreeFourthsCouncil;
            /// Origin from which proposals may be blacklisted.
            type BlacklistOrigin = EnsureRootOrAllCouncil;
            /// Origin from which a proposal may be cancelled and its backers slashed.
            type CancelProposalOrigin = EnsureRootOrAllTechnicalCommittee;
            /// Origin for anyone able to veto proposals.
            type VetoOrigin =
                pallet_collective::EnsureMember<AccountId, TechnicalCommitteeInstance>;
            type CooloffPeriod = CooloffPeriod;
            type Slash = Treasury;
            type Scheduler = Scheduler;
            type PalletsOrigin = OriginCaller;
            type MaxVotes = MaxVotes;
            type WeightInfo = weights::pallet_democracy::WeightInfo<Runtime>;
            type MaxProposals = MaxProposals;
            type Preimages = Preimage;
            type MaxBlacklisted = ConstU32<100>;
            type MaxDeposits = ConstU32<100>;
        }

        impl pallet_identity::Config for Runtime {
            type BasicDeposit = BasicDeposit;
            type Currency = Balances;
            type RuntimeEvent = RuntimeEvent;
            type FieldDeposit = FieldDeposit;
            type ForceOrigin = EnsureRootOrTwoThirdsAdvisoryCommittee;
            type MaxAdditionalFields = MaxAdditionalFields;
            type MaxRegistrars = MaxRegistrars;
            type MaxSubAccounts = MaxSubAccounts;
            type RegistrarOrigin = EnsureRootOrHalfCouncil;
            type Slashed = Treasury;
            type SubAccountDeposit = SubAccountDeposit;
            type WeightInfo = weights::pallet_identity::WeightInfo<Runtime>;
        }

        impl pallet_membership::Config<AdvisoryCommitteeMembershipInstance> for Runtime {
            type AddOrigin = EnsureRootOrTwoThirdsCouncil;
            type RuntimeEvent = RuntimeEvent;
            type MaxMembers = AdvisoryCommitteeMaxMembers;
            type MembershipChanged = AdvisoryCommittee;
            type MembershipInitialized = AdvisoryCommittee;
            type PrimeOrigin = EnsureRootOrTwoThirdsCouncil;
            type RemoveOrigin = EnsureRootOrTwoThirdsCouncil;
            type ResetOrigin = EnsureRootOrTwoThirdsCouncil;
            type SwapOrigin = EnsureRootOrTwoThirdsCouncil;
            type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
        }

        impl pallet_membership::Config<CouncilMembershipInstance> for Runtime {
            type AddOrigin = EnsureRootOrThreeFourthsCouncil;
            type RuntimeEvent = RuntimeEvent;
            type MaxMembers = CouncilMaxMembers;
            type MembershipChanged = Council;
            type MembershipInitialized = Council;
            type PrimeOrigin = EnsureRootOrThreeFourthsCouncil;
            type RemoveOrigin = EnsureRootOrThreeFourthsCouncil;
            type ResetOrigin = EnsureRootOrThreeFourthsCouncil;
            type SwapOrigin = EnsureRootOrThreeFourthsCouncil;
            type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
        }

        impl pallet_membership::Config<TechnicalCommitteeMembershipInstance> for Runtime {
            type AddOrigin = EnsureRootOrTwoThirdsCouncil;
            type RuntimeEvent = RuntimeEvent;
            type MaxMembers = TechnicalCommitteeMaxMembers;
            type MembershipChanged = TechnicalCommittee;
            type MembershipInitialized = TechnicalCommittee;
            type PrimeOrigin = EnsureRootOrTwoThirdsCouncil;
            type RemoveOrigin = EnsureRootOrTwoThirdsCouncil;
            type ResetOrigin = EnsureRootOrTwoThirdsCouncil;
            type SwapOrigin = EnsureRootOrTwoThirdsCouncil;
            type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
        }

        impl pallet_multisig::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type RuntimeCall = RuntimeCall;
            type Currency = Balances;
            type DepositBase = DepositBase;
            type DepositFactor = DepositFactor;
            type MaxSignatories = ConstU32<100>;
            type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
        }

        impl pallet_preimage::Config for Runtime {
            type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            type ManagerOrigin = EnsureRoot<AccountId>;
            type BaseDeposit = PreimageBaseDeposit;
            type ByteDeposit = PreimageByteDeposit;
        }

        impl InstanceFilter<RuntimeCall> for ProxyType {
            fn filter(&self, c: &RuntimeCall) -> bool {
                match self {
                    ProxyType::Any => true,
                    ProxyType::CancelProxy => {
                        matches!(
                            c,
                            RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })
                        )
                    }
                    ProxyType::Governance => matches!(
                        c,
                        RuntimeCall::Democracy(..)
                            | RuntimeCall::Council(..)
                            | RuntimeCall::TechnicalCommittee(..)
                            | RuntimeCall::AdvisoryCommittee(..)
                            | RuntimeCall::Treasury(..)
                    ),
                    #[cfg(feature = "parachain")]
                    ProxyType::Staking => matches!(c, RuntimeCall::ParachainStaking(..)),
                    #[cfg(not(feature = "parachain"))]
                    ProxyType::Staking => false,
                    ProxyType::CreateEditMarket => matches!(
                        c,
                        RuntimeCall::PredictionMarkets(
                            zrml_prediction_markets::Call::create_market { .. }
                        ) | RuntimeCall::PredictionMarkets(
                            zrml_prediction_markets::Call::edit_market { .. }
                        )
                    ),
                    ProxyType::ReportOutcome => matches!(
                        c,
                        RuntimeCall::PredictionMarkets(
                            zrml_prediction_markets::Call::report { .. }
                        )
                    ),
                    ProxyType::Dispute => matches!(
                        c,
                        RuntimeCall::PredictionMarkets(
                            zrml_prediction_markets::Call::dispute { .. }
                        )
                    ),
                    ProxyType::ProvideLiquidity => matches!(
                        c,
                        RuntimeCall::Swaps(zrml_swaps::Call::pool_join { .. })
                            | RuntimeCall::Swaps(zrml_swaps::Call::pool_exit { .. })
                    ),
                    ProxyType::BuySellCompleteSets => matches!(
                        c,
                        RuntimeCall::PredictionMarkets(
                            zrml_prediction_markets::Call::buy_complete_set { .. }
                        ) | RuntimeCall::PredictionMarkets(
                            zrml_prediction_markets::Call::sell_complete_set { .. }
                        )
                    ),
                    ProxyType::Trading => matches!(
                        c,
                        RuntimeCall::Swaps(zrml_swaps::Call::swap_exact_amount_in { .. })
                            | RuntimeCall::Swaps(zrml_swaps::Call::swap_exact_amount_out { .. })
                            | RuntimeCall::Orderbook(zrml_orderbook::Call::place_order { .. })
                            | RuntimeCall::Orderbook(zrml_orderbook::Call::fill_order { .. })
                            | RuntimeCall::Orderbook(zrml_orderbook::Call::remove_order { .. })
                    ),
                    ProxyType::HandleAssets => matches!(
                        c,
                        RuntimeCall::Swaps(zrml_swaps::Call::pool_join { .. })
                            | RuntimeCall::Swaps(zrml_swaps::Call::pool_exit { .. })
                            | RuntimeCall::Swaps(zrml_swaps::Call::swap_exact_amount_in { .. })
                            | RuntimeCall::Swaps(zrml_swaps::Call::swap_exact_amount_out { .. })
                            | RuntimeCall::PredictionMarkets(
                                zrml_prediction_markets::Call::buy_complete_set { .. }
                            )
                            | RuntimeCall::PredictionMarkets(
                                zrml_prediction_markets::Call::sell_complete_set { .. }
                            )
                            | RuntimeCall::Orderbook(zrml_orderbook::Call::place_order { .. })
                            | RuntimeCall::Orderbook(zrml_orderbook::Call::fill_order { .. })
                            | RuntimeCall::Orderbook(zrml_orderbook::Call::remove_order { .. })
                    ),
                }
            }

            fn is_superset(&self, o: &Self) -> bool {
                match (self, o) {
                    (x, y) if x == y => true,
                    (ProxyType::Any, _) => true,
                    (_, ProxyType::Any) => false,
                    _ => false,
                }
            }
        }

        impl pallet_proxy::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type RuntimeCall = RuntimeCall;
            type Currency = Balances;
            type ProxyType = ProxyType;
            type ProxyDepositBase = ProxyDepositBase;
            type ProxyDepositFactor = ProxyDepositFactor;
            type MaxProxies = ConstU32<32>;
            type WeightInfo = weights::pallet_proxy::WeightInfo<Runtime>;
            type MaxPending = ConstU32<32>;
            type CallHasher = BlakeTwo256;
            type AnnouncementDepositBase = AnnouncementDepositBase;
            type AnnouncementDepositFactor = AnnouncementDepositFactor;
        }

        impl pallet_randomness_collective_flip::Config for Runtime {}

        impl pallet_scheduler::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type RuntimeOrigin = RuntimeOrigin;
            type PalletsOrigin = OriginCaller;
            type RuntimeCall = RuntimeCall;
            type MaximumWeight = MaximumSchedulerWeight;
            type ScheduleOrigin = EnsureRoot<AccountId>;
            type MaxScheduledPerBlock = MaxScheduledPerBlock;
            type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
            type OriginPrivilegeCmp = EqualPrivilegeOnly;
            type Preimages = Preimage;
        }

        // Timestamp
        /// Custom getter for minimum timestamp delta.
        /// This ensures that consensus systems like Aura don't break assertions
        /// in a benchmark environment
        pub struct MinimumPeriod;
        impl MinimumPeriod {
            /// Returns the value of this parameter type.
            pub fn get() -> u64 {
                #[cfg(feature = "runtime-benchmarks")]
                {
                    use frame_benchmarking::benchmarking::get_whitelist;
                    // Should that condition be true, we can assume that we are in a benchmark environment.
                    if !get_whitelist().is_empty() {
                        return u64::MAX;
                    }
                }

                MinimumPeriodValue::get()
            }
        }
        impl<I: From<u64>> frame_support::traits::Get<I> for MinimumPeriod {
            fn get() -> I {
                I::from(Self::get())
            }
        }
        impl frame_support::traits::TypedGet for MinimumPeriod {
            type Type = u64;
            fn get() -> u64 {
                Self::get()
            }
        }

        impl pallet_timestamp::Config for Runtime {
            type MinimumPeriod = MinimumPeriod;
            type Moment = u64;
            #[cfg(feature = "parachain")]
            type OnTimestampSet = ();
            #[cfg(not(feature = "parachain"))]
            type OnTimestampSet = Aura;
            type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
        }

        common_runtime::impl_foreign_fees!();

        impl pallet_asset_tx_payment::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type Fungibles = Tokens;
            type OnChargeAssetTransaction = TokensTxCharger;
        }

        impl pallet_transaction_payment::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Runtime>;
            type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
            type OnChargeTransaction =
                pallet_transaction_payment::CurrencyAdapter<Balances, DealWithFees>;
            type OperationalFeeMultiplier = OperationalFeeMultiplier;
            type WeightToFee = IdentityFee<Balance>;
        }

        impl pallet_treasury::Config for Runtime {
            type ApproveOrigin = EnsureRootOrTwoThirdsCouncil;
            type Burn = Burn;
            type BurnDestination = ();
            type Currency = Balances;
            type RuntimeEvent = RuntimeEvent;
            type MaxApprovals = MaxApprovals;
            type OnSlash = Treasury;
            type PalletId = TreasuryPalletId;
            type ProposalBond = ProposalBond;
            type ProposalBondMinimum = ProposalBondMinimum;
            type ProposalBondMaximum = ProposalBondMaximum;
            type RejectOrigin = EnsureRootOrTwoThirdsCouncil;
            type SpendFunds = Bounties;
            type SpendOrigin =
                EnsureWithSuccess<EnsureRoot<AccountId>, AccountId, MaxTreasurySpend>;
            type SpendPeriod = SpendPeriod;
            type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
        }

        impl pallet_bounties::Config for Runtime {
            type BountyDepositBase = BountyDepositBase;
            type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
            type BountyUpdatePeriod = BountyUpdatePeriod;
            type BountyValueMinimum = BountyValueMinimum;
            type ChildBountyManager = ();
            type CuratorDepositMax = CuratorDepositMax;
            type CuratorDepositMin = CuratorDepositMin;
            type CuratorDepositMultiplier = CuratorDepositMultiplier;
            type DataDepositPerByte = DataDepositPerByte;
            type RuntimeEvent = RuntimeEvent;
            type MaximumReasonLength = MaximumReasonLength;
            type WeightInfo = weights::pallet_bounties::WeightInfo<Runtime>;
        }

        impl pallet_utility::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type RuntimeCall = RuntimeCall;
            type PalletsOrigin = OriginCaller;
            type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
        }

        impl pallet_vesting::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            type BlockNumberToBalance = sp_runtime::traits::ConvertInto;
            type MinVestedTransfer = MinVestedTransfer;
            type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
            type WeightInfo = weights::pallet_vesting::WeightInfo<Runtime>;

            // `VestingInfo` encode length is 36bytes. 28 schedules gets encoded as 1009 bytes, which is the
            // highest number of schedules that encodes less than 2^10.
            const MAX_VESTING_SCHEDULES: u32 = 28;
        }

        #[cfg(feature = "parachain")]
        impl parachain_info::Config for Runtime {}

        impl zrml_authorized::Config for Runtime {
            type AuthorizedDisputeResolutionOrigin = EnsureRootOrMoreThanHalfAdvisoryCommittee;
            type Currency = Balances;
            type CorrectionPeriod = CorrectionPeriod;
            type DisputeResolution = zrml_prediction_markets::Pallet<Runtime>;
            type RuntimeEvent = RuntimeEvent;
            type MarketCommons = MarketCommons;
            type PalletId = AuthorizedPalletId;
            type WeightInfo = zrml_authorized::weights::WeightInfo<Runtime>;
        }

        impl zrml_court::Config for Runtime {
            type AppealBond = AppealBond;
            type BlocksPerYear = BlocksPerYear;
            type VotePeriod = CourtVotePeriod;
            type AggregationPeriod = CourtAggregationPeriod;
            type AppealPeriod = CourtAppealPeriod;
            type LockId = CourtLockId;
            type PalletId = CourtPalletId;
            type Currency = Balances;
            type DisputeResolution = zrml_prediction_markets::Pallet<Runtime>;
            type RuntimeEvent = RuntimeEvent;
            type InflationPeriod = InflationPeriod;
            type MarketCommons = MarketCommons;
            type MaxAppeals = MaxAppeals;
            type MaxDelegations = MaxDelegations;
            type MaxSelectedDraws = MaxSelectedDraws;
            type MaxCourtParticipants = MaxCourtParticipants;
            type MaxYearlyInflation = MaxYearlyInflation;
            type MinJurorStake = MinJurorStake;
            type MonetaryGovernanceOrigin = EnsureRoot<AccountId>;
            type Random = RandomnessCollectiveFlip;
            type RequestInterval = RequestInterval;
            type Slash = Treasury;
            type TreasuryPalletId = TreasuryPalletId;
            type WeightInfo = zrml_court::weights::WeightInfo<Runtime>;
        }

        impl zrml_liquidity_mining::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            type MarketCommons = MarketCommons;
            type MarketId = MarketId;
            type PalletId = LiquidityMiningPalletId;
            type WeightInfo = zrml_liquidity_mining::weights::WeightInfo<Runtime>;
        }

        impl zrml_market_commons::Config for Runtime {
            type Balance = Balance;
            type MarketId = MarketId;
            type Timestamp = Timestamp;
        }

        // NoopLiquidityMining implements LiquidityMiningPalletApi with no-ops.
        // Has to be public because it will be exposed by Runtime.
        pub struct NoopLiquidityMining;

        impl zrml_liquidity_mining::LiquidityMiningPalletApi for NoopLiquidityMining {
            type AccountId = AccountId;
            type Balance = Balance;
            type BlockNumber = BlockNumber;
            type MarketId = MarketId;

            fn add_shares(_: Self::AccountId, _: Self::MarketId, _: Self::Balance) {}

            fn distribute_market_incentives(
                _: &Self::MarketId,
            ) -> frame_support::pallet_prelude::DispatchResult {
                Ok(())
            }

            fn remove_shares(_: &Self::AccountId, _: &Self::MarketId, _: Self::Balance) {}
        }

        impl zrml_prediction_markets::Config for Runtime {
            type AdvisoryBond = AdvisoryBond;
            type AdvisoryBondSlashPercentage = AdvisoryBondSlashPercentage;
            type ApproveOrigin = EnsureRootOrMoreThanOneThirdAdvisoryCommittee;
            type Authorized = Authorized;
            type Currency = Balances;
            type Court = Court;
            type CloseEarlyDisputeBond = CloseEarlyDisputeBond;
            type CloseMarketEarlyOrigin = EnsureRootOrMoreThanOneThirdAdvisoryCommittee;
            type CloseOrigin = EnsureRoot<AccountId>;
            type CloseEarlyProtectionTimeFramePeriod = CloseEarlyProtectionTimeFramePeriod;
            type CloseEarlyProtectionBlockPeriod = CloseEarlyProtectionBlockPeriod;
            type CloseEarlyRequestBond = CloseEarlyRequestBond;
            type DestroyOrigin = EnsureRootOrAllAdvisoryCommittee;
            type DeployPool = NeoSwaps;
            type DisputeBond = DisputeBond;
            type RuntimeEvent = RuntimeEvent;
            type GlobalDisputes = GlobalDisputes;
            // LiquidityMining is currently unstable.
            // NoopLiquidityMining will be applied only to mainnet once runtimes are separated.
            type LiquidityMining = NoopLiquidityMining;
            // type LiquidityMining = LiquidityMining;
            type MaxCategories = MaxCategories;
            type MaxCreatorFee = MaxCreatorFee;
            type MaxDisputes = MaxDisputes;
            type MaxMarketLifetime = MaxMarketLifetime;
            type MinDisputeDuration = MinDisputeDuration;
            type MaxDisputeDuration = MaxDisputeDuration;
            type MaxGracePeriod = MaxGracePeriod;
            type MaxOracleDuration = MaxOracleDuration;
            type MinOracleDuration = MinOracleDuration;
            type MinCategories = MinCategories;
            type MaxEditReasonLen = MaxEditReasonLen;
            type MaxRejectReasonLen = MaxRejectReasonLen;
            type OracleBond = OracleBond;
            type OutsiderBond = OutsiderBond;
            type PalletId = PmPalletId;
            type CloseEarlyBlockPeriod = CloseEarlyBlockPeriod;
            type CloseEarlyTimeFramePeriod = CloseEarlyTimeFramePeriod;
            type RejectOrigin = EnsureRootOrMoreThanTwoThirdsAdvisoryCommittee;
            type RequestEditOrigin = EnsureRootOrMoreThanOneThirdAdvisoryCommittee;
            type ResolveOrigin = EnsureRoot<AccountId>;
            type AssetManager = AssetManager;
            #[cfg(feature = "parachain")]
            type AssetRegistry = AssetRegistry;
            type SimpleDisputes = SimpleDisputes;
            type Slash = Treasury;
            type ValidityBond = ValidityBond;
            type WeightInfo = zrml_prediction_markets::weights::WeightInfo<Runtime>;
        }

        impl zrml_rikiddo::Config<RikiddoSigmoidFeeMarketVolumeEma> for Runtime {
            type Timestamp = Timestamp;
            type Balance = Balance;
            type FixedTypeU = FixedU128<U33>;
            type FixedTypeS = FixedI128<U33>;
            type BalanceFractionalDecimals = BalanceFractionalDecimals;
            type PoolId = PoolId;
            type Rikiddo = RikiddoSigmoidMV<
                Self::FixedTypeU,
                Self::FixedTypeS,
                FeeSigmoid<Self::FixedTypeS>,
                EmaMarketVolume<Self::FixedTypeU>,
            >;
        }

        impl zrml_simple_disputes::Config for Runtime {
            type Currency = Balances;
            type OutcomeBond = OutcomeBond;
            type OutcomeFactor = OutcomeFactor;
            type DisputeResolution = zrml_prediction_markets::Pallet<Runtime>;
            type RuntimeEvent = RuntimeEvent;
            type MarketCommons = MarketCommons;
            type MaxDisputes = MaxDisputes;
            type PalletId = SimpleDisputesPalletId;
            type WeightInfo = zrml_simple_disputes::weights::WeightInfo<Runtime>;
        }

        impl zrml_global_disputes::Config for Runtime {
            type AddOutcomePeriod = AddOutcomePeriod;
            type Currency = Balances;
            type DisputeResolution = zrml_prediction_markets::Pallet<Runtime>;
            type RuntimeEvent = RuntimeEvent;
            type GlobalDisputeLockId = GlobalDisputeLockId;
            type GlobalDisputesPalletId = GlobalDisputesPalletId;
            type MarketCommons = MarketCommons;
            type MaxGlobalDisputeVotes = MaxGlobalDisputeVotes;
            type MaxOwners = MaxOwners;
            type MinOutcomeVoteAmount = MinOutcomeVoteAmount;
            type RemoveKeysLimit = RemoveKeysLimit;
            type GdVotingPeriod = GdVotingPeriod;
            type VotingOutcomeFee = VotingOutcomeFee;
            type WeightInfo = zrml_global_disputes::weights::WeightInfo<Runtime>;
        }

        impl zrml_swaps::Config for Runtime {
            type Asset = Asset<MarketId>;
            type RuntimeEvent = RuntimeEvent;
            type ExitFee = ExitFee;
            type MinAssets = MinAssets;
            type MaxAssets = MaxAssets;
            type MaxInRatio = MaxInRatio;
            type MaxOutRatio = MaxOutRatio;
            type MaxSwapFee = MaxSwapFee;
            type MaxTotalWeight = MaxTotalWeight;
            type MaxWeight = MaxWeight;
            type MinWeight = MinWeight;
            type PalletId = SwapsPalletId;
            type AssetManager = AssetManager;
            type WeightInfo = zrml_swaps::weights::WeightInfo<Runtime>;
        }

        impl zrml_styx::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type SetBurnAmountOrigin = EnsureRootOrHalfCouncil;
            type Currency = Balances;
            type WeightInfo = zrml_styx::weights::WeightInfo<Runtime>;
        }

        common_runtime::impl_market_creator_fees!();

        impl zrml_neo_swaps::Config for Runtime {
            type CompleteSetOperations = PredictionMarkets;
            type ExternalFees = MarketCreatorFee;
            type MarketCommons = MarketCommons;
            type MultiCurrency = AssetManager;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = zrml_neo_swaps::weights::WeightInfo<Runtime>;
            type MaxLiquidityTreeDepth = MaxLiquidityTreeDepth;
            type MaxSwapFee = NeoSwapsMaxSwapFee;
            type PalletId = NeoSwapsPalletId;
        }

        impl zrml_orderbook::Config for Runtime {
            type AssetManager = AssetManager;
            type ExternalFees = MarketCreatorFee;
            type RuntimeEvent = RuntimeEvent;
            type MarketCommons = MarketCommons;
            type PalletId = OrderbookPalletId;
            type WeightInfo = zrml_orderbook::weights::WeightInfo<Runtime>;
        }

        impl zrml_parimutuel::Config for Runtime {
            type ExternalFees = MarketCreatorFee;
            type RuntimeEvent = RuntimeEvent;
            type MarketCommons = MarketCommons;
            type AssetManager = AssetManager;
            type MinBetSize = MinBetSize;
            type PalletId = ParimutuelPalletId;
            type WeightInfo = zrml_parimutuel::weights::WeightInfo<Runtime>;
        }
    };
}

// Implement runtime apis
#[macro_export]
macro_rules! create_runtime_api {
    ($($additional_apis:tt)*) => {
        // Prints debug output of the `contracts` pallet to stdout if the node is
        // started with `-lruntime::contracts=debug`.
        const CONTRACTS_DEBUG_OUTPUT: bool = true;

        impl_runtime_apis! {
            #[cfg(feature = "parachain")]
            impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
                fn collect_collation_info(
                    header: &<Block as BlockT>::Header
                ) -> cumulus_primitives_core::CollationInfo {
                    ParachainSystem::collect_collation_info(header)
                }
            }

            #[cfg(feature = "parachain")]
            impl nimbus_primitives::NimbusApi<Block> for Runtime {
                fn can_author(
                    author: nimbus_primitives::NimbusId,
                    slot: u32,
                    parent_header: &<Block as BlockT>::Header
                ) -> bool {

                    // Ensure that an update is enforced when we are close to maximum block number
                    let block_number = if let Some(bn) = parent_header.number.checked_add(1) {
                        bn
                    } else {
                        log::error!("ERROR: No block numbers left");
                        return false;
                    };

                    use frame_support::traits::OnInitialize;
                    System::initialize(
                        &block_number,
                        &parent_header.hash(),
                        &parent_header.digest,
                    );
                    RandomnessCollectiveFlip::on_initialize(block_number);

                    // Because the staking solution calculates the next staking set at the beginning
                    // of the first block in the new round, the only way to accurately predict the
                    // authors is to compute the selection during prediction.
                    if pallet_parachain_staking::Pallet::<Self>::round().should_update(block_number) {
                        // get author account id
                        use nimbus_primitives::AccountLookup;
                        let author_account_id = if let Some(account) =
                            pallet_author_mapping::Pallet::<Self>::lookup_account(&author) {
                            account
                        } else {
                            // return false if author mapping not registered like in can_author impl
                            return false
                        };
                        // predict eligibility post-selection by computing selection results now
                        let (eligible, _) =
                            pallet_author_slot_filter::compute_pseudo_random_subset::<Self>(
                                pallet_parachain_staking::Pallet::<Self>::compute_top_candidates(),
                                &slot
                            );
                        eligible.contains(&author_account_id)
                    } else {
                        AuthorInherent::can_author(&author, &slot)
                    }
                }
            }

            #[cfg(feature = "runtime-benchmarks")]
            impl frame_benchmarking::Benchmark<Block> for Runtime {
                fn benchmark_metadata(extra: bool) -> (
                    Vec<frame_benchmarking::BenchmarkList>,
                    Vec<frame_support::traits::StorageInfo>,
                ) {
                    use frame_benchmarking::{list_benchmark, baseline::Pallet as BaselineBench, Benchmarking, BenchmarkList};
                    use frame_support::traits::StorageInfoTrait;
                    use frame_system_benchmarking::Pallet as SystemBench;
                    use orml_benchmarking::list_benchmark as orml_list_benchmark;

                    let mut list = Vec::<BenchmarkList>::new();

                    list_benchmark!(list, extra, frame_benchmarking, BaselineBench::<Runtime>);
                    list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
                    orml_list_benchmark!(list, extra, orml_currencies, crate::benchmarks::currencies);
                    orml_list_benchmark!(list, extra, orml_tokens, crate::benchmarks::tokens);
                    list_benchmark!(list, extra, pallet_balances, Balances);
                    list_benchmark!(list, extra, pallet_bounties, Bounties);
                    list_benchmark!(list, extra, pallet_collective, AdvisoryCommittee);
                    list_benchmark!(list, extra, pallet_contracts, Contracts);
                    list_benchmark!(list, extra, pallet_democracy, Democracy);
                    list_benchmark!(list, extra, pallet_identity, Identity);
                    list_benchmark!(list, extra, pallet_membership, AdvisoryCommitteeMembership);
                    list_benchmark!(list, extra, pallet_multisig, Multisig);
                    list_benchmark!(list, extra, pallet_preimage, Preimage);
                    list_benchmark!(list, extra, pallet_proxy, Proxy);
                    list_benchmark!(list, extra, pallet_scheduler, Scheduler);
                    list_benchmark!(list, extra, pallet_timestamp, Timestamp);
                    list_benchmark!(list, extra, pallet_treasury, Treasury);
                    list_benchmark!(list, extra, pallet_utility, Utility);
                    list_benchmark!(list, extra, pallet_vesting, Vesting);
                    list_benchmark!(list, extra, zrml_swaps, Swaps);
                    list_benchmark!(list, extra, zrml_authorized, Authorized);
                    list_benchmark!(list, extra, zrml_court, Court);
                    list_benchmark!(list, extra, zrml_simple_disputes, SimpleDisputes);
                    list_benchmark!(list, extra, zrml_global_disputes, GlobalDisputes);
                    list_benchmark!(list, extra, zrml_orderbook, Orderbook);
                    list_benchmark!(list, extra, zrml_parimutuel, Parimutuel);
                    #[cfg(not(feature = "parachain"))]
                    list_benchmark!(list, extra, zrml_prediction_markets, PredictionMarkets);
                    list_benchmark!(list, extra, zrml_liquidity_mining, LiquidityMining);
                    list_benchmark!(list, extra, zrml_styx, Styx);
                    list_benchmark!(list, extra, zrml_neo_swaps, NeoSwaps);

                    cfg_if::cfg_if! {
                        if #[cfg(feature = "parachain")] {
                            list_benchmark!(list, extra, cumulus_pallet_xcmp_queue, XcmpQueue);
                            list_benchmark!(list, extra, pallet_author_inherent, AuthorInherent);
                            list_benchmark!(list, extra, pallet_author_mapping, AuthorMapping);
                            list_benchmark!(list, extra, pallet_author_slot_filter, AuthorFilter);
                            list_benchmark!(list, extra, pallet_parachain_staking, ParachainStaking);
                        } else {
                            list_benchmark!(list, extra, pallet_grandpa, Grandpa);
                        }
                    }

                    (list, AllPalletsWithSystem::storage_info())
                }

                fn dispatch_benchmark(
                    config: frame_benchmarking::BenchmarkConfig,
                ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
                    use frame_benchmarking::{
                        add_benchmark, baseline::{Pallet as BaselineBench, Config as BaselineConfig}, vec, BenchmarkBatch, Benchmarking, TrackedStorageKey, Vec
                    };
                    use frame_system_benchmarking::Pallet as SystemBench;
                    use orml_benchmarking::{add_benchmark as orml_add_benchmark};

                    impl frame_system_benchmarking::Config for Runtime {}
                    impl BaselineConfig for Runtime {}

                    let whitelist: Vec<TrackedStorageKey> = vec![
                        // Block Number
                        hex_literal::hex!(  "26aa394eea5630e07c48ae0c9558cef7"
                                            "02a5c1b19ab7a04f536c519aca4983ac")
                            .to_vec().into(),
                        // Total Issuance
                        hex_literal::hex!(  "c2261276cc9d1f8598ea4b6a74b15c2f"
                                            "57c875e4cff74148e4628f264b974c80")
                            .to_vec().into(),
                        // Execution Phase
                        hex_literal::hex!(  "26aa394eea5630e07c48ae0c9558cef7"
                                            "ff553b5a9862a516939d82b3d3d8661a")
                            .to_vec().into(),
                        // Event Count
                        hex_literal::hex!(  "26aa394eea5630e07c48ae0c9558cef7"
                                            "0a98fdbe9ce6c55837576c60c7af3850")
                            .to_vec().into(),
                        // System Events
                        hex_literal::hex!(  "26aa394eea5630e07c48ae0c9558cef7"
                                            "80d41e5e16056765bc8461851072c9d7")
                            .to_vec().into(),
                        // System BlockWeight
                        hex_literal::hex!(  "26aa394eea5630e07c48ae0c9558cef7"
                                            "34abf5cb34d6244378cddbf18e849d96")
                            .to_vec().into(),
                        // ParachainStaking Round
                        hex_literal::hex!(  "a686a3043d0adcf2fa655e57bc595a78"
                                            "13792e785168f725b60e2969c7fc2552")
                            .to_vec().into(),
                        // Treasury Account (zge/tsry)
                        hex_literal::hex!(  "26aa394eea5630e07c48ae0c9558cef7"
                                            "b99d880ec681799c0cf30e8886371da9"
                                            "7be2919ac397ba499ea5e57132180ec6"
                                            "6d6f646c7a67652f7473727900000000"
                                            "00000000000000000000000000000000"
                        ).to_vec().into(),
                        // ParachainInfo ParachainId
                        hex_literal::hex!(  "0d715f2646c8f85767b5d2764bb27826"
                                            "04a74d81251e398fd8a0a4d55023bb3f")
                            .to_vec().into(),
                    ];

                    let mut batches = Vec::<BenchmarkBatch>::new();
                    let params = (&config, &whitelist);

                    add_benchmark!(params, batches, frame_benchmarking, BaselineBench::<Runtime>);
                    add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
                    orml_add_benchmark!(params, batches, orml_currencies, crate::benchmarks::currencies);
                    orml_add_benchmark!(params, batches, orml_tokens, crate::benchmarks::tokens);
                    add_benchmark!(params, batches, pallet_balances, Balances);
                    add_benchmark!(params, batches, pallet_bounties, Bounties);
                    add_benchmark!(params, batches, pallet_collective, AdvisoryCommittee);
                    add_benchmark!(params, batches, pallet_contracts, Contracts);
                    add_benchmark!(params, batches, pallet_democracy, Democracy);
                    add_benchmark!(params, batches, pallet_identity, Identity);
                    add_benchmark!(params, batches, pallet_membership, AdvisoryCommitteeMembership);
                    add_benchmark!(params, batches, pallet_multisig, Multisig);
                    add_benchmark!(params, batches, pallet_preimage, Preimage);
                    add_benchmark!(params, batches, pallet_proxy, Proxy);
                    add_benchmark!(params, batches, pallet_scheduler, Scheduler);
                    add_benchmark!(params, batches, pallet_timestamp, Timestamp);
                    add_benchmark!(params, batches, pallet_treasury, Treasury);
                    add_benchmark!(params, batches, pallet_utility, Utility);
                    add_benchmark!(params, batches, pallet_vesting, Vesting);
                    add_benchmark!(params, batches, zrml_swaps, Swaps);
                    add_benchmark!(params, batches, zrml_authorized, Authorized);
                    add_benchmark!(params, batches, zrml_court, Court);
                    add_benchmark!(params, batches, zrml_simple_disputes, SimpleDisputes);
                    add_benchmark!(params, batches, zrml_global_disputes, GlobalDisputes);
                    add_benchmark!(params, batches, zrml_orderbook, Orderbook);
                    add_benchmark!(params, batches, zrml_parimutuel, Parimutuel);
                    #[cfg(not(feature = "parachain"))]
                    add_benchmark!(params, batches, zrml_prediction_markets, PredictionMarkets);
                    add_benchmark!(params, batches, zrml_liquidity_mining, LiquidityMining);
                    add_benchmark!(params, batches, zrml_styx, Styx);
                    add_benchmark!(params, batches, zrml_neo_swaps, NeoSwaps);


                    cfg_if::cfg_if! {
                        if #[cfg(feature = "parachain")] {
                            add_benchmark!(params, batches, cumulus_pallet_xcmp_queue, XcmpQueue);
                            add_benchmark!(params, batches, pallet_author_inherent, AuthorInherent);
                            add_benchmark!(params, batches, pallet_author_mapping, AuthorMapping);
                            add_benchmark!(params, batches, pallet_author_slot_filter, AuthorFilter);
                            add_benchmark!(params, batches, pallet_parachain_staking, ParachainStaking);

                        } else {
                            add_benchmark!(params, batches, pallet_grandpa, Grandpa);
                        }
                    }

                    if batches.is_empty() {
                        return Err("Benchmark not found for this pallet.".into());
                    }
                    Ok(batches)
                }
            }

            impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
                fn account_nonce(account: AccountId) -> Index {
                    System::account_nonce(account)
                }
            }

            impl pallet_contracts::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash>
                for Runtime
            {
                fn call(
                    origin: AccountId,
                    dest: AccountId,
                    value: Balance,
                    gas_limit: Option<Weight>,
                    storage_deposit_limit: Option<Balance>,
                    input_data: Vec<u8>,
                ) -> pallet_contracts_primitives::ContractExecResult<Balance> {
                    let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
                    Contracts::bare_call(
                        origin,
                        dest,
                        value,
                        gas_limit,
                        storage_deposit_limit,
                        input_data,
                        CONTRACTS_DEBUG_OUTPUT,
                        pallet_contracts::Determinism::Deterministic,
                    )
                }

                fn instantiate(
                    origin: AccountId,
                    value: Balance,
                    gas_limit: Option<Weight>,
                    storage_deposit_limit: Option<Balance>,
                    code: pallet_contracts_primitives::Code<Hash>,
                    data: Vec<u8>,
                    salt: Vec<u8>,
                ) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance>
                {
                    let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
                    Contracts::bare_instantiate(
                        origin,
                        value,
                        gas_limit,
                        storage_deposit_limit,
                        code,
                        data,
                        salt,
                        CONTRACTS_DEBUG_OUTPUT,
                    )
                }

                fn upload_code(
                    origin: AccountId,
                    code: Vec<u8>,
                    storage_deposit_limit: Option<Balance>,
                    determinism: pallet_contracts::Determinism,
                ) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance>
                {
                    Contracts::bare_upload_code(
                        origin,
                        code,
                        storage_deposit_limit,
                        determinism,
                    )
                }

                fn get_storage(
                    address: AccountId,
                    key: Vec<u8>,
                ) -> pallet_contracts_primitives::GetStorageResult {
                    Contracts::get_storage(address, key)
                }
            }

            impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
                fn query_fee_details(
                    uxt: <Block as BlockT>::Extrinsic,
                    len: u32,
                ) -> pallet_transaction_payment::FeeDetails<Balance> {
                    TransactionPayment::query_fee_details(uxt, len)
                }

                fn query_info(
                    uxt: <Block as BlockT>::Extrinsic,
                    len: u32,
                ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
                    TransactionPayment::query_info(uxt, len)
                }

                fn query_weight_to_fee(weight: Weight) -> Balance {
                    TransactionPayment::weight_to_fee(weight)
                }

                fn query_length_to_fee(length: u32) -> Balance {
                    TransactionPayment::length_to_fee(length)
                }
            }

            impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
            for Runtime
            {
                fn query_call_info(
                    call: RuntimeCall,
                    len: u32,
                ) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
                    TransactionPayment::query_call_info(call, len)
                }

                fn query_call_fee_details(
                    call: RuntimeCall,
                    len: u32,
                ) -> pallet_transaction_payment::FeeDetails<Balance> {
                    TransactionPayment::query_call_fee_details(call, len)
                }

                fn query_weight_to_fee(weight: Weight) -> Balance {
                    TransactionPayment::weight_to_fee(weight)
                }

                fn query_length_to_fee(length: u32) -> Balance {
                    TransactionPayment::length_to_fee(length)
                }
            }

            #[cfg(feature = "parachain")]
            impl session_keys_primitives::VrfApi<Block> for Runtime {
                fn get_last_vrf_output() -> Option<<Block as BlockT>::Hash> {
                    None
                }
                fn vrf_key_lookup(
                    nimbus_id: nimbus_primitives::NimbusId
                ) -> Option<session_keys_primitives::VrfId> {
                    use session_keys_primitives::KeysLookup;
                    AuthorMapping::lookup_keys(&nimbus_id)
                }
            }

            impl sp_api::Core<Block> for Runtime {
                fn execute_block(block: Block) {
                    Executive::execute_block(block)
                }

                fn initialize_block(header: &<Block as BlockT>::Header) {
                    Executive::initialize_block(header)
                }

                fn version() -> RuntimeVersion {
                    VERSION
                }
            }

            impl sp_api::Metadata<Block> for Runtime {
                fn metadata() -> OpaqueMetadata {
                    OpaqueMetadata::new(Runtime::metadata().into())
                }
            }

            impl sp_block_builder::BlockBuilder<Block> for Runtime {
                fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
                    Executive::apply_extrinsic(extrinsic)
                }

                fn check_inherents(
                    block: Block,
                    data: sp_inherents::InherentData,
                ) -> sp_inherents::CheckInherentsResult {
                    data.check_extrinsics(&block)
                }

                fn finalize_block() -> <Block as BlockT>::Header {
                    Executive::finalize_block()
                }

                fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
                    data.create_extrinsics()
                }
            }

            #[cfg(not(feature = "parachain"))]
            impl sp_consensus_aura::AuraApi<Block, sp_consensus_aura::sr25519::AuthorityId> for Runtime {
                fn authorities() -> Vec<sp_consensus_aura::sr25519::AuthorityId> {
                    Aura::authorities().into_inner()
                }

                fn slot_duration() -> sp_consensus_aura::SlotDuration {
                    sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
                }
            }

            #[cfg(not(feature = "parachain"))]
            impl sp_finality_grandpa::GrandpaApi<Block> for Runtime {
                fn current_set_id() -> pallet_grandpa::fg_primitives::SetId {
                    Grandpa::current_set_id()
                }

                fn generate_key_ownership_proof(
                    _set_id: pallet_grandpa::fg_primitives::SetId,
                    _authority_id: pallet_grandpa::AuthorityId,
                ) -> Option<pallet_grandpa::fg_primitives::OpaqueKeyOwnershipProof> {
                    None
                }

                fn grandpa_authorities() -> pallet_grandpa::AuthorityList {
                    Grandpa::grandpa_authorities()
                }

                fn submit_report_equivocation_unsigned_extrinsic(
                    _equivocation_proof: pallet_grandpa::fg_primitives::EquivocationProof<
                        <Block as BlockT>::Hash,
                        sp_runtime::traits::NumberFor<Block>,
                    >,
                    _key_owner_proof: pallet_grandpa::fg_primitives::OpaqueKeyOwnershipProof,
                ) -> Option<()> {
                    None
                }
            }

            impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
                fn offchain_worker(header: &<Block as BlockT>::Header) {
                    Executive::offchain_worker(header)
                }
            }

            impl sp_session::SessionKeys<Block> for Runtime {
                fn decode_session_keys(encoded: Vec<u8>) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
                    opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
                }

                fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
                    opaque::SessionKeys::generate(seed)
                }
            }

            impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
                fn validate_transaction(
                    source: TransactionSource,
                    tx: <Block as BlockT>::Extrinsic,
                    block_hash: <Block as BlockT>::Hash,
                ) -> TransactionValidity {
                    // Filtered calls should not enter the tx pool as they'll fail if inserted.
                    // If this call is not allowed, we return early.
                    if !<Runtime as frame_system::Config>::BaseCallFilter::contains(&tx.function) {
                        return frame_support::pallet_prelude::InvalidTransaction::Call.into();
                    }

                    Executive::validate_transaction(source, tx, block_hash)
                }
            }

            impl zrml_swaps_runtime_api::SwapsApi<Block, PoolId, AccountId, Balance, MarketId>
            for Runtime
            {
                fn get_spot_price(
                    pool_id: &PoolId,
                    asset_in: &Asset<MarketId>,
                    asset_out: &Asset<MarketId>,
                    with_fees: bool,
                ) -> SerdeWrapper<Balance> {
                    SerdeWrapper(Swaps::get_spot_price(pool_id, asset_in, asset_out, with_fees).ok().unwrap_or(0))
                }

                fn pool_account_id(pool_id: &PoolId) -> AccountId {
                    Swaps::pool_account_id(pool_id)
                }

                fn pool_shares_id(pool_id: PoolId) -> Asset<SerdeWrapper<MarketId>> {
                    Asset::PoolShare(SerdeWrapper(pool_id))
                }
            }

            #[cfg(feature = "try-runtime")]
            impl frame_try_runtime::TryRuntime<Block> for Runtime {
                fn on_runtime_upgrade(checks: UpgradeCheckSelect) -> (Weight, Weight) {
                    let weight = Executive::try_runtime_upgrade(checks).unwrap();
                    (weight, RuntimeBlockWeights::get().max_block)
                }

                fn execute_block(
                    block: Block,
                    state_root_check: bool,
                    signature_check: bool,
                    select: TryStateSelect,
                ) -> Weight {
                    // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
                    // have a backtrace here.
                    Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
                }
            }

            $($additional_apis)*
        }

        // Check the timestamp and parachain inherents
        #[cfg(feature = "parachain")]
        struct CheckInherents;

        #[cfg(feature = "parachain")]
        impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
            fn check_inherents(
                block: &Block,
                relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
            ) -> sp_inherents::CheckInherentsResult {
                let relay_chain_slot = relay_state_proof
                    .read_slot()
                    .expect("Could not read the relay chain slot from the proof");

                let inherent_data =
                    cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
                        relay_chain_slot,
                        core::time::Duration::from_secs(6),
                    )
                    .create_inherent_data()
                    .expect("Could not create the timestamp inherent data");

                inherent_data.check_extrinsics(block)
            }
        }

        // Nimbus's Executive wrapper allows relay validators to verify the seal digest
        #[cfg(feature = "parachain")]
        cumulus_pallet_parachain_system::register_validate_block! {
            Runtime = Runtime,
            BlockExecutor = pallet_author_inherent::BlockExecutor::<Runtime, Executive>,
            CheckInherents = CheckInherents,
        }
    }
}

#[macro_export]
macro_rules! create_common_benchmark_logic {
    {} => {
        #[cfg(feature = "runtime-benchmarks")]
        pub(crate) mod benchmarks {
            pub(crate) mod currencies {
                use super::utils::{lookup_of_account, set_balance};
                use crate::{
                    AccountId, Amount, AssetManager, Balance, CurrencyId, ExistentialDeposit,
                    GetNativeCurrencyId, Runtime
                };
                use frame_benchmarking::{account, vec, whitelisted_caller};
                use frame_system::RawOrigin;
                use sp_runtime::traits::UniqueSaturatedInto;
                use orml_benchmarking::runtime_benchmarks;
                use orml_traits::MultiCurrency;
                use zeitgeist_primitives::{
                    constants::BASE,
                    types::Asset,
                };

                const SEED: u32 = 0;
                const NATIVE: CurrencyId = GetNativeCurrencyId::get();
                const ASSET: CurrencyId = Asset::CategoricalOutcome(0, 0);

                runtime_benchmarks! {
                    { Runtime, orml_currencies }

                    // `transfer` non-native currency
                    transfer_non_native_currency {
                        let amount: Balance = 1_000 * BASE;
                        let from: AccountId = whitelisted_caller();
                        set_balance(ASSET, &from, amount);

                        let to: AccountId = account("to", 0, SEED);
                        let to_lookup = lookup_of_account(to.clone());
                    }: transfer(RawOrigin::Signed(from), to_lookup, ASSET, amount)
                    verify {
                        assert_eq!(<AssetManager as MultiCurrency<_>>::total_balance(ASSET, &to), amount);
                    }

                    // `transfer` native currency and in worst case
                    #[extra]
                    transfer_native_currency_worst_case {
                        let existential_deposit = ExistentialDeposit::get();
                        let amount: Balance = existential_deposit.saturating_mul(1000);
                        let from: AccountId = whitelisted_caller();
                        set_balance(NATIVE, &from, amount);

                        let to: AccountId = account("to", 0, SEED);
                        let to_lookup = lookup_of_account(to.clone());
                    }: transfer(RawOrigin::Signed(from), to_lookup, NATIVE, amount)
                    verify {
                        assert_eq!(<AssetManager as MultiCurrency<_>>::total_balance(NATIVE, &to), amount);
                    }

                    // `transfer_native_currency` in worst case
                    // * will create the `to` account.
                    // * will kill the `from` account.
                    transfer_native_currency {
                        let existential_deposit = ExistentialDeposit::get();
                        let amount: Balance = existential_deposit.saturating_mul(1000);
                        let from: AccountId = whitelisted_caller();
                        set_balance(NATIVE, &from, amount);

                        let to: AccountId = account("to", 0, SEED);
                        let to_lookup = lookup_of_account(to.clone());
                    }: _(RawOrigin::Signed(from), to_lookup, amount)
                    verify {
                        assert_eq!(<AssetManager as MultiCurrency<_>>::total_balance(NATIVE, &to), amount);
                    }

                    // `update_balance` for non-native currency
                    update_balance_non_native_currency {
                        let balance: Balance = 2 * BASE;
                        let amount: Amount = balance.unique_saturated_into();
                        let who: AccountId = account("who", 0, SEED);
                        let who_lookup = lookup_of_account(who.clone());
                    }: update_balance(RawOrigin::Root, who_lookup, ASSET, amount)
                    verify {
                        assert_eq!(<AssetManager as MultiCurrency<_>>::total_balance(ASSET, &who), balance);
                    }

                    // `update_balance` for native currency
                    // * will create the `who` account.
                    update_balance_native_currency_creating {
                        let existential_deposit = ExistentialDeposit::get();
                        let balance: Balance = existential_deposit.saturating_mul(1000);
                        let amount: Amount = balance.unique_saturated_into();
                        let who: AccountId = account("who", 0, SEED);
                        let who_lookup = lookup_of_account(who.clone());
                    }: update_balance(RawOrigin::Root, who_lookup, NATIVE, amount)
                    verify {
                        assert_eq!(<AssetManager as MultiCurrency<_>>::total_balance(NATIVE, &who), balance);
                    }

                    // `update_balance` for native currency
                    // * will kill the `who` account.
                    update_balance_native_currency_killing {
                        let existential_deposit = ExistentialDeposit::get();
                        let balance: Balance = existential_deposit.saturating_mul(1000);
                        let amount: Amount = balance.unique_saturated_into();
                        let who: AccountId = account("who", 0, SEED);
                        let who_lookup = lookup_of_account(who.clone());
                        set_balance(NATIVE, &who, balance);
                    }: update_balance(RawOrigin::Root, who_lookup, NATIVE, -amount)
                    verify {
                        assert_eq!(<AssetManager as MultiCurrency<_>>::free_balance(NATIVE, &who), 0);
                    }
                }

                #[cfg(test)]
                mod tests {
                    use super::*;
                    use crate::benchmarks::utils::tests::new_test_ext;
                    use orml_benchmarking::impl_benchmark_test_suite;

                    impl_benchmark_test_suite!(new_test_ext(),);
                }
            }

            pub(crate) mod tokens {
                use super::utils::{lookup_of_account, set_balance as update_balance};
                use crate::{AccountId, Balance, CurrencyId, Tokens, Runtime};
                use frame_benchmarking::{account, vec, whitelisted_caller};
                use frame_system::RawOrigin;
                use orml_benchmarking::runtime_benchmarks;
                use orml_traits::MultiCurrency;
                use zeitgeist_primitives::{constants::BASE, types::Asset};

                const SEED: u32 = 0;
                const ASSET: CurrencyId = Asset::CategoricalOutcome(0, 0);

                runtime_benchmarks! {
                    { Runtime, orml_tokens }

                    transfer {
                        let amount: Balance = BASE;

                        let from: AccountId = whitelisted_caller();
                        update_balance(ASSET, &from, amount);

                        let to: AccountId = account("to", 0, SEED);
                        let to_lookup = lookup_of_account(to.clone());
                    }: _(RawOrigin::Signed(from), to_lookup, ASSET, amount)
                    verify {
                        assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(ASSET, &to), amount);
                    }

                    transfer_all {
                        let amount: Balance = BASE;

                        let from: AccountId = whitelisted_caller();
                        update_balance(ASSET, &from, amount);

                        let to: AccountId = account("to", 0, SEED);
                        let to_lookup = lookup_of_account(to);
                    }: _(RawOrigin::Signed(from.clone()), to_lookup, ASSET, false)
                    verify {
                        assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(ASSET, &from), 0);
                    }

                    transfer_keep_alive {
                        let from: AccountId = whitelisted_caller();
                        update_balance(ASSET, &from, 2 * BASE);

                        let to: AccountId = account("to", 0, SEED);
                        let to_lookup = lookup_of_account(to.clone());
                    }: _(RawOrigin::Signed(from), to_lookup, ASSET, BASE)
                    verify {
                        assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(ASSET, &to), BASE);
                    }

                    force_transfer {
                        let from: AccountId = account("from", 0, SEED);
                        let from_lookup = lookup_of_account(from.clone());
                        update_balance(ASSET, &from, 2 * BASE);

                        let to: AccountId = account("to", 0, SEED);
                        let to_lookup = lookup_of_account(to.clone());
                    }: _(RawOrigin::Root, from_lookup, to_lookup, ASSET, BASE)
                    verify {
                        assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(ASSET, &to), BASE);
                    }

                    set_balance {
                        let who: AccountId = account("who", 0, SEED);
                        let who_lookup = lookup_of_account(who.clone());

                    }: _(RawOrigin::Root, who_lookup, ASSET, BASE, BASE)
                    verify {
                        assert_eq!(<Tokens as MultiCurrency<_>>::total_balance(ASSET, &who), 2 * BASE);
                    }
                }

                #[cfg(test)]
                mod tests {
                    use super::*;
                    use crate::benchmarks::utils::tests::new_test_ext;
                    use orml_benchmarking::impl_benchmark_test_suite;

                    impl_benchmark_test_suite!(new_test_ext(),);
                }
            }

            pub(crate) mod utils {
                use crate::{AccountId, AssetManager, Balance, CurrencyId, Runtime,
                };
                use frame_support::assert_ok;
                use orml_traits::MultiCurrencyExtended;
                use sp_runtime::traits::{SaturatedConversion, StaticLookup};

                pub fn lookup_of_account(
                    who: AccountId,
                ) -> <<Runtime as frame_system::Config>::Lookup as StaticLookup>::Source {
                    <Runtime as frame_system::Config>::Lookup::unlookup(who)
                }

                pub fn set_balance(currency_id: CurrencyId, who: &AccountId, balance: Balance) {
                    assert_ok!(<AssetManager as MultiCurrencyExtended<_>>::update_balance(
                        currency_id,
                        who,
                        balance.saturated_into()
                    ));
                }

                #[cfg(test)]
                pub mod tests {
                    pub fn new_test_ext() -> sp_io::TestExternalities {
                        frame_system::GenesisConfig::default().build_storage::<crate::Runtime>().unwrap().into()
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! create_common_tests {
    {} => {
        #[cfg(test)]
        mod common_tests {
            common_runtime::fee_tests!();

            mod dust_removal {
                use crate::*;
                use frame_support::PalletId;
                use test_case::test_case;

                #[test_case(AuthorizedPalletId::get(); "authorized")]
                #[test_case(CourtPalletId::get(); "court")]
                #[test_case(LiquidityMiningPalletId::get(); "liquidity_mining")]
                #[test_case(PmPalletId::get(); "prediction_markets")]
                #[test_case(SimpleDisputesPalletId::get(); "simple_disputes")]
                #[test_case(SwapsPalletId::get(); "swaps")]
                #[test_case(TreasuryPalletId::get(); "treasury")]
                fn whitelisted_pallet_accounts_dont_get_reaped(pallet_id: PalletId) {
                    let mut t: sp_io::TestExternalities =
                        frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap().into();
                    t.execute_with(|| {
                        let pallet_main_account: AccountId = pallet_id.into_account_truncating();
                        let pallet_sub_account: AccountId = pallet_id.into_sub_account_truncating(42);
                        assert!(DustRemovalWhitelist::contains(&pallet_main_account));
                        assert!(DustRemovalWhitelist::contains(&pallet_sub_account));
                    });
                }

                #[test]
                fn non_whitelisted_accounts_get_reaped() {
                    let mut t: sp_io::TestExternalities =
                        frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap().into();
                    t.execute_with(|| {
                        let not_whitelisted = AccountId::from([0u8; 32]);
                        assert!(!DustRemovalWhitelist::contains(&not_whitelisted))
                    });
                }
            }
        }
    }
}
