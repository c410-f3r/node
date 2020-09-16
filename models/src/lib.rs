//use std::collections::HashMap;
use std::fmt;

type PaymentDetail = (String, Option<i32>);

trait Medium: fmt::Display {
    fn asset(&self) -> Asset;

    fn payment_details(&self) -> Vec<PaymentDetail>;

    fn name(&self) -> String {
        self.to_string()
    }

    fn asset_name(&self) -> String {
        self.asset().name()
    }
}

type Amount = f32;

struct Money<T: Medium>(T, Amount);

struct Actor;

struct DebtBalance<T: Medium> {
    borrower: Actor,
    lender: Actor,
    amount: Money<T>,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

        fn payment_details(&self) -> Vec<PaymentDetail> {
            vec![]
        }
    }

    #[test]
    fn medium_name() {
        let bancolombia = Bancolombia::new();

        assert_eq!(bancolombia.name(), "Bancolombia(COP)");
    }
}
