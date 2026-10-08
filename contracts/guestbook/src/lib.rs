#![no_std]
//! Guestbook: a tiny Soroban contract used as a test project for Grainlify's evidence engine.
//! Anyone can sign (with their own auth); each signature emits a `signed` event.
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

#[contracttype]
enum Key {
    Admin,
    Count,
    Signer(Address),
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
        env.storage().persistent().set(&Key::Signer(from.clone()), &mine);
        env.events().publish((symbol_short!("signed"), from), msg);
        Ok(count)
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
