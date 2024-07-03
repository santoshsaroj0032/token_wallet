use super::contract::*;
use ic_cdk::export::candid::Principal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet() {
        create_wallet("test_wallet".into());
        let balance = get_balance("test_wallet".into());
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_send_tokens() {
        create_wallet("wallet1".into());
        create_wallet("wallet2".into());
        
        // Add initial balance
        send_tokens("wallet1".into(), "wallet1".into(), 100).unwrap();

        // Transfer tokens
        send_tokens("wallet1".into(), "wallet2".into(), 50).unwrap();
        let balance1 = get_balance("wallet1".into());
        let balance2 = get_balance("wallet2".into());

        assert_eq!(balance1, 50);
        assert_eq!(balance2, 50);
    }
}
