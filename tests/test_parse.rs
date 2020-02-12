include!("../build/rustc.rs");

#[test]
fn test_parse() {
    let cases = &[
        (
            "rustc 1.0.0 (a59de37e9 2015-05-13) (built 2015-05-14)",
            Version {
                minor: 0,
                patch: 0,
                channel: Stable,
            },
        ),
        (
            "rustc 1.18.0",
            Version {
                minor: 18,
                patch: 0,
                channel: Stable,
            },
        ),
        (
            "rustc 1.24.1 (d3ae9a9e0 2018-02-27)",
            Version {
                minor: 24,
                patch: 1,
                channel: Stable,
            },
        ),
        (
            "rustc 1.35.0-beta.3 (c13114dc8 2019-04-27)",
            Version {
                minor: 35,
                patch: 0,
                channel: Beta,
            },
        ),
        (
            "rustc 1.36.0-nightly (938d4ffe1 2019-04-27)",
            Version {
                minor: 36,
                patch: 0,
                channel: Nightly(Date {
                    year: 2019,
                    month: 4,
                    day: 27,
                }),
            },
        ),
        (
            "rustc 1.36.0-dev",
            Version {
                minor: 36,
                patch: 0,
                channel: Dev,
            },
        ),
        (
            "rustc 1.40.0 (73528e339 2019-12-16)
            binary: rustc
            commit-hash: 73528e339aae0f17a15ffa49a8ac608f50c6cf14
            commit-date: 2019-12-16
            host: x86_64-unknown-linux-gnu
            release: 1.40.0
            LLVM version: 9.0",
            Version {
                minor: 40,
                patch: 0,
                channel: Stable,
            },
        ),
        (
            "rustc 1.36.0-nightly",
            Version {
                minor: 36,
                patch: 0,
                channel: Dev,
            },
        ),
        (
            "warning: invalid logging spec 'warning', ignoring it
             rustc 1.30.0-nightly (3bc2ca7e4 2018-09-20)",
            Version {
                minor: 30,
                patch: 0,
                channel: Nightly(Date {
                    year: 2018,
                    month: 9,
                    day: 20,
                }),
            },
        ),
    ];

    for (string, expected) in cases {
        assert_eq!(
            parse(string).as_ref(),
            Some(expected),
            "string {} expected {:#?}",
            string,
            expected
        );
    }
}
