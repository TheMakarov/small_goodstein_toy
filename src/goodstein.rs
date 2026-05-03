#[derive(Debug)]
pub struct GNumber {
    pub constituents: Vec<Constituent>,
    pub sum_value: u64,
}

#[derive(Debug, Clone)]
pub struct Constituent {
    pub base: u64,
    pub exponent: ConstituentExp,
}

// this is temporary, this is for building the graph
#[derive(Debug, Clone)]
pub enum ConstituentExp {
    Plain(u64),
    Constituent(Vec<Constituent>),
}

impl GNumber {
    pub fn new(number: u64) -> Self {
        Self {
            constituents: Self::build_constituent(number, 2),
            sum_value: number,
        }
    }

    fn helper2(constituent: ConstituentExp) -> ConstituentExp {
        match constituent {
            ConstituentExp::Plain(value) => ConstituentExp::Plain(value),
            ConstituentExp::Constituent(mut constituents) => {
                constituents.iter_mut().for_each(|cons| {
                    cons.base = cons.base + 1;
                    cons.exponent = Self::helper2(cons.exponent.clone());
                });
                ConstituentExp::Constituent(constituents)
            }
        }
    }

    pub fn do_goodstein(&mut self) {
        self.constituents
            .iter_mut()
            .for_each(|cons: &mut Constituent| {
                cons.base = cons.base + 1;
                cons.exponent = Self::helper2(cons.exponent.clone());
            });
    }

    fn helper1(number: u64, base: u64, step: u64, acc: &mut Vec<Constituent>) {
        if number == 0 {
            return;
        }

        let remainder = number % base;

        if remainder != 0 {
            let exponent = if step < base {
                ConstituentExp::Plain(step)
            } else {
                ConstituentExp::Constituent(Self::build_constituent(step, base))
            };

            acc.push(Constituent { base, exponent });
        }

        Self::helper1(number / base, base, step + 1, acc);
    }

    fn helper3(constituent: ConstituentExp) -> u64 {
        match constituent {
            ConstituentExp::Plain(value) => value,
            ConstituentExp::Constituent(constituents) => {
                constituents.iter().fold(0u64, |acc, cons| {
                    acc + cons
                        .base
                        .pow(Self::helper3(cons.exponent.clone()).try_into().unwrap())
                })
            }
        }
    }

    pub fn sum_over_power_factors(&mut self) {
        let result = self.constituents.iter().fold(0u64, |acc, cons| {
            acc + cons
                .base
                .pow(Self::helper3(cons.exponent.clone()).try_into().unwrap())
        });

        self.sum_value = result;
    }

    pub fn substract_one_and_restart(&mut self) {
        self.constituents = Self::build_constituent(self.sum_value - 1, self.constituents[0].base);
    }

    fn build_constituent(number: u64, base: u64) -> Vec<Constituent> {
        let mut result = vec![];
        Self::helper1(number, base, 0, &mut result);
        result.reverse();
        result
    }
}
