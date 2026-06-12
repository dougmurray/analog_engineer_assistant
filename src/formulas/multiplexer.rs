use super::{FormulaEntry, SolveVariant, VarDef};

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        FormulaEntry {
            name: "Op-Amp Gain with MUX R_ON",
            note: None,
            variants: &[SolveVariant {
                solves_for: "AG",
                expression: "AG = −R_F / (R1 + R_ON)",
                inputs: &[
                    VarDef {
                        symbol: "R_F",
                        name: "Feedback resistor",
                        unit: "Ω",
                        default: 500.0,
                    },
                    VarDef {
                        symbol: "R1",
                        name: "Input resistor",
                        unit: "Ω",
                        default: 500.0,
                    },
                    VarDef {
                        symbol: "R_ON",
                        name: "MUX ON resistance",
                        unit: "Ω",
                        default: 100.0,
                    },
                ],
                output_unit: "V/V",
                compute: |v| -v[0] / (v[1] + v[2]),
            }],
        },
        FormulaEntry {
            name: "MUX ON Capacitance",
            note: None,
            variants: &[SolveVariant {
                solves_for: "C_ON",
                expression: "C_ON ≈ C_S + C_D",
                inputs: &[
                    VarDef {
                        symbol: "C_S",
                        name: "Source capacitance",
                        unit: "F",
                        default: 5e-12,
                    },
                    VarDef {
                        symbol: "C_D",
                        name: "Drain capacitance",
                        unit: "F",
                        default: 5e-12,
                    },
                ],
                output_unit: "F",
                compute: |v| v[0] + v[1],
            }],
        },
        FormulaEntry {
            name: "MUX Settling Time (Simple)",
            note: None,
            variants: &[SolveVariant {
                solves_for: "t_settle",
                expression: "t_settle = t_transition + R_ON × C_D × K",
                inputs: &[
                    VarDef {
                        symbol: "t_tr",
                        name: "Channel transition time",
                        unit: "s",
                        default: 92e-9,
                    },
                    VarDef {
                        symbol: "R_ON",
                        name: "MUX ON resistance",
                        unit: "Ω",
                        default: 125.0,
                    },
                    VarDef {
                        symbol: "C_D",
                        name: "Drain capacitance",
                        unit: "F",
                        default: 7.5e-12,
                    },
                    VarDef {
                        symbol: "K",
                        name: "Time constants (ln(2ᴺ))",
                        unit: "",
                        default: 9.704,
                    },
                ],
                output_unit: "s",
                compute: |v| v[0] + v[1] * v[2] * v[3],
            }],
        },
        FormulaEntry {
            name: "MUX Settling Time (with R_LOAD / C_LOAD)",
            note: None,
            variants: &[SolveVariant {
                solves_for: "t_settle",
                expression: "t_settle = t_tr + (R_ON‖R_L)×(C_L+C_D)×K",
                inputs: &[
                    VarDef {
                        symbol: "t_tr",
                        name: "Channel transition time",
                        unit: "s",
                        default: 92e-9,
                    },
                    VarDef {
                        symbol: "R_ON",
                        name: "MUX ON resistance",
                        unit: "Ω",
                        default: 125.0,
                    },
                    VarDef {
                        symbol: "R_L",
                        name: "Load resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "C_L",
                        name: "Load capacitance",
                        unit: "F",
                        default: 5e-12,
                    },
                    VarDef {
                        symbol: "C_D",
                        name: "Drain capacitance",
                        unit: "F",
                        default: 7.5e-12,
                    },
                    VarDef {
                        symbol: "K",
                        name: "Time constants (ln(2ᴺ))",
                        unit: "",
                        default: 9.704,
                    },
                ],
                output_unit: "s",
                compute: |v| {
                    let r_par = (v[1] * v[2]) / (v[1] + v[2]);
                    v[0] + r_par * (v[3] + v[4]) * v[5]
                },
            }],
        },
        FormulaEntry {
            name: "Leakage Error Voltage (Switch OFF)",
            note: None,
            variants: &[SolveVariant {
                solves_for: "V_error",
                expression: "V_error = R_L × I_D(OFF)",
                inputs: &[
                    VarDef {
                        symbol: "R_L",
                        name: "Load resistance",
                        unit: "Ω",
                        default: 1e6,
                    },
                    VarDef {
                        symbol: "I_D_OFF",
                        name: "OFF leakage current",
                        unit: "A",
                        default: 1e-10,
                    },
                ],
                output_unit: "V",
                compute: |v| v[0] * v[1],
            }],
        },
        FormulaEntry {
            name: "Leakage Error Voltage (Switch ON)",
            note: None,
            variants: &[SolveVariant {
                solves_for: "V_error",
                expression: "V_error = (R_ON + R_source) × I_D(ON)",
                inputs: &[
                    VarDef {
                        symbol: "R_ON",
                        name: "ON resistance",
                        unit: "Ω",
                        default: 100.0,
                    },
                    VarDef {
                        symbol: "R_source",
                        name: "Source resistance",
                        unit: "Ω",
                        default: 1e6,
                    },
                    VarDef {
                        symbol: "I_D_ON",
                        name: "ON leakage current",
                        unit: "A",
                        default: 100e-12,
                    },
                ],
                output_unit: "V",
                compute: |v| (v[0] + v[1]) * v[2],
            }],
        },
        FormulaEntry {
            name: "Charge Injection Error",
            note: None,
            variants: &[SolveVariant {
                solves_for: "V_error",
                expression: "V_error ≈ Q_INJ / C_L",
                inputs: &[
                    VarDef {
                        symbol: "C_D",
                        name: "Drain capacitance",
                        unit: "F",
                        default: 10e-12,
                    },
                    VarDef {
                        symbol: "C_L",
                        name: "Load capacitance",
                        unit: "F",
                        default: 100e-12,
                    },
                    VarDef {
                        symbol: "ΔV",
                        name: "Supply swing",
                        unit: "V",
                        default: 30.0,
                    },
                ],
                output_unit: "V",
                compute: |v| {
                    let q_inj = (v[0] + v[1]) * v[2];
                    q_inj / v[1]
                },
            }],
        },
        FormulaEntry {
            name: "MUX Bandwidth",
            note: None,
            variants: &[
                SolveVariant {
                    solves_for: "f_-3dB",
                    expression: "f_-3dB = 1 / (2π × (R_ON‖R_L) × (C_D+C_L))",
                    inputs: &[
                        VarDef {
                            symbol: "R_ON",
                            name: "MUX ON resistance",
                            unit: "Ω",
                            default: 100.0,
                        },
                        VarDef {
                            symbol: "R_L",
                            name: "Load resistance",
                            unit: "Ω",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "C_D",
                            name: "Drain capacitance",
                            unit: "F",
                            default: 5e-12,
                        },
                        VarDef {
                            symbol: "C_L",
                            name: "Load capacitance",
                            unit: "F",
                            default: 5e-12,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| {
                        let r_par = (v[0] * v[1]) / (v[0] + v[1]);
                        1.0 / (2.0 * std::f64::consts::PI * r_par * (v[2] + v[3]))
                    },
                },
                SolveVariant {
                    solves_for: "f_-3dB (R_L≫R_ON)",
                    expression: "f_-3dB ≈ 1 / (2π × R_ON × (C_D+C_L))",
                    inputs: &[
                        VarDef {
                            symbol: "R_ON",
                            name: "MUX ON resistance",
                            unit: "Ω",
                            default: 100.0,
                        },
                        VarDef {
                            symbol: "C_D",
                            name: "Drain capacitance",
                            unit: "F",
                            default: 5e-12,
                        },
                        VarDef {
                            symbol: "C_L",
                            name: "Load capacitance",
                            unit: "F",
                            default: 5e-12,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| 1.0 / (2.0 * std::f64::consts::PI * v[0] * (v[1] + v[2])),
                },
            ],
        },
        FormulaEntry {
            name: "Channel-to-Channel Crosstalk",
            note: None,
            variants: &[SolveVariant {
                solves_for: "X_TALK",
                expression: "X_TALK = 20 × log(V_out / V_S)",
                inputs: &[
                    VarDef {
                        symbol: "V_out",
                        name: "Voltage at OFF input",
                        unit: "V",
                        default: 1e-3,
                    },
                    VarDef {
                        symbol: "V_S",
                        name: "Voltage at ON input",
                        unit: "V",
                        default: 1.0,
                    },
                ],
                output_unit: "dB",
                compute: |v| 20.0 * (v[0] / v[1]).log10(),
            }],
        },
        FormulaEntry {
            name: "OFF-Isolation",
            note: None,
            variants: &[SolveVariant {
                solves_for: "OFF_iso",
                expression: "OFF_iso = 20 × log(V_out / V_in)",
                inputs: &[
                    VarDef {
                        symbol: "V_out",
                        name: "Output voltage (OFF ch)",
                        unit: "V",
                        default: 1e-4,
                    },
                    VarDef {
                        symbol: "V_in",
                        name: "Source voltage (OFF ch)",
                        unit: "V",
                        default: 1.0,
                    },
                ],
                output_unit: "dB",
                compute: |v| 20.0 * (v[0] / v[1]).log10(),
            }],
        },
    ]
}
