use crate as pallet_fungible;
use sp_core::H256;
use frame_support::{construct_runtime, parameter_types};
use sp_runtime::{
    traits::{ BlakeTwo256, IdentityLookup },
    testing::Header,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		FungiblePallet: pallet_fungible::{Module, Call, Storage, Event<T>},
	}
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
    type Version = ();
	type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type BaseCallFilter = ();
    type DbWeight = ();
    type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

impl pallet_fungible::Config for Test {
    type Event = ();
    // type Currency = pallet_balances::Module<Test>;
    type TokenBalance = u64;
    type TokenId = u64;
}

// impl pallet_balances::Config for Test {
//     type Balance = u64;
//     type Event = ();
//     type DustRemoval = ();
//     type ExistentialDeposit = ExistentialDeposit;
//     type AccountStore = System;
// }

// type System = frame_system::Module<Test>;
// type Balances = pallet_balances::Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
