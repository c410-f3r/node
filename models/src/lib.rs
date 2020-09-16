use std::collections::HashMap;
use std::fmt;
use std::iter::Iterator;

pub struct Field {
    name: String,
    validator: Box<dyn Fn(&str) -> bool>,
}
impl Field {
    fn new(name: &str, func: Box<dyn Fn(&str) -> bool>) -> Self {
        Field {
            name: name.to_string(),
            validator: func,
        }
    }
}

pub trait Medium: fmt::Display {
    fn asset(&self) -> Asset;

    fn payment_details(&self) -> Vec<Field>;

    fn name(&self) -> String {
        self.to_string()
    }

    fn asset_name(&self) -> String {
        self.asset().name()
    }
}

type Amount = f32;

#[derive(Debug, Clone, PartialEq)]
pub struct Money(Asset, Amount);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Asset {
    Fiat(Fiat),
    Digital(Digital),
}
impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Asset::Fiat(a) => a.to_string(),
            Asset::Digital(a) => a.to_string(),
        };
        write!(f, "{}", name)
    }
}

impl Asset {
    pub fn name(&self) -> String {
        self.to_string()
    }
}
impl From<Digital> for Asset {
    fn from(x: Digital) -> Self {
        Asset::Digital(x)
    }
}
impl From<Fiat> for Asset {
    fn from(x: Fiat) -> Self {
        Asset::Fiat(x)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Fiat {
    COP,
    VES,
}

impl fmt::Display for Fiat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fiat::COP => write!(f, "COP"),
            Fiat::VES => write!(f, "VES"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Digital {
    BTC,
    USDv,
}

impl fmt::Display for Digital {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Digital::BTC => write!(f, "BTC"),
            Digital::USDv => write!(f, "USDv"),
        }
    }
}

//struct Asset {
//    id: Uuid,
//    name: String,
//    medium: ,
//
//}
pub struct AccountId;

pub struct DebtBalance {
    borrower: AccountId,
    lender: AccountId,
    amount: Money,
}

///
pub struct Debt {
    borrower: AccountId,
    lender: AccountId,
    amount_owed: Money,
    payment_assets: Vec<Asset>,
    status: DebtStatus,
    payments: Vec<PaymentFraction>,
    fees: Vec<Fee>,
}

impl Debt {
    fn new() -> Self {
        Debt {
            borrower: AccountId,
            lender: AccountId,
            amount_owed: Money(Fiat::COP.into(), 0.0),
            payment_assets: vec![],
            status: DebtStatus::Approved,
            payments: vec![],
            fees: vec![],
        }
    }

    fn status(&self) -> DebtStatus {
        self.status.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DebtStatus {
    Approved,
    Active { payed_so_far: Money },
    Paid,
}

struct PaymentFraction {
    amount: Amount,
    payment_id: PaymentId,
}

struct PaymentId;

struct Fee {
    name: String,
    amount: Amount,
}

///
struct Payment<T: Medium> {
    payer: AccountId,
    recipient: AccountId,
    paid_debt: Money,
    execution_amount: Money,
    status: PaymentStatus,
    medium: T,
    payment_details: HashMap<String, String>,
}

impl<T: Medium> Payment<T> {
    fn payment_details(&self) -> impl Iterator<Item = (&String, &String)> {
        self.payment_details.iter()
    }
}

enum PaymentStatus {
    Created,
    InProgress,
    Completed,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Bancolombia {
        asset: Asset,
    }

    impl Bancolombia {
        fn new() -> Self {
            Bancolombia {
                asset: Asset::Fiat(Fiat::COP),
            }
        }
    }

    impl fmt::Display for Bancolombia {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Bancolombia({})", self.asset_name())
        }
    }

    impl Medium for Bancolombia {
        fn asset(&self) -> Asset {
            self.asset
        }

        fn payment_details(&self) -> Vec<Field> {
            vec![]
        }
    }

    #[test]
    fn medium_name() {
        let bancolombia = Bancolombia::new();

        assert_eq!(bancolombia.name(), "Bancolombia(COP)");
    }

    #[test]
    fn debt() {
        let debt = Debt::new();
        assert_eq!(debt.status(), DebtStatus::Approved);
    }
}
