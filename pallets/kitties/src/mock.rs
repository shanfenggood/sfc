use crate::{Module, Trait};
use sp_core::H256;
use frame_support::{impl_outer_origin,impl_outer_event, parameter_types, weights::Weight,
	traits::{OnFinalize,OnInitialize}};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;

impl_outer_origin! {
	pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	pub const MaxPoeLength:u8 = 4;
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}
type Randomness = pallet_randomness_collective_flip::Module<Test>;

mod simple_event {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for Test {
		simple_event<T>,
		system<T>,
	}
}

impl Trait for Test {
	type Event = TestEvent;
	type Randomness = Randomness;
	type KittyIndex = u32;
}

pub type KittiesModule = Module<Test>;

pub type System = frame_system::Module<Test>;
pub fn run_to_block(n: u64){
	while System::block_number() <n {
		KittiesModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number()+1);
		System::on_initialize(System::block_number());
		KittiesModule::on_initialize(System::block_number());
	}
}
// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {

	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
