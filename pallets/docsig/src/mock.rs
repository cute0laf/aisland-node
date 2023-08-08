use crate as pallet_docsig;
use frame_support::traits::{ConstU16, ConstU64};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use frame_system as system;
//use frame_system::weights;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		DocSig: pallet_docsig,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_docsig::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime starting from block(1)
pub fn new_test_ext() -> sp_io::TestExternalities {
	// need to set block number to 1 to test events
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let mut ext: sp_io::TestExternalities = t.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
	//frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

pub fn last_event() -> RuntimeEvent {
	System::events().pop().expect("Event expected").event
}
