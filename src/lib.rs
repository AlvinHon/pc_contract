use pchain_sdk::{contract_methods, contract, action, transaction, emit_event, pay};
use pchain_types::PublicAddress;


#[contract]
struct TokenTransfer{}

#[contract_methods(meta)]
impl TokenTransfer {
    #[action]
    fn e(address: PublicAddress, value: u64) -> u64 {
        assert_eq!(transaction::value(), value);
        emit_event(
            format!("{:?}", address).as_bytes(),
            format!("{}", value).as_bytes()
        );
        pay(address, value)
    }
}