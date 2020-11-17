pub fn pmt(rate: f64, nper: i64, pv: i64, fv: i64, payment_flag: bool) -> f64 {
    if nper == 0 {
        return 0.0;
    }

    let nper_f64: f64 = nper as f64;
    let pv_f64: f64 = pv as f64;
    let fv_f64: f64 = fv as f64;

    if rate == 0.0 {
        return -(pv_f64 + fv_f64) / nper_f64;
    }

    let pvif = (1.0 + rate).powf(nper_f64);
    let pmt = (rate / (pvif - 1.0)) * -(pv_f64 * pvif + fv_f64);

    if !payment_flag {
        return pmt;
    }
    return pmt / (1.0 + rate);
}

#[cfg(test)]
mod tests_pmt {
    use super::*;

    #[derive(Debug)]
    struct TestArgs {
        rate: f64,
        nper: i64,
        pv: i64,
        fv: i64,
        payment_flag: bool,
    }

    struct TestData {
        args: TestArgs,
        expected: f64,
    }

    #[test]
    fn test_nper_is_0() {
        let actual = pmt(0.3, 0, 100_000, 0, false);
        assert_eq!(actual, 0.0);
    }

    #[test]
    fn test_rate_is_0() {
        let test_cases: [TestData; 4] = [
            TestData {
                args: TestArgs {
                    rate: 0.0,
                    nper: 36,
                    pv: 100_000,
                    fv: 0,
                    payment_flag: false,
                },
                expected: -2_777.777777777778,
            },
            TestData {
                args: TestArgs {
                    rate: 0.0,
                    nper: 36,
                    pv: 100_000,
                    fv: 0,
                    payment_flag: true,
                },
                expected: -2_777.777777777778,
            },
            TestData {
                args: TestArgs {
                    rate: 0.0,
                    nper: 36,
                    pv: 100_000,
                    fv: 1_000,
                    payment_flag: false,
                },
                expected: -2805.5555555555557,
            },
            TestData {
                args: TestArgs {
                    rate: 0.0,
                    nper: 36,
                    pv: 100_000,
                    fv: 1_000,
                    payment_flag: true,
                },
                expected: -2805.5555555555557,
            },
        ];
        for t in &test_cases {
            let actual = pmt(
                t.args.rate,
                t.args.nper,
                t.args.pv,
                t.args.fv,
                t.args.payment_flag,
            );
            assert_eq!(actual, t.expected, "args: {:#?}", t.args);
        }
    }

    #[test]
    fn test_rate_is_over_0() {
        let test_cases: [TestData; 4] = [
            TestData {
                args: TestArgs {
                    rate: 0.3,
                    nper: 36,
                    pv: 100_000,
                    fv: 0,
                    payment_flag: false,
                },
                expected: -30_002.37243823623,
            },
            TestData {
                args: TestArgs {
                    rate: 0.3,
                    nper: 36,
                    pv: 100_000,
                    fv: 0,
                    payment_flag: true,
                },
                expected: -23078.748029412483,
            },
            TestData {
                args: TestArgs {
                    rate: 0.3,
                    nper: 36,
                    pv: 100_000,
                    fv: 1_000,
                    payment_flag: false,
                },
                expected: -30_002.396162618596,
            },
            TestData {
                args: TestArgs {
                    rate: 0.3,
                    nper: 36,
                    pv: 100_000,
                    fv: 1_000,
                    payment_flag: true,
                },
                expected: -23078.76627893738,
            },
        ];
        for t in &test_cases {
            let actual = pmt(
                t.args.rate,
                t.args.nper,
                t.args.pv,
                t.args.fv,
                t.args.payment_flag,
            );
            assert_eq!(actual, t.expected, "args: {:#?}", t.args);
        }
    }
}
