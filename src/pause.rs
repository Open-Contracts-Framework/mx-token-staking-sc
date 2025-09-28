use errors::{ERROR_NOT_PAUSED, ERROR_PAUSED};
use multiversx_sc::imports::*;

/// Smart Contract module that offers pausability.
///
/// It provides:
/// * two endpoints where an admin can pause/unpause the contract
/// * a view to check if the contract is paused
/// * two methods to require the contract to be paused/not paused
#[multiversx_sc::module]
pub trait PauseModule: crate::admins::AdminsModule {
    // === Endpoints ===

    #[endpoint(pause)]
    fn pause(&self) {
        self.require_is_admin(&self.blockchain().get_caller());
        self.require_not_paused();

        self.is_paused().set(true);
        self.event_paused();
    }

    #[endpoint(unpause)]
    fn unpause(&self) {
        self.require_is_admin(&self.blockchain().get_caller());
        self.require_paused();

        self.is_paused().set(false);
        self.event_unpaused();
    }

    // === Private ===

    fn require_paused(&self) {
        require!(self.is_paused().get(), ERROR_NOT_PAUSED);
    }

    fn require_not_paused(&self) {
        require!(!self.is_paused().get(), ERROR_PAUSED);
    }

    // === Storage ===

    #[view(isPaused)]
    #[storage_mapper("is_paused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;

    // === Events ===

    #[event("paused")]
    fn event_paused(&self);

    #[event("unpaused")]
    fn event_unpaused(&self);
}
