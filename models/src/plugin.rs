use crate::Money;
use futures::stream::BoxStream;

type AccountId = [u8; 32];

trait Plugin {
    type TransferStatus: Status;

    fn quote(amount: Money, to: AccountId) -> Quote;
    fn accounts() -> BoxStream<'static, Account>;
    fn transfer(
        amount: Money,
        to: AccountId,
        quote: Quote,
    ) -> BoxStream<'static, Self::TransferStatus>;
}

trait Status {
    fn is_completed() -> bool;
}

struct Quote {
    rate: f32,
}

struct Account {
    id: AccountId,
    balance: Money,
}
