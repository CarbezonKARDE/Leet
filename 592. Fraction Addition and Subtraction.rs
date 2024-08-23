struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Default for Fraction {
    fn default() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }
}

impl std::ops::Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.numerator == 0 {
            return other;
        }

        let mut numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let mut denominator = if numerator == 0 {
            1
        } else {
            self.denominator * other.denominator
        };

        for i in 2..(denominator).min(numerator.abs()) {
            while denominator % i == 0 && numerator % i == 0 {
                denominator /= i;
                numerator /= i;
            }
        }

        Self {
            numerator,
            denominator,
        }
    }
}

impl Solution {
    pub fn fraction_addition(exp: String) -> String {
        let indices: Vec<usize> = if exp.as_bytes()[0] == b'-' {
            exp.match_indices(|c| c == '+' || c == '-')
                .map(|(i, _)| i)
                .chain([exp.len()])
                .collect()
        } else {
            std::iter::once(0)
                .chain(exp.match_indices(|c| c == '+' || c == '-').map(|(i, _)| i))
                .chain([exp.len()])
                .collect()
        };

        let fra = indices
            .windows(2)
            .map(|v| Self::parse_fraction(&exp[v[0]..v[1]]))
            .fold(Fraction::default(), |acc, fra| acc + fra);

        format!("{}/{}", fra.numerator, fra.denominator)
    }

    fn parse_fraction(exp: &str) -> Fraction {
        let (num, den) = exp.split_once('/').unwrap();
        Fraction {
            numerator: num.parse().unwrap(),
            denominator: den.parse().unwrap(),
        }
    }
}
