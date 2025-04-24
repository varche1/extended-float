use std::cmp::Eq;

use super::ExtendedFloat;
use crate::traits::{DisplayableFloat, Float};

impl<T: DisplayableFloat> PartialEq for ExtendedFloat<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.downgrade() == other.downgrade() {
            return true;
        }

        if self.downgrade().is_nan() || other.downgrade().is_nan() {
            return false;
        }

        let abs_diff = (self.downgrade() - other.downgrade()).abs();
        let epsilon = <T as Float>::epsilon();

        if abs_diff <= epsilon {
            return true;
        }

        abs_diff < epsilon * self.downgrade().abs().min(other.downgrade().abs())
    }
}

impl<T: DisplayableFloat> Eq for ExtendedFloat<T> {}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::constants::f64::EPSILON as EPSILON_F64;
    use crate::types::ExtendedFloat;

    const ITERATIONS: i32 = 10_000_000;

    fn calculate_f64(start: f64, decrement: f64, count: i32) -> ExtendedFloat<f64> {
        let mut start = start;
        for _ in 0..count {
            start -= decrement;
        }
        ExtendedFloat::new(start)
    }

    #[test]
    fn test_equal_f64() {
        let total = ITERATIONS;
        let mut count = 0;
        for i in 0..total {
            let expected = (i as f64) / 10.0;
            let actual = calculate_f64(9.0 + expected, 0.2, 45);
            if actual == ExtendedFloat::new(expected) {
                count += 1;
            }
        }
        assert_eq!(count, total);
    }

    #[test]
    fn test_equal_table_f64() {
        struct TestCase {
            a: f64,
            b: f64,
            equal: bool,
        }

        let cases = vec![
            TestCase {
                a: 0.0000000000001,
                b: 0.0000000000001,
                equal: true,
            },
            TestCase {
                a: 0.000000000001,
                b: 0.000000000001,
                equal: true,
            },
            TestCase {
                a: 0.00000000001,
                b: 0.00000000001,
                equal: true,
            },
            TestCase {
                a: 0.0000000001,
                b: 0.0000000001,
                equal: true,
            },
            TestCase {
                a: 0.000000001,
                b: 0.000000001,
                equal: true,
            },
            TestCase {
                a: 0.00000001,
                b: 0.00000001,
                equal: true,
            },
            TestCase {
                a: 0.0000001,
                b: 0.0000001,
                equal: true,
            },
            TestCase {
                a: 0.000001,
                b: 0.000001,
                equal: true,
            },
            TestCase {
                a: 0.00001,
                b: 0.00001,
                equal: true,
            },
            TestCase {
                a: 0.0001,
                b: 0.0001,
                equal: true,
            },
            TestCase {
                a: 0.001,
                b: 0.001,
                equal: true,
            },
            TestCase {
                a: 0.1,
                b: 0.1,
                equal: true,
            },
            TestCase {
                a: 0.0,
                b: 0.0,
                equal: true,
            },
            TestCase {
                a: 10.0,
                b: 10.0,
                equal: true,
            },
            TestCase {
                a: 100.0,
                b: 100.0,
                equal: true,
            },
            TestCase {
                a: 1_000.0,
                b: 1_000.0,
                equal: true,
            },
            TestCase {
                a: 10_000.0,
                b: 10_000.0,
                equal: true,
            },
            TestCase {
                a: 100_000.0,
                b: 100_000.0,
                equal: true,
            },
            TestCase {
                a: 1_000_000.0,
                b: 1_000_000.0,
                equal: true,
            },
            TestCase {
                a: 10_000_000.0,
                b: 10_000_000.0,
                equal: true,
            },
            TestCase {
                a: 100_000_000.0,
                b: 100_000_000.0,
                equal: true,
            },
            TestCase {
                a: 1_000_000_000.0,
                b: 1_000_000_000.0,
                equal: true,
            },
            TestCase {
                a: 10_000_000_000.0,
                b: 10_000_000_000.0,
                equal: true,
            },
            TestCase {
                a: 100_000_000_000.0,
                b: 100_000_000_000.0,
                equal: true,
            },
            TestCase {
                a: 1_000_000_000_000.0,
                b: 1_000_000_000_000.0,
                equal: true,
            },
            TestCase {
                a: 0.0000000000001,
                b: 0.0000000000002,
                equal: true,
            }, // тут уже считаем, что равны
            TestCase {
                a: 0.000000000001,
                b: 0.000000000002,
                equal: true,
            }, // тут уже считаем, что равны
            TestCase {
                a: 0.00000000001,
                b: 0.00000000002,
                equal: false,
            },
            TestCase {
                a: 0.0000000001,
                b: 0.0000000002,
                equal: false,
            },
            TestCase {
                a: 0.000000001,
                b: 0.000000002,
                equal: false,
            },
            TestCase {
                a: 0.00000001,
                b: 0.00000002,
                equal: false,
            },
            TestCase {
                a: 0.0000001,
                b: 0.0000002,
                equal: false,
            },
            TestCase {
                a: 0.000001,
                b: 0.000002,
                equal: false,
            },
            TestCase {
                a: 0.00001,
                b: 0.00002,
                equal: false,
            },
            TestCase {
                a: 0.0001,
                b: 0.0002,
                equal: false,
            },
            TestCase {
                a: 0.001,
                b: 0.002,
                equal: false,
            },
            TestCase {
                a: 0.1,
                b: 0.2,
                equal: false,
            },
            TestCase {
                a: 0.0,
                b: 1.0,
                equal: false,
            },
            TestCase {
                a: 10.0,
                b: 11.0,
                equal: false,
            },
            TestCase {
                a: 100.0,
                b: 101.0,
                equal: false,
            },
            TestCase {
                a: 1_000.0,
                b: 1_001.0,
                equal: false,
            },
            TestCase {
                a: 10_000.0,
                b: 10_001.0,
                equal: false,
            },
            TestCase {
                a: 100_000.0,
                b: 100_001.0,
                equal: false,
            },
            TestCase {
                a: 1_000_000.0,
                b: 1_000_001.0,
                equal: false,
            },
            TestCase {
                a: 10_000_000.0,
                b: 10_000_001.0,
                equal: false,
            },
            TestCase {
                a: 100_000_000.0,
                b: 100_000_001.0,
                equal: false,
            },
            TestCase {
                a: 1_000_000_000.0,
                b: 1_000_000_001.0,
                equal: false,
            },
            TestCase {
                a: 10_000_000_000.0,
                b: 10_000_000_001.0,
                equal: false,
            },
            TestCase {
                a: 100_000_000_000.0,
                b: 100_000_000_001.0,
                equal: false,
            },
            TestCase {
                a: 1_000_000_000_000.0,
                b: 1_000_000_000_001.0,
                equal: false,
            },
            TestCase {
                a: 10_000_000_000_000.0,
                b: 10_000_000_000_001.0,
                equal: true,
            },
            TestCase {
                a: 100_000_000_000_000.0,
                b: 100_000_000_000_001.0,
                equal: true,
            },
        ];

        for case in cases {
            assert_eq!(
                ExtendedFloat::new(case.a) == ExtendedFloat::new(case.b),
                case.equal
            );
        }
    }

    #[test]
    fn test_zero_equality() {
        // Test zero equality
        assert_eq!(ExtendedFloat::new(0.0), ExtendedFloat::new(0.0));
        assert_eq!(ExtendedFloat::new(0.0), ExtendedFloat::new(-0.0));

        // Numbers very close to zero should be equal to zero
        assert_eq!(
            ExtendedFloat::new(0.0),
            ExtendedFloat::new(EPSILON_F64 * 0.5)
        );
        assert_eq!(
            ExtendedFloat::new(0.0),
            ExtendedFloat::new(-EPSILON_F64 * 0.5)
        );

        // Numbers just at epsilon boundary
        assert_eq!(ExtendedFloat::new(0.0), ExtendedFloat::new(EPSILON_F64));
        assert_eq!(ExtendedFloat::new(0.0), ExtendedFloat::new(-EPSILON_F64));

        // Numbers beyond epsilon boundary
        assert_ne!(
            ExtendedFloat::new(0.0),
            ExtendedFloat::new(EPSILON_F64 * 10.0)
        );
        assert_ne!(
            ExtendedFloat::new(0.0),
            ExtendedFloat::new(-EPSILON_F64 * 10.0)
        );
    }

    #[test]
    fn test_special_values_f64() {
        // NaN should never equal anything, including itself
        assert_ne!(unsafe { ExtendedFloat::new_unchecked(f64::NAN) }, unsafe {
            ExtendedFloat::new_unchecked(f64::NAN)
        });
        assert_ne!(unsafe { ExtendedFloat::new_unchecked(f64::NAN) }, unsafe {
            ExtendedFloat::new_unchecked(0.0)
        });
        assert_ne!(unsafe { ExtendedFloat::new_unchecked(f64::NAN) }, unsafe {
            ExtendedFloat::new_unchecked(1.0)
        });

        // Infinity comparisons
        assert_eq!(
            unsafe { ExtendedFloat::new_unchecked(f64::INFINITY) },
            unsafe { ExtendedFloat::new_unchecked(f64::INFINITY) }
        );
        assert_eq!(
            unsafe { ExtendedFloat::new_unchecked(f64::NEG_INFINITY) },
            unsafe { ExtendedFloat::new_unchecked(f64::NEG_INFINITY) }
        );
        assert_ne!(
            unsafe { ExtendedFloat::new_unchecked(f64::INFINITY) },
            unsafe { ExtendedFloat::new_unchecked(f64::NEG_INFINITY) }
        );
        assert_ne!(
            unsafe { ExtendedFloat::new_unchecked(f64::INFINITY) },
            unsafe { ExtendedFloat::new_unchecked(0.0) }
        );
        assert_ne!(
            unsafe { ExtendedFloat::new_unchecked(f64::NEG_INFINITY) },
            unsafe { ExtendedFloat::new_unchecked(0.0) }
        );
    }

    #[test]
    fn test_eq_trait_usage() {
        let values = [
            (ExtendedFloat::new(1.0), "one"),
            (ExtendedFloat::new(2.0), "two"),
        ];

        let found = values.iter().find(|(k, _)| *k == ExtendedFloat::new(1.0));
        assert_eq!(found, Some(&(ExtendedFloat::new(1.0), "one")));

        let found = values
            .iter()
            .find(|(k, _)| *k == ExtendedFloat::new(1.0 + EPSILON_F64 * 0.5));
        assert_eq!(found, Some(&(ExtendedFloat::new(1.0), "one")));

        let found = values
            .iter()
            .find(|(k, _)| *k == ExtendedFloat::new(1.0 + EPSILON_F64 * 10.0));
        assert_eq!(found, None);
    }

    #[test]
    fn test_transitivity() {
        // Test transitivity property of equality
        // If a == b and b == c, then a == c
        let a = ExtendedFloat::new(1.0);
        let b = ExtendedFloat::new(1.0 + EPSILON_F64 * 0.3);
        let c = ExtendedFloat::new(1.0 + EPSILON_F64 * 0.6);

        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }

    #[test]
    fn test_option_equality() {
        let none_f64: Option<ExtendedFloat<f64>> = None;
        assert_eq!(none_f64, none_f64);

        assert_eq!(Some(ExtendedFloat::new(1.0)), Some(ExtendedFloat::new(1.0)));

        assert_eq!(
            Some(ExtendedFloat::new(1.0)),
            Some(ExtendedFloat::new(1.0 + EPSILON_F64 * 0.5))
        );

        assert_ne!(none_f64, Some(ExtendedFloat::new(1.0)));
        assert_ne!(Some(ExtendedFloat::new(1.0)), none_f64);

        assert_ne!(Some(ExtendedFloat::new(1.0)), Some(ExtendedFloat::new(2.0)));
        assert_ne!(
            Some(ExtendedFloat::new(1.0)),
            Some(ExtendedFloat::new(1.0 + EPSILON_F64 * 10.0))
        );

        let opt1 = Some(ExtendedFloat::new(1.0));
        let opt2 = Some(ExtendedFloat::new(1.0));
        let none_opt: Option<ExtendedFloat<f64>> = None;

        let ref1 = &opt1;
        let ref2 = &opt2;
        let ref_none = &none_opt;

        assert_eq!(ref1, ref2);
        assert_ne!(ref1, ref_none);
    }

    #[test]
    fn test_smart_pointers_equality() {
        let opt1 = Some(ExtendedFloat::new(1.0));
        let opt2 = Some(ExtendedFloat::new(1.0));
        let none_opt: Option<ExtendedFloat<f64>> = None;

        let rc1 = Rc::new(opt1);
        let rc2 = Rc::new(opt2);
        let rc_none = Rc::new(none_opt);

        assert_eq!(*rc1, *rc2);
        assert_ne!(*rc1, *rc_none);
    }
}

// TODO: benches
