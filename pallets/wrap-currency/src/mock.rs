// This file is part of Web3Games.

// Copyright (C) 2021 Web3Games https://web3games.org
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// use crate as pallet_currency_token;
// use frame_support::{parameter_types, PalletId};
// use frame_system as system;
// use sp_core::H256;
// use sp_runtime::{
// 	testing::Header,
// 	traits::{BlakeTwo256, IdentityLookup},
// };
//
// type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
// type Block = frame_system::mocking::MockBlock<Test>;
//
// // Configure a mock runtime to test the pallet.
// frame_support::construct_runtime!(
// 	pub enum Test where
// 		Block = Block,
// 		NodeBlock = Block,
// 		UncheckedExtrinsic = UncheckedExtrinsic,
// 	{
// 		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
// 		WrapCurrency: pallet_currency_token::{Pallet, Call, Storage, Event<T>},
// 	}
// );
//
// parameter_types! {
// 	pub const BlockHashCount: u64 = 250;
// 	pub const SS58Prefix: u8 = 42;
// }
//
// impl system::Config for Test {
// 	type BaseCallFilter = ();
// 	type BlockWeights = ();
// 	type BlockLength = ();
// 	type DbWeight = ();
// 	type Origin = Origin;
// 	type Call = Call;
// 	type Index = u64;
// 	type BlockNumber = u64;
// 	type Hash = H256;
// 	type Hashing = BlakeTwo256;
// 	type AccountId = u64;
// 	type Lookup = IdentityLookup<Self::AccountId>;
// 	type Header = Header;
// 	type Event = Event;
// 	type BlockHashCount = BlockHashCount;
// 	type Version = ();
// 	type PalletInfo = PalletInfo;
// 	type AccountData = ();
// 	type OnNewAccount = ();
// 	type OnKilledAccount = ();
// 	type SystemWeightInfo = ();
// 	type SS58Prefix = SS58Prefix;
// }
//
// parameter_types! {
// 	pub const CurrencyTokenModuleId: PalletId = PalletId(*b"w3g/curr");
// 	pub const DexModuleId: PalletId = PalletId(*b"w3g/dexm");
// }
//
// impl pallet_currency_token::Config for Test {
// 	type Event = Event;
// 	type PalletId = CurrencyTokenModuleId;
// 	type Currency = Currencies;
// }
//
// // Build genesis storage according to the mock runtime.
// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	system::GenesisConfig::default()
// 		.build_storage::<Test>()
// 		.unwrap()
// 		.into()
// }
