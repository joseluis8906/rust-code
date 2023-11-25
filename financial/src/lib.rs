use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Currency {
    UNKNOWN,
    USD,
    CHF,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Currency::UNKNOWN => write!(f, "UNKNOWN"),
            Currency::USD => write!(f, "USD"),
            Currency::CHF => write!(f, "CHF"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Money {
    amount: i32,
    currency: Currency,
}

impl Money {
    pub fn new(amount: i32, currency: Currency) -> Money {
        match currency {
            Currency::UNKNOWN => Money {
                amount: 0,
                currency,
            },
            _ => Money { amount, currency },
        }
    }

    pub fn default() -> Money {
        Money {
            amount: 0,
            currency: Currency::UNKNOWN,
        }
    }

    pub fn dollar(amount: i32) -> Money {
        Money::new(amount, Currency::USD)
    }

    pub fn franc(amount: i32) -> Money {
        Money::new(amount, Currency::CHF)
    }

    pub fn times(&self, multiplier: i32) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency,
        }
    }

    pub fn plus(&self, augend: &Money) -> Money {
        if self.currency != augend.currency {
            Money::default();
        }

        Money::new(self.amount + augend.amount, self.currency.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn equality() {
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(6));
        assert_eq!(Money::franc(5), Money::franc(5));
    }

    #[test]
    fn plus() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.plus(&Money::dollar(5)));
    }

    #[test]
    fn valid_currency() {
        assert_eq!(Money::new(10, Currency::UNKNOWN), Money::default());
        assert_eq!(Money::dollar(10), Money::new(10, Currency::USD));
    }
}
