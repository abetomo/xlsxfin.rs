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

pub fn ipmt(rate: f64, per: i64, nper: i64, pv: i64, fv: i64, payment_flag: bool) -> f64 {
    if nper == 0 {
        return 0.0;
    }

    if per == 0 {
        return 0.0;
    }

    if rate < 0.0 {
        return 0.0;
    }

    let pmt = pmt(rate, nper, pv, fv, false);
    let per_sub_1_f64 = (per - 1) as f64;

    let n = if rate.abs() > 0.5 {
        (1.0 + rate).powf(per_sub_1_f64)
    } else {
        (per_sub_1_f64 * (1.0 + rate).ln()).exp()
    };

    let m = (per_sub_1_f64 * (1.0 + rate).ln()).exp() - 1.0;

    let ip = -((pv as f64) * n * rate + pmt * m);
    if !payment_flag {
        return ip;
    }
    return ip / (1.0 + rate);
}

#[cfg(test)]
mod tests_ipmt {
    use super::*;

    #[derive(Debug)]
    struct TestArgs {
        rate: f64,
        per: i64,
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
    fn test_per_is_0() {
        let actual = ipmt(0.3, 0, 36, 100_000, 0, false);
        assert_eq!(actual, 0.0);
    }

    #[test]
    fn test_nper_is_0() {
        let actual = ipmt(0.3, 3, 0, 100_000, 0, false);
        assert_eq!(actual, 0.0);
    }

    #[test]
    fn test_rate_less_than_0() {
        let actual = ipmt(-0.1, 3, 36, 100_000, 0, false);
        assert_eq!(actual, 0.0);
    }

    #[test]
    fn test_rate_is_over_0() {
        let test_cases: [TestData; 8] = [
            TestData {
                args: TestArgs {
                    rate: 0.1,
                    per: 2,
                    nper: 36,
                    pv: 800_000,
                    fv: 0,
                    payment_flag: false,
                },
                expected: -79_732.55489453014,
            },
            TestData {
                args: TestArgs {
                    rate: 0.1,
                    per: 2,
                    nper: 36,
                    pv: 800_000,
                    fv: 0,
                    payment_flag: true,
                },
                expected: -72_484.14081320922,
            },
            TestData {
                args: TestArgs {
                    rate: 0.1,
                    per: 2,
                    nper: 36,
                    pv: 800_000,
                    fv: 1_000,
                    payment_flag: false,
                },
                expected: -79_732.22058814831,
            },
            TestData {
                args: TestArgs {
                    rate: 0.1,
                    per: 2,
                    nper: 36,
                    pv: 800_000,
                    fv: 1_000,
                    payment_flag: true,
                },
                expected: -72_483.83689831664,
            },
            TestData {
                args: TestArgs {
                    rate: 0.6,
                    per: 2,
                    nper: 36,
                    pv: 800_000,
                    fv: 0,
                    payment_flag: false,
                },
                expected: -479_999.9870856327,
            },
            TestData {
                args: TestArgs {
                    rate: 0.6,
                    per: 2,
                    nper: 36,
                    pv: 800_000,
                    fv: 0,
                    payment_flag: true,
                },
                expected: -299_999.99192852044,
            },
            TestData {
                args: TestArgs {
                    rate: 0.6,
                    per: 2,
                    nper: 36,
                    pv: 800_000,
                    fv: 1_000,
                    payment_flag: false,
                },
                expected: -479_999.9870694897,
            },
            TestData {
                args: TestArgs {
                    rate: 0.6,
                    per: 2,
                    nper: 36,
                    pv: 800_000,
                    fv: 1_000,
                    payment_flag: true,
                },
                expected: -299_999.9919184311,
            },
        ];
        for t in &test_cases {
            let actual = ipmt(
                t.args.rate,
                t.args.per,
                t.args.nper,
                t.args.pv,
                t.args.fv,
                t.args.payment_flag,
            );
            assert_eq!(actual, t.expected, "args: {:#?}", t.args);
        }
    }
}

pub fn fv(rate: f64, nper: i64, pmt: f64, pv: i64, payment_flag: bool) -> f64 {
    let pv_f64 = pv as f64;
    let nper_f64 = nper as f64;

    if rate == 0.0 {
        return -(pv_f64 + pmt * nper_f64);
    }

    let term = (1.0 + rate).powf(nper_f64);
    if payment_flag {
        return -(pv_f64 * term + (pmt * (1.0 + rate) * (term - 1.0)) / rate);
    }
    return -(pv_f64 * term + (pmt * (term - 1.0)) / rate);
}

#[cfg(test)]
mod tests_fv {
    use super::*;

    #[derive(Debug)]
    struct TestArgs {
        rate: f64,
        nper: i64,
        pmt: f64,
        pv: i64,
        payment_flag: bool,
    }

    struct TestData {
        args: TestArgs,
        expected: f64,
    }

    #[test]
    fn test_rate_is_0() {
        let test_cases: [TestData; 4] = [
            TestData {
                args: TestArgs {
                    rate: 0.0,
                    nper: 12,
                    pmt: 10_000.0,
                    pv: 0,
                    payment_flag: false,
                },
                expected: -120_000.0,
            },
            TestData {
                args: TestArgs {
                    rate: 0.0,
                    nper: 12,
                    pmt: 10_000.0,
                    pv: 0,
                    payment_flag: true,
                },
                expected: -120_000.0,
            },
            TestData {
                args: TestArgs {
                    rate: 0.0,
                    nper: 12,
                    pmt: 10_000.0,
                    pv: 1_000,
                    payment_flag: false,
                },
                expected: -121_000.0,
            },
            TestData {
                args: TestArgs {
                    rate: 0.0,
                    nper: 12,
                    pmt: 10_000.0,
                    pv: 1_000,
                    payment_flag: true,
                },
                expected: -121_000.0,
            },
        ];
        for t in &test_cases {
            let actual = fv(
                t.args.rate,
                t.args.nper,
                t.args.pmt,
                t.args.pv,
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
                    rate: 0.1,
                    nper: 12,
                    pmt: 10_000.0,
                    pv: 0,
                    payment_flag: false,
                },
                expected: -213_842.83767210032,
            },
            TestData {
                args: TestArgs {
                    rate: 0.1,
                    nper: 12,
                    pmt: 10_000.0,
                    pv: 0,
                    payment_flag: true,
                },
                expected: -235_227.12143931031,
            },
            TestData {
                args: TestArgs {
                    rate: 0.1,
                    nper: 12,
                    pmt: 10_000.0,
                    pv: 1_000,
                    payment_flag: false,
                },
                expected: -216_981.26604882133,
            },
            TestData {
                args: TestArgs {
                    rate: 0.1,
                    nper: 12,
                    pmt: 10_000.0,
                    pv: 1_000,
                    payment_flag: true,
                },
                expected: -238_365.54981603133,
            },
        ];
        for t in &test_cases {
            let actual = fv(
                t.args.rate,
                t.args.nper,
                t.args.pmt,
                t.args.pv,
                t.args.payment_flag,
            );
            assert_eq!(actual, t.expected, "args: {:#?}", t.args);
        }
    }
}
