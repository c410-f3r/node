mod test_extrinsic;

use crate::{mock::test_extrinsic::TestXt, Call, DefaultWeightInfo, Module, Trait};
use alloc::{boxed::Box, vec};
use frame_support::{impl_outer_origin, ord_parameter_types, parameter_types, weights::Weight};
use frame_system::{offchain::AppCrypto, EnsureRoot};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
use valiu_node_commons::{Asset, Collateral};
use valiu_node_runtime_types::{
    AccountData, AccountId, Balance, BlockNumber, Hash, Header, Index, Signature,
};

pub const USD_ASSET: Asset = Asset::Collateral(USD_COLLATERAL);
pub const USD_COLLATERAL: Collateral = Collateral::Usd;

pub type Extrinsic = TestXt<Call<Test>, ()>;
pub type ProviderMembers = pallet_membership::Module<Test, pallet_membership::DefaultInstance>;
pub type TestProvider = Module<Test>;
pub type Tokens = orml_tokens::Module<Test>;

impl_outer_origin! {
    pub enum Origin for Test where system = frame_system {}
}

ord_parameter_types! {
    pub const Root: AccountId = <AccountId>::from_raw([0; 32]);
}

parameter_types! {
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const BlockHashCount: BlockNumber = 250;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const OffchainUnsignedGracePeriod: u32 = 5;
    pub const OffchainUnsignedInterval: u32 = 128;
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;

impl frame_system::Trait for Test {
    type AccountData = AccountData;
    type AccountId = AccountId;
    type AvailableBlockRatio = AvailableBlockRatio;
    type BaseCallFilter = ();
    type BlockExecutionWeight = ();
    type BlockHashCount = BlockHashCount;
    type BlockNumber = BlockNumber;
    type Call = ();
    type DbWeight = ();
    type Event = ();
    type ExtrinsicBaseWeight = ();
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Index = Index;
    type Lookup = IdentityLookup<AccountId>;
    type MaximumBlockLength = MaximumBlockLength;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type Origin = Origin;
    type PalletInfo = ();
    type SystemWeightInfo = ();
    type Version = ();
}

impl<LC> frame_system::offchain::SendTransactionTypes<LC> for Test
where
    Call<Test>: From<LC>,
{
    type Extrinsic = Extrinsic;
    type OverarchingCall = Call<Test>;
}

impl frame_system::offchain::SigningTypes for Test {
    type Public = AccountId;
    type Signature = Signature;
}

impl orml_tokens::Trait for Test {
    type Amount = i64;
    type Balance = Balance;
    type CurrencyId = Asset;
    type Event = ();
    type OnReceived = ();
    type WeightInfo = ();
}

impl pallet_membership::Trait<pallet_membership::DefaultInstance> for Test {
    type AddOrigin = EnsureRoot<AccountId>;
    type Event = ();
    type MembershipChanged = ();
    type MembershipInitialized = ();
    type PrimeOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type ResetOrigin = EnsureRoot<AccountId>;
    type SwapOrigin = EnsureRoot<AccountId>;
}

impl Trait for Test {
    type Asset = Tokens;
    type Collateral = Tokens;
    type Event = ();
    type OffchainAuthority = TestAuth;
    type OffchainUnsignedGracePeriod = OffchainUnsignedGracePeriod;
    type OffchainUnsignedInterval = OffchainUnsignedInterval;
    type WeightInfo = DefaultWeightInfo;
}

pub struct TestAuth;

impl AppCrypto<AccountId, Signature> for TestAuth {
    type GenericPublic = AccountId;
    type GenericSignature = Signature;
    type RuntimeAppPublic = crate::Public;
}
