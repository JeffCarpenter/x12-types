use crate::v004010::*;

#[test]
fn render_204() {
    let obj = _204 {
        st: ST {
            _01: "204".to_string(),
            _02: "18711".to_string(),
        },
        b2: B2 {
            _02: Some("SNDR".to_string()),
            _04: Some("6XULT02DCM".to_string()),
            _06: "DE".to_string(),
            ..Default::default()
        },
        b2a: B2A {
            _01: "00".to_string(),
            _02: None,
        },
        l11: Some(L11 {
            _01: Some("6XULT02DCM".to_string()),
            _02: Some("9R".to_string()),
            ..Default::default()
        }),
        g62: Some(G62 {
            _01: Some("04".to_string()),
            _02: Some("20221121".to_string()),
            _03: None,
            _04: Some("1513".to_string()),
            _05: Some("LT".to_string()),
        }),
        at5: Some(AT5 {
            _01: Some("XP".to_string()),
            _02: None,
            _03: Some("EXPORT".to_string()),
        }),
        loop_100: vec![
            _204Loop100 {
                n1: Some(N1 {
                    _01: "SH".to_string(),
                    _02: Some("EXPORT US LLC".to_string()),
                    _03: Some("93".to_string()),
                    _04: Some("753244123".to_string()),
                    ..Default::default()
                }),
                n3: vec![N3 {
                    _01: "2400 WASHINGTON AVE".to_string(),
                    ..Default::default()
                }],
                n4: Some(N4 {
                    _01: Some("CHICAGO".to_string()),
                    _02: Some("IL".to_string()),
                    _03: Some("60609".to_string()),
                    _04: Some("US".to_string()),
                    ..Default::default()
                }),
                g61: vec![G61 {
                    _01: "IC".to_string(),
                    _02: "N/A".to_string(),
                    _03: Some("TE".to_string()),
                    _04: Some("1 283737543".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
            _204Loop100 {
                n1: Some(N1 {
                    _01: "CA".to_string(),
                    _02: Some("HS".to_string()),
                    ..Default::default()
                }),
                n3: vec![N3 {
                    _01: "1500 MADISON ST".to_string(),
                    _02: Some("F 2 STE 340".to_string()),
                    ..Default::default()
                }],
                n4: Some(N4 {
                    _01: Some("PHILADELPHIA".to_string()),
                    _02: Some("PA".to_string()),
                    _03: Some("19103".to_string()),
                    _04: Some("US".to_string()),
                    ..Default::default()
                }),
                g61: vec![G61 {
                    _01: "IC".to_string(),
                    _02: "TONY X".to_string(),
                    _03: Some("TE".to_string()),
                    _04: Some("7462437293".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
            _204Loop100 {
                n1: Some(N1 {
                    _01: "BT".to_string(),
                    _02: Some("SUD NORTH AMERICA,".to_string()),
                    ..Default::default()
                }),
                n3: vec![N3 {
                    _01: "1000 PARK AVE".to_string(),
                    ..Default::default()
                }],
                n4: Some(N4 {
                    _01: Some("NICK SAM".to_string()),
                    _02: Some("NJ".to_string()),
                    _03: Some("07932".to_string()),
                    _04: Some("US".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ],
        loop_200: vec![],
        loop_300: vec![
            _204Loop300 {
                s5: S5 {
                    _01: "1".to_string(),
                    _02: "LD".to_string(),
                    ..Default::default()
                },
                loop_310: vec![_204Loop310 {
                    n1: Some(N1 {
                        _01: "PW".to_string(),
                        _02: Some("BNSF OAKLAND RAMP".to_string()),
                        ..Default::default()
                    }),
                    n3: vec![N3 {
                        _01: "333 MARITIME STREET".to_string(),
                        ..Default::default()
                    }],
                    n4: Some(N4 {
                        _01: Some("OAKLAND".to_string()),
                        _02: Some("CA".to_string()),
                        _03: Some("94607".to_string()),
                        _04: Some("US".to_string()),
                        _05: Some("ZZ".to_string()),
                        _06: Some("USOAKBNOA".to_string()),
                    }),
                    ..Default::default()
                }],
                loop_320: vec![_204Loop320 {
                    l5: Some(L5 {
                        _01: Some("1".to_string()),
                        _02: Some("FOOD:(NOS)".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                loop_350: vec![],
                loop_380: vec![_204Loop380 {
                    n7: Some(N7 {
                        _01: Some("MSKU".to_string()),
                        _02: "777374".to_string(),
                        _03: Some("43784.03".to_string()),
                        _04: Some("G".to_string()),
                        _05: Some("4784".to_string()),
                        _11: Some("CN".to_string()),
                        _12: Some("SNDR".to_string()),
                        _16: Some("A".to_string()),
                        _17: Some("L".to_string()),
                        _18: Some("9".to_string()),
                        _19: Some("RP".to_string()),
                        _22: Some("2200".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            },
            _204Loop300 {
                s5: S5 {
                    _01: "2".to_string(),
                    _02: "DT".to_string(),
                    ..Default::default()
                },
                loop_310: vec![_204Loop310 {
                    n1: Some(N1 {
                        _01: "DA".to_string(),
                        _02: Some("OAKLAND INTERNATIONAL CONTAINER TERMINAL".to_string()),
                        ..Default::default()
                    }),
                    n3: vec![N3 {
                        _01: "1717 MIDDLE HARBOR RD.".to_string(),
                        _02: Some("BERTH 58".to_string()),
                    }],
                    n4: Some(N4 {
                        _01: Some("OAKLAND".to_string()),
                        _02: Some("CA".to_string()),
                        _03: Some("94607".to_string()),
                        _04: Some("US".to_string()),
                        _05: Some("ZZ".to_string()),
                        _06: Some("USOAKSSAM".to_string()),
                    }),
                    g61: vec![G61 {
                        _01: "IC".to_string(),
                        _02: "N/A".to_string(),
                        _03: Some("TE".to_string()),
                        _04: Some("NOT AVAILABLE".to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            },
        ],
        se: SE {
            _01: "37".to_string(),
            _02: "18711".to_string(),
        },
        ..Default::default()
    };
    let obj = serde_x12::to_string(&obj).unwrap();
    println!("{}", obj);
}

#[test]
fn test_parse_204() {
    let str = r#"ST*204*18711~
B2**SNDR**6XULT02DCM**DE~
B2A*00~
L11*6XULT02DCM*9R~
G62*04*20221121**1513*LT~
AT5*XO**EXPORT~
N1*SH*EXPORT US LLC*93*753244123~
N3*2400 WASHINGTON AVE~
N4*CHICAGO*IL*60609*US~
G61*IC*N/A*TE*1 283737543~
N1*CA*HS~
N3*1500 MADISON ST*F 2 STE 340~
N4*PHILADELPHIA*PA*19103*US~
G61*IC*TONY X*TE*7462437293~
N1*BT*SUD NORTH AMERICA,~
N3*1000 PARK AVE~
N4*NICK SAM*NJ*07932*US~
S5*1*LD~
N1*PW*BNSF OAKLAND RAMP~
N3*333 MARITIME STREET~
N4*OAKLAND*CA*94607*US*ZZ*USOAKBNOA~
L5*1*FOOD:(NOS)~
N7*MSKU*777374*43784.03*G*4784******CN*SNDR****A*L*9*RP***2200~
S5*2*DT~
N1*DA*OAKLAND INTERNATIONAL CONTAINER TERMINAL~
N3*1717 MIDDLE HARBOR RD.*BERTH 58~
N4*OAKLAND*CA*94607*US*ZZ*USOAKSSAM~
G61*IC*N/A*TE*NOT AVAILABLE~
SE*37*18711~"#;
    let (str, obj) = _204::parse(str).unwrap();
    println!("{:?}", obj);
    assert!(str.is_empty());
    assert_eq!(obj.se._01, "37");
    assert_eq!(obj.se._02, "18711");
}
