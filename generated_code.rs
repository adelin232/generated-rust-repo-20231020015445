#![no_std]

imports!();

pub struct MultiversXToken {
    pub storage: Box<dyn StorageMapper>,
    pub token_id: TokenIdentifier
}

impl MultiversXToken {
    pub const TOKEN_INIT_FAILED: &'static str = "Could not mint the required token";

    #[init]
    pub fn init(
        &self,
        amount: &BigUint,
        token_id: TokenIdentifier,
        alice_address: Address,
        bob_address: Address,
    ) {
        let _ = self.storage.mint(amount, &token_id, alice_address).map_err(|err| {
            sc_error!(Self::TOKEN_INIT_FAILED);
        });
        let _ = self
            .storage
            .transfer(&token_id, alice_address, bob_address, &amount)
            .map_err(|err| {
                sc_error!(Self::TOKEN_INIT_FAILED);
            });
    }

    #[endpoint]
    pub fn mint_and_send(&self, amount: &BigUint, token_id: TokenIdentifier, from: Address, to: Address) {
        let _ = self.storage.mint(amount, &token_id, from).map_err(|err| {
            sc_error!("Could not mint the required token");
        });

        let _ = self
            .storage
            .transfer(&token_id, from, to, &amount)
            .map_err(|err| {
                sc_error!("Could not send token");
            });
    }
}

multiversx_sc! { MultiversXToken }