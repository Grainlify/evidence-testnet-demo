#![no_std]
//! Guestbook: a tiny Soroban contract used as a test project for Grainlify's evidence engine.
//! Anyone can sign (with their own auth); each signature emits a `signed` event.
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, Address, BytesN, Env, Symbol};

#[contracttype]
enum Key {
    Admin,
    Count,
    Signer(Address),
    Last,
    Unique,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[contract]
pub struct Guestbook;

#[contractimpl]
impl Guestbook {
    /// Admin-only setup, called once by the deployer.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&Key::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&Key::Admin, &admin);
        env.storage().instance().set(&Key::Count, &0u32);
        Ok(())
    }

    /// Signs the guestbook with a short message. Returns the total number of signatures.
    pub fn sign(env: Env, from: Address, msg: Symbol) -> Result<u32, Error> {
        if !env.storage().instance().has(&Key::Admin) {
            return Err(Error::NotInitialized);
        }
        from.require_auth();
        let count: u32 = env.storage().instance().get(&Key::Count).unwrap_or(0) + 1;
        env.storage().instance().set(&Key::Count, &count);
        let mine: u32 = env.storage().persistent().get(&Key::Signer(from.clone())).unwrap_or(0) + 1;
        if mine == 1 {
            let unique: u32 = env.storage().instance().get(&Key::Unique).unwrap_or(0) + 1;
            env.storage().instance().set(&Key::Unique, &unique);
        }
        env.storage().persistent().set(&Key::Signer(from.clone()), &mine);
        env.storage().instance().set(&Key::Last, &(from.clone(), msg.clone()));
        env.events().publish((symbol_short!("signed"), from), msg);
        Ok(count)
    }

    /// How many different accounts have signed.
    pub fn unique_signers(env: Env) -> u32 {
        env.storage().instance().get(&Key::Unique).unwrap_or(0)
    }

    /// The most recent signer and message, if any.
    pub fn last_message(env: Env) -> Option<(Address, Symbol)> {
        env.storage().instance().get(&Key::Last)
    }

    /// Admin-only: replace the contract code, keeping its storage.
    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), Error> {
        let admin: Address = env.storage().instance().get(&Key::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }

    pub fn count(env: Env) -> u32 {
        env.storage().instance().get(&Key::Count).unwrap_or(0)
    }

    pub fn signatures_of(env: Env, who: Address) -> u32 {
        env.storage().persistent().get(&Key::Signer(who)).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
