#![no_std]

use multiversx_sc::imports::*;

mod admins;
mod pause;

#[multiversx_sc::contract]
pub trait Template: admins::AdminsModule + pause::PauseModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
