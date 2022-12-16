use cumulus_primitives_core::ParaId;
use nimbus_primitives::NimbusId;
use pallet_author_slot_filter::EligibilityValue;
use parachain_template_runtime::{
	AccountId, Balance, CouncilConfig, MaxNominations, NominationPoolsConfig, Signature,
	StakerStatus, StakingConfig, TechnicalCommitteeConfig, UNIT,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup, Properties};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
// use rand::seq::SliceRandom;
use rand::{seq::SliceRandom, Rng};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

pub const PARA_ID: u32 = 2022;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<parachain_template_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> NimbusId {
	get_pair_from_seed::<NimbusId>(seed)
}

/// Helper function to generate a crypto pair from seed
pub fn get_pair_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: NimbusId) -> parachain_template_runtime::SessionKeys {
	parachain_template_runtime::SessionKeys { nimbus: keys }
}

const ENDOWMENT: Balance = 10_000_000 * UNIT;
const STASH: Balance = ENDOWMENT / 1000;

fn get_properties() -> Properties {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "BAND".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());
	properties
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Aband Development",
		// ID
		"aband dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				PARA_ID.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(get_properties()),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: PARA_ID,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places

	ChainSpec::from_genesis(
		// Name
		"Aband Testnet",
		// ID
		"aband_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				PARA_ID.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("aband_testnet"),
		// Fork ID
		None,
		// Properties
		Some(get_properties()),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: PARA_ID,
		},
	)
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AccountId, NimbusId)>,
	initial_nominators: Vec<AccountId>,
	mut endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_template_runtime::GenesisConfig {
	// endow all authorities and nominators.
	invulnerables
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = invulnerables
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			let limit = (MaxNominations::get() as usize).min(invulnerables.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = invulnerables
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	parachain_template_runtime::GenesisConfig {
		system: parachain_template_runtime::SystemConfig {
			code: parachain_template_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: parachain_template_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: parachain_template_runtime::ParachainInfoConfig { parachain_id: id },
		// collator_selection: parachain_template_runtime::CollatorSelectionConfig {
		// 	invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
		// 	candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
		// 	..Default::default()
		// },
		session: parachain_template_runtime::SessionConfig {
			keys: invulnerables
				.clone()
				.into_iter()
				.map(|(acc, _, nimbus)| {
					(
						acc.clone(),                   // account id
						acc,                           // validator id
						template_session_keys(nimbus), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		// aura: Default::default(),
		// aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: parachain_template_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		author_filter: parachain_template_runtime::AuthorFilterConfig {
			eligible_count: EligibilityValue::default(),
		},
		validators: parachain_template_runtime::ValidatorsConfig {
			mapping: invulnerables
				.iter()
				.map(|(x, _y, z)| (x.clone(), z.clone()))
				.collect::<Vec<(AccountId, NimbusId)>>()
				.to_vec(),
		},
		staking: StakingConfig {
			validator_count: invulnerables.len() as u32,
			minimum_validator_count: invulnerables.len() as u32,
			invulnerables: invulnerables.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig::default(),
		treasury: Default::default(),
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * UNIT,
			min_join_bond: UNIT,
			..Default::default()
		},
	}
}
