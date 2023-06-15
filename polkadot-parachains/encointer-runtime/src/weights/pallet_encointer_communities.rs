
//! Autogenerated weights for `pallet_encointer_communities`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("encointer-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-collator
// benchmark
// --chain=encointer-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_communities
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_encointer_communities.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_encointer_communities`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_encointer_communities::WeightInfo for WeightInfo<T> {
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:1)
	// Storage: EncointerCommunities MaxSpeedMps (r:1 w:0)
	// Storage: EncointerCommunities MinSolarTripTimeS (r:1 w:0)
	// Storage: EncointerCommunities CommunityIdentifiersByGeohash (r:1 w:1)
	// Storage: EncointerCommunities Locations (r:1 w:1)
	// Storage: EncointerCommunities NominalIncome (r:0 w:1)
	// Storage: EncointerCommunities CommunityMetadata (r:0 w:1)
	// Storage: EncointerCommunities Bootstrappers (r:0 w:1)
	// Storage: EncointerBalances DemurragePerBlock (r:0 w:1)
	fn new_community() -> Weight {
		Weight::from_parts(8_678_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: EncointerScheduler CurrentPhase (r:1 w:0)
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerCommunities MaxSpeedMps (r:1 w:0)
	// Storage: EncointerCommunities MinSolarTripTimeS (r:1 w:0)
	// Storage: EncointerCommunities CommunityIdentifiersByGeohash (r:1 w:0)
	// Storage: EncointerCommunities Locations (r:1 w:1)
	fn add_location() -> Weight {
		Weight::from_parts(9_091_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerScheduler CurrentPhase (r:1 w:0)
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerCommunities Locations (r:1 w:1)
	fn remove_location() -> Weight {
		Weight::from_parts(61_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerCommunities CommunityMetadata (r:0 w:1)
	fn update_community_metadata() -> Weight {
		Weight::from_parts(34_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerBalances DemurragePerBlock (r:0 w:1)
	fn update_demurrage() -> Weight {
		Weight::from_parts(31_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerCommunities NominalIncome (r:0 w:1)
	fn update_nominal_income() -> Weight {
		Weight::from_parts(31_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerCommunities MinSolarTripTimeS (r:0 w:1)
	fn set_min_solar_trip_time_s() -> Weight {
		Weight::from_parts(22_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerCommunities MaxSpeedMps (r:0 w:1)
	fn set_max_speed_mps() -> Weight {
		Weight::from_parts(22_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerCommunities Locations (r:2 w:1)
	// Storage: EncointerCommunities CommunityIdentifiersByGeohash (r:1 w:1)
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:1)
	// Storage: EncointerCommunities NominalIncome (r:0 w:1)
	// Storage: EncointerCommunities CommunityMetadata (r:0 w:1)
	// Storage: EncointerCommunities Bootstrappers (r:0 w:1)
	fn purge_community() -> Weight {
		Weight::from_parts(44_041_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}
