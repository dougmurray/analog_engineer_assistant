use super::{FormulaEntry, SolveVariant, VarDef};

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        FormulaEntry {
            name: "LSB Size",
            note: None,
            variants: &[SolveVariant {
                solves_for: "LSB",
                expression: "LSB = FSR / 2ⁿ",
                inputs: &[
                    VarDef {
                        symbol: "FSR",
                        name: "Full-scale range",
                        unit: "V",
                        default: 5.0,
                    },
                    VarDef {
                        symbol: "n",
                        name: "Resolution",
                        unit: "bits",
                        default: 12.0,
                    },
                ],
                output_unit: "V",
                compute: |v| v[0] / 2f64.powi(v[1] as i32),
            }],
        },
        FormulaEntry {
            name: "Output Voltage from Code",
            note: None,
            variants: &[
                SolveVariant {
                    solves_for: "V_out",
                    expression: "V_out = code × (FSR / 2ⁿ)",
                    inputs: &[
                        VarDef {
                            symbol: "code",
                            name: "Input code",
                            unit: "",
                            default: 2048.0,
                        },
                        VarDef {
                            symbol: "FSR",
                            name: "Full-scale range",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "n",
                            name: "Resolution",
                            unit: "bits",
                            default: 12.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] * v[1] / 2f64.powi(v[2] as i32),
                },
                SolveVariant {
                    solves_for: "code",
                    expression: "code = V_out × 2ⁿ / FSR",
                    inputs: &[
                        VarDef {
                            symbol: "V_out",
                            name: "Output voltage",
                            unit: "V",
                            default: 2.5,
                        },
                        VarDef {
                            symbol: "FSR",
                            name: "Full-scale range",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "n",
                            name: "Resolution",
                            unit: "bits",
                            default: 12.0,
                        },
                    ],
                    output_unit: "codes",
                    compute: |v| (v[0] * 2f64.powi(v[2] as i32) / v[1]).round(),
                },
            ],
        },
        FormulaEntry {
            name: "Full-Scale Output Voltage",
            note: None,
            variants: &[SolveVariant {
                solves_for: "V_FS",
                expression: "V_FS = (2ⁿ − 1) × LSB",
                inputs: &[
                    VarDef {
                        symbol: "FSR",
                        name: "Full-scale range",
                        unit: "V",
                        default: 5.0,
                    },
                    VarDef {
                        symbol: "n",
                        name: "Resolution",
                        unit: "bits",
                        default: 12.0,
                    },
                ],
                output_unit: "V",
                compute: |v| {
                    let lsb = v[0] / 2f64.powi(v[1] as i32);
                    (2f64.powi(v[1] as i32) - 1.0) * lsb
                },
            }],
        },
        FormulaEntry {
            name: "Gain Error",
            note: None,
            variants: &[SolveVariant {
                solves_for: "GE",
                expression: "GE(%) = ((m_measured − m_ideal) / m_ideal) × 100",
                inputs: &[
                    VarDef {
                        symbol: "m_meas",
                        name: "Measured slope",
                        unit: "V/code",
                        default: 1.22e-3,
                    },
                    VarDef {
                        symbol: "m_ideal",
                        name: "Ideal slope",
                        unit: "V/code",
                        default: 1.221e-3,
                    },
                ],
                output_unit: "%",
                compute: |v| ((v[0] - v[1]) / v[1]) * 100.0,
            }],
        },
        FormulaEntry {
            name: "Total Unadjusted Error (TUE)",
            note: None,
            variants: &[SolveVariant {
                solves_for: "TUE",
                expression: "TUE = √(OffsetErr² + GainErr² + INLErr²)",
                inputs: &[
                    VarDef {
                        symbol: "V_offset",
                        name: "Offset error",
                        unit: "LSB",
                        default: 0.5,
                    },
                    VarDef {
                        symbol: "V_gain",
                        name: "Gain error",
                        unit: "LSB",
                        default: 0.5,
                    },
                    VarDef {
                        symbol: "INL",
                        name: "INL error",
                        unit: "LSB",
                        default: 1.0,
                    },
                ],
                output_unit: "LSB",
                compute: |v| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt(),
            }],
        },
    ]
}
