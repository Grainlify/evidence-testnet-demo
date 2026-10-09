use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn signs_and_counts() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Guestbook, ());
    let c = GuestbookClient::new(&env, &id);
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    c.init(&admin);
    assert_eq!(c.sign(&alice, &symbol_short!("hello")), 1);
    assert_eq!(c.sign(&alice, &symbol_short!("again")), 2);
    assert_eq!(c.count(), 2);
    assert_eq!(c.signatures_of(&alice), 2);
}

#[test]
fn init_only_once() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Guestbook, ());
    let c = GuestbookClient::new(&env, &id);
    let admin = Address::generate(&env);
    c.init(&admin);
    assert_eq!(c.try_init(&admin), Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn sign_needs_init() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Guestbook, ());
    let c = GuestbookClient::new(&env, &id);
    let alice = Address::generate(&env);
    assert_eq!(c.try_sign(&alice, &symbol_short!("hi")), Err(Ok(Error::NotInitialized)));
}

#[test]
fn remembers_the_last_message() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Guestbook, ());
    let c = GuestbookClient::new(&env, &id);
    let admin = Address::generate(&env);
    let bob = Address::generate(&env);
    c.init(&admin);
    assert_eq!(c.last_message(), None);
    c.sign(&bob, &symbol_short!("first"));
    c.sign(&bob, &symbol_short!("second"));
    assert_eq!(c.last_message(), Some((bob, symbol_short!("second"))));
}
