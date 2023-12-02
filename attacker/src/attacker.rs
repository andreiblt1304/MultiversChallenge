#![no_std]

multiversx_sc::imports!();

use crate::lottery_proxy;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Attacker {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint(participate)]
    fn participate(&self) {

        //self.call
    }
}
