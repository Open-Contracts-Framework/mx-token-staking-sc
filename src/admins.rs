use errors::ERROR_NOT_ADMIN;
use multiversx_sc::imports::*;

/// Smart Contract module that offers admin management capabilities.
///
/// It provides:
/// * two endpoints where the owner can add/remove admins
/// * a view to check if an address is an admin
/// * a view to get the list of admins
/// * a method to require an address to be an admin
#[multiversx_sc::module]
pub trait AdminsModule {
    // === Endpoints ===

    #[only_owner]
    #[endpoint(addAdmins)]
    fn add_admin(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        self.event_admins_added(&addresses);

        for address in addresses.into_iter() {
            self.admins().insert(address);
        }
    }

    #[only_owner]
    #[endpoint(removeAdmins)]
    fn remove_admin(&self, addresses: MultiValueEncoded<ManagedAddress>) {
        self.event_admins_removed(&addresses);

        for address in addresses.into_iter() {
            self.admins().swap_remove(&address);
        }
    }

    // === Views ===

    #[view(isAdmin)]
    fn is_admin(&self, address: &ManagedAddress) -> bool {
        self.admins().contains(address)
    }

    // === Private ===

    fn require_is_admin(&self, address: &ManagedAddress) {
        require!(self.is_admin(address), ERROR_NOT_ADMIN);
    }

    // === Storage ===

    #[view(getAdmins)]
    #[storage_mapper("admins")]
    fn admins(&self) -> UnorderedSetMapper<ManagedAddress>;

    // === Events ===

    #[event("adminsAdded")]
    fn event_admins_added(&self, #[indexed] admins: &MultiValueEncoded<ManagedAddress>);

    #[event("adminsRemoved")]
    fn event_admins_removed(&self, #[indexed] admins: &MultiValueEncoded<ManagedAddress>);
}
