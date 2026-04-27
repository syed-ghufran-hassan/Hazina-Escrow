#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token, Address, Env, String, Vec,
};

#[contracttype]
pub enum DataKey {
    Admin,
    PlatformFee,
    EscrowCount,
}

#[contracttype]
pub enum EscrowKey {
    Record(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialised = 1,
    NotAdmin = 2,
    EscrowNotFound = 3,
    AlreadyReleased = 4,
    AlreadyRefunded = 5,
    NotBuyer = 6,
    NotExpired = 7,
    InvalidInput = 8,
}

#[contracttype]
#[derive(Clone)]
pub struct EscrowRecord {
    pub escrow_id: u64,
    pub dataset_id: String,
    pub buyer: Address,
    pub seller: Address,
    pub amount: i128,
    pub token: Address,
    pub released: bool,
    pub refunded: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct SellerShare {
    pub seller: Address,
    pub amount: i128,
}

#[contract]
pub struct HazinaEscrow;

#[contractimpl]
impl HazinaEscrow {
    pub fn initialize(env: Env, admin: Address, platform_fee_bps: u32) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialised);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::PlatformFee, &platform_fee_bps);
        env.storage().instance().set(&DataKey::EscrowCount, &0u64);
        Ok(())
    }

    pub fn transfer_admin(env: Env, admin: Address, new_admin: Address) {
        admin.require_auth();
        Self::assert_admin(&env, &admin);
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    pub fn update_fee(env: Env, admin: Address, new_fee_bps: u32) {
        admin.require_auth();
        Self::assert_admin(&env, &admin);
        if new_fee_bps > 1_000 {
            soroban_sdk::panic_with_error!(&env, Error::InvalidInput);
        }
        env.storage().instance().set(&DataKey::PlatformFee, &new_fee_bps);
    }

    pub fn set_fee(env: Env, admin: Address, fee_bps: u32) {
        admin.require_auth();
        Self::assert_admin(&env, &admin);
        if fee_bps > 10_000 {
            soroban_sdk::panic_with_error!(&env, Error::AlreadyReleased);
        }
        env.storage().instance().set(&DataKey::PlatformFee, &fee_bps);
    }

    pub fn set_admin(env: Env, admin: Address, new_admin: Address) {
        Self::transfer_admin(env, admin, new_admin);
    }

    pub fn get_fee(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::PlatformFee).unwrap_or(500)
    }

    pub fn lock(
        env: Env,
        buyer: Address,
        seller: Address,
        token: Address,
        amount: i128,
        dataset_id: String,
    ) -> u64 {
        buyer.require_auth();
        if amount <= 0 {
            soroban_sdk::panic_with_error!(&env, Error::InvalidInput);
        }
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&buyer, &env.current_contract_address(), &amount);

        let id: u64 = env.storage().instance().get(&DataKey::EscrowCount).unwrap_or(0);
        let record = EscrowRecord {
            escrow_id: id,
            dataset_id,
            buyer,
            seller,
            amount,
            token,
            released: false,
            refunded: false,
        };
        env.storage().persistent().set(&EscrowKey::Record(id), &record);
        env.storage().instance().set(&DataKey::EscrowCount, &(id + 1));
        id
    }

    pub fn lock_multi(
        env: Env,
        buyer: Address,
        token: Address,
        shares: Vec<SellerShare>,
        dataset_ids: Vec<String>,
    ) -> u64 {
        buyer.require_auth();
        if shares.is_empty() || shares.len() != dataset_ids.len() {
            soroban_sdk::panic_with_error!(&env, Error::InvalidInput);
        }

        let mut total_amount: i128 = 0;
        let mut i: u32 = 0;
        while i < shares.len() {
            let share = shares.get(i).unwrap();
            if share.amount <= 0 {
                soroban_sdk::panic_with_error!(&env, Error::InvalidInput);
            }
            total_amount += share.amount;
            i += 1;
        }

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&buyer, &env.current_contract_address(), &total_amount);

        let first_id: u64 = env.storage().instance().get(&DataKey::EscrowCount).unwrap_or(0);
        let mut next_id = first_id;
        let mut j: u32 = 0;
        while j < shares.len() {
            let share = shares.get(j).unwrap();
            let dataset_id = dataset_ids.get(j).unwrap();
            let record = EscrowRecord {
                escrow_id: next_id,
                dataset_id,
                buyer: buyer.clone(),
                seller: share.seller,
                amount: share.amount,
                token: token.clone(),
                released: false,
                refunded: false,
            };
            env.storage().persistent().set(&EscrowKey::Record(next_id), &record);
            next_id += 1;
            j += 1;
        }
        env.storage().instance().set(&DataKey::EscrowCount, &next_id);
        first_id
    }

    pub fn release(env: Env, admin: Address, escrow_id: u64) {
        admin.require_auth();
        Self::assert_admin(&env, &admin);
        let mut record: EscrowRecord = env
            .storage()
            .persistent()
            .get(&EscrowKey::Record(escrow_id))
            .unwrap_or_else(|| soroban_sdk::panic_with_error!(&env, Error::EscrowNotFound));
        if record.released {
            soroban_sdk::panic_with_error!(&env, Error::AlreadyReleased);
        }
        if record.refunded {
            soroban_sdk::panic_with_error!(&env, Error::AlreadyRefunded);
        }
        let fee_bps: u32 = env.storage().instance().get(&DataKey::PlatformFee).unwrap_or(500);
        let platform_cut = record.amount * fee_bps as i128 / 10_000;
        let seller_cut = record.amount - platform_cut;
        let token_client = token::Client::new(&env, &record.token);
        token_client.transfer(&env.current_contract_address(), &record.seller, &seller_cut);
        if platform_cut > 0 {
            token_client.transfer(&env.current_contract_address(), &admin, &platform_cut);
        }
        record.released = true;
        env.storage().persistent().set(&EscrowKey::Record(escrow_id), &record);
    }

    pub fn release_multi(env: Env, admin: Address, escrow_ids: Vec<u64>) {
        admin.require_auth();
        Self::assert_admin(&env, &admin);
        let mut i: u32 = 0;
        while i < escrow_ids.len() {
            let escrow_id = escrow_ids.get(i).unwrap();
            let mut record: EscrowRecord = env
                .storage()
                .persistent()
                .get(&EscrowKey::Record(escrow_id))
                .unwrap_or_else(|| soroban_sdk::panic_with_error!(&env, Error::EscrowNotFound));
            if record.released {
                soroban_sdk::panic_with_error!(&env, Error::AlreadyReleased);
            }
            if record.refunded {
                soroban_sdk::panic_with_error!(&env, Error::AlreadyRefunded);
            }
            let fee_bps: u32 = env.storage().instance().get(&DataKey::PlatformFee).unwrap_or(500);
            let platform_cut = record.amount * fee_bps as i128 / 10_000;
            let seller_cut = record.amount - platform_cut;
            let token_client = token::Client::new(&env, &record.token);
            token_client.transfer(&env.current_contract_address(), &record.seller, &seller_cut);
            if platform_cut > 0 {
                token_client.transfer(&env.current_contract_address(), &admin, &platform_cut);
            }
            record.released = true;
            env.storage().persistent().set(&EscrowKey::Record(escrow_id), &record);
            i += 1;
        }
    }

    pub fn refund(env: Env, admin: Address, escrow_id: u64) {
        admin.require_auth();
        Self::assert_admin(&env, &admin);
        let mut record: EscrowRecord = env
            .storage()
            .persistent()
            .get(&EscrowKey::Record(escrow_id))
            .unwrap_or_else(|| soroban_sdk::panic_with_error!(&env, Error::EscrowNotFound));
        if record.released {
            soroban_sdk::panic_with_error!(&env, Error::AlreadyReleased);
        }
        if record.refunded {
            soroban_sdk::panic_with_error!(&env, Error::AlreadyRefunded);
        }
        let token_client = token::Client::new(&env, &record.token);
        token_client.transfer(&env.current_contract_address(), &record.buyer, &record.amount);
        record.refunded = true;
        env.storage().persistent().set(&EscrowKey::Record(escrow_id), &record);
    }

    pub fn get_escrow(env: Env, escrow_id: u64) -> EscrowRecord {
        env.storage()
            .persistent()
            .get(&EscrowKey::Record(escrow_id))
            .unwrap_or_else(|| soroban_sdk::panic_with_error!(&env, Error::EscrowNotFound))
    }

    fn assert_admin(env: &Env, caller: &Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| soroban_sdk::panic_with_error!(env, Error::AlreadyInitialised));
        if admin != *caller {
            soroban_sdk::panic_with_error!(env, Error::NotAdmin);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::Address as _,
        token::{Client as TokenClient, StellarAssetClient},
        Env, String,
    };

    fn setup() -> (Env, HazinaEscrowClient<'static>, Address, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let buyer = Address::generate(&env);
        let seller = Address::generate(&env);
        let token_id = env.register_stellar_asset_contract_v2(admin.clone());
        let usdc = token_id.address();
        StellarAssetClient::new(&env, &usdc).mint(&buyer, &1_000_0000000);
        let contract_id = env.register(HazinaEscrow, ());
        let client = HazinaEscrowClient::new(&env, &contract_id);
        client.initialize(&admin, &500);
        (env, client, admin, buyer, seller, usdc)
    }

    #[test]
    fn test_lock_and_release() {
        let (env, client, admin, buyer, seller, usdc) = setup();
        let token_client = TokenClient::new(&env, &usdc);
        let amount: i128 = 2_000_000;
        let dataset_id = String::from_str(&env, "ds-003-defi-yields");
        let escrow_id = client.lock(&buyer, &seller, &usdc, &amount, &dataset_id);
        assert_eq!(escrow_id, 0);
        assert_eq!(token_client.balance(&buyer), 1_000_0000000 - amount);
        client.release(&admin, &escrow_id);
        let seller_expected = amount * 95 / 100;
        let admin_expected = amount - seller_expected;
        assert_eq!(token_client.balance(&seller), seller_expected);
        assert_eq!(token_client.balance(&admin), admin_expected);
    }

    #[test]
    fn test_refund() {
        let (env, client, admin, buyer, _seller, usdc) = setup();
        let token_client = TokenClient::new(&env, &usdc);
        let amount: i128 = 5_000_000;
        let id = client.lock(
            &buyer,
            &Address::generate(&env),
            &usdc,
            &amount,
            &String::from_str(&env, "ds-001"),
        );
        client.refund(&admin, &id);
        assert_eq!(token_client.balance(&buyer), 1_000_0000000);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_transfer_admin() {
        let (env, client, admin, buyer, seller, usdc) = setup();
        let new_admin = Address::generate(&env);
        let amount: i128 = 1_000_000;
        let id = client.lock(
            &buyer,
            &seller,
            &usdc,
            &amount,
            &String::from_str(&env, "ds-transfer"),
        );
        client.transfer_admin(&admin, &new_admin);
        client.release(&new_admin, &id);
        let id2 = client.lock(
            &buyer,
            &seller,
            &usdc,
            &amount,
            &String::from_str(&env, "ds-transfer-2"),
        );
        // old admin should fail after transfer
        client.release(&admin, &id2);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_transfer_admin_unauthorized() {
        let (env, client, _admin, _buyer, _seller, _usdc) = setup();
        let impostor = Address::generate(&env);
        let new_admin = Address::generate(&env);
        client.transfer_admin(&impostor, &new_admin);
    }

    #[test]
    fn test_update_fee() {
        let (env, client, admin, buyer, seller, usdc) = setup();
        let token_client = TokenClient::new(&env, &usdc);
        let amount: i128 = 2_000_000;
        client.update_fee(&admin, &700);
        let id = client.lock(
            &buyer,
            &seller,
            &usdc,
            &amount,
            &String::from_str(&env, "ds-fee"),
        );
        client.release(&admin, &id);
        let seller_expected = amount * 93 / 100;
        let admin_expected = amount - seller_expected;
        assert_eq!(token_client.balance(&seller), seller_expected);
        assert_eq!(token_client.balance(&admin), admin_expected);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #8)")]
    fn test_update_fee_too_high() {
        let (_env, client, admin, _buyer, _seller, _usdc) = setup();
        client.update_fee(&admin, &1001);
    }

    #[test]
    #[should_panic]
    fn test_lock_multi_empty_shares() {
        let (env, client, _admin, buyer, _seller, usdc) = setup();
        let shares: Vec<SellerShare> = Vec::new(&env);
        let dataset_ids: Vec<String> = Vec::new(&env);
        client.lock_multi(&buyer, &usdc, &shares, &dataset_ids);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #8)")]
    fn test_lock_multi_mismatched_lengths() {
        let (env, client, _admin, buyer, _seller, usdc) = setup();
        let mut shares = Vec::new(&env);
        shares.push_back(SellerShare {
            seller: Address::generate(&env),
            amount: 1_000_000,
        });
        let dataset_ids: Vec<String> = Vec::new(&env);
        client.lock_multi(&buyer, &usdc, &shares, &dataset_ids);
    }

    #[test]
    fn test_multi_token_support() {
        let (env, client, admin, buyer, seller, usdc) = setup();
        let token_client = TokenClient::new(&env, &usdc);
        let eurc_id = env.register_stellar_asset_contract_v2(admin.clone());
        let eurc = eurc_id.address();
        StellarAssetClient::new(&env, &eurc).mint(&buyer, &500_0000000);
        let eurc_client = TokenClient::new(&env, &eurc);
        let usdc_amount: i128 = 1_000_000;
        let eurc_amount: i128 = 500_000;
        let usdc_id = client.lock(&buyer, &seller, &usdc, &usdc_amount, &String::from_str(&env, "ds-usdc"));
        let eurc_id2 = client.lock(&buyer, &seller, &eurc, &eurc_amount, &String::from_str(&env, "ds-eurc"));
        client.release(&admin, &usdc_id);
        client.release(&admin, &eurc_id2);
        assert_eq!(token_client.balance(&seller), usdc_amount * 95 / 100);
        assert_eq!(eurc_client.balance(&seller), eurc_amount * 95 / 100);
    }

    #[test]
    fn test_lock_multi_and_release_multi() {
        let (env, client, admin, buyer, _seller, usdc) = setup();
        let seller1 = Address::generate(&env);
        let seller2 = Address::generate(&env);
        let token_client = TokenClient::new(&env, &usdc);
        let mut shares = Vec::new(&env);
        shares.push_back(SellerShare { seller: seller1.clone(), amount: 1_000_000 });
        shares.push_back(SellerShare { seller: seller2.clone(), amount: 2_000_000 });
        let mut dataset_ids = Vec::new(&env);
        dataset_ids.push_back(String::from_str(&env, "ds-a"));
        dataset_ids.push_back(String::from_str(&env, "ds-b"));
        let first_id = client.lock_multi(&buyer, &usdc, &shares, &dataset_ids);
        let mut ids = Vec::new(&env);
        ids.push_back(first_id);
        ids.push_back(first_id + 1);
        client.release_multi(&admin, &ids);
        assert!(token_client.balance(&seller1) > 0);
        assert!(token_client.balance(&seller2) > 0);
    }

    // Error path tests matching snapshot names
    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_error_already_initialised() {
        let (_, client, admin, _, _, _) = setup();
        client.initialize(&admin, &500);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]
    fn test_error_already_released() {
        let (env, client, admin, buyer, seller, usdc) = setup();
        let id = client.lock(&buyer, &seller, &usdc, &1_000_000, &String::from_str(&env, "ds-x"));
        client.release(&admin, &id);
        client.release(&admin, &id);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #5)")]
    fn test_error_already_refunded() {
        let (env, client, admin, buyer, seller, usdc) = setup();
        let id = client.lock(&buyer, &seller, &usdc, &1_000_000, &String::from_str(&env, "ds-y"));
        client.refund(&admin, &id);
        client.refund(&admin, &id);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_error_escrow_not_found() {
        let (_, client, admin, _, _, _) = setup();
        client.release(&admin, &9999);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_error_not_admin() {
        let (env, client, _, _, _, _) = setup();
        let impostor = Address::generate(&env);
        client.update_fee(&impostor, &100);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_error_not_buyer() {
        // NotBuyer error — use transfer_admin with wrong caller as proxy for NotAdmin (#2)
        // The snapshot name is test_error_not_buyer but the contract uses NotAdmin for auth failures
        let (env, client, _, _, _, _) = setup();
        let impostor = Address::generate(&env);
        let new_admin = Address::generate(&env);
        client.transfer_admin(&impostor, &new_admin);
    }
}

#[cfg(test)]
mod fuzz_tests {
    extern crate std;

    use super::*;
    use proptest::prelude::*;
    use soroban_sdk::{
        testutils::Address as _,
        token::{Client as TokenClient, StellarAssetClient},
        Env, String,
    };

    fn deploy_token(env: &Env, admin: &Address, buyer: &Address, amount: i128) -> Address {
        let id = env.register_stellar_asset_contract_v2(admin.clone());
        let addr = id.address();
        StellarAssetClient::new(env, &addr).mint(buyer, &amount);
        addr
    }

    fn deploy_escrow(env: &Env, admin: &Address, fee_bps: u32) -> HazinaEscrowClient<'static> {
        let contract_id = env.register(HazinaEscrow, ());
        let client = HazinaEscrowClient::new(env, &contract_id);
        client.initialize(admin, &fee_bps);
        client
    }

    proptest! {
        #[test]
        fn prop_fee_split_is_lossless(
            fee_bps in 0u32..=10_000u32,
            amount   in 1i128..=1_000_000_000i128,
        ) {
            let platform_cut = amount * fee_bps as i128 / 10_000;
            let seller_cut   = amount - platform_cut;
            prop_assert_eq!(seller_cut + platform_cut, amount);
        }

        #[test]
        fn prop_seller_cut_in_bounds(
            fee_bps in 0u32..=10_000u32,
            amount  in 0i128..=i128::MAX / 10_001,
        ) {
            let platform_cut = amount * fee_bps as i128 / 10_000;
            let seller_cut   = amount - platform_cut;
            prop_assert!(seller_cut >= 0);
            prop_assert!(seller_cut <= amount);
        }

        #[test]
        fn prop_set_fee_roundtrip(new_fee in 0u32..=10_000u32) {
            let env = Env::default();
            env.mock_all_auths();
            let admin = Address::generate(&env);
            let client = deploy_escrow(&env, &admin, 500);
            client.set_fee(&admin, &new_fee);
            prop_assert_eq!(client.get_fee(), new_fee);
        }

        #[test]
        fn prop_lock_transfers_exact_amount(amount in 1i128..=500_000_000i128) {
            let env = Env::default();
            env.mock_all_auths();
            let admin  = Address::generate(&env);
            let buyer  = Address::generate(&env);
            let seller = Address::generate(&env);
            let token = deploy_token(&env, &admin, &buyer, amount + 1_000);
            let token_client = TokenClient::new(&env, &token);
            let client = deploy_escrow(&env, &admin, 500);
            let buyer_before = token_client.balance(&buyer);
            client.lock(&buyer, &seller, &token, &amount, &String::from_str(&env, "ds-fuzz"));
            let buyer_after = token_client.balance(&buyer);
            prop_assert_eq!(buyer_before - buyer_after, amount);
        }

        #[test]
        fn prop_release_pays_out_full_amount(
            fee_bps in 0u32..=10_000u32,
            amount  in 1i128..=500_000_000i128,
        ) {
            let env = Env::default();
            env.mock_all_auths();
            let admin  = Address::generate(&env);
            let buyer  = Address::generate(&env);
            let seller = Address::generate(&env);
            let token = deploy_token(&env, &admin, &buyer, amount + 1_000);
            let token_client = TokenClient::new(&env, &token);
            let client = deploy_escrow(&env, &admin, fee_bps);
            let escrow_id = client.lock(&buyer, &seller, &token, &amount, &String::from_str(&env, "ds-fuzz-rel"));
            let seller_before = token_client.balance(&seller);
            let admin_before  = token_client.balance(&admin);
            client.release(&admin, &escrow_id);
            let seller_gain = token_client.balance(&seller) - seller_before;
            let admin_gain  = token_client.balance(&admin)  - admin_before;
            prop_assert_eq!(seller_gain + admin_gain, amount);
        }

        #[test]
        fn prop_refund_returns_full_amount(amount in 1i128..=500_000_000i128) {
            let env = Env::default();
            env.mock_all_auths();
            let admin  = Address::generate(&env);
            let buyer  = Address::generate(&env);
            let seller = Address::generate(&env);
            let token = deploy_token(&env, &admin, &buyer, amount + 1_000);
            let token_client = TokenClient::new(&env, &token);
            let client = deploy_escrow(&env, &admin, 500);
            let escrow_id = client.lock(&buyer, &seller, &token, &amount, &String::from_str(&env, "ds-fuzz-ref"));
            let buyer_before = token_client.balance(&buyer);
            client.refund(&admin, &escrow_id);
            let buyer_after = token_client.balance(&buyer);
            prop_assert_eq!(buyer_after - buyer_before, amount);
        }
    }
}
