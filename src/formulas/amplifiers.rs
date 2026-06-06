use super::{FormulaEntry, SolveVariant, VarDef};

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        FormulaEntry {
            name: "Non-Inverting Gain",
            variants: &[
                SolveVariant {
                    solves_for: "V_out",
                    expression: "V_out = V_in × (1 + R_f / R_g)",
                    inputs: &[
                        VarDef {
                            symbol: "V_in",
                            name: "Input voltage",
                            unit: "V",
                            default: 1.0,
                        },
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] * (1.0 + v[1] / v[2]),
                },
                SolveVariant {
                    solves_for: "R_f",
                    expression: "R_f = R_g × (V_out / V_in − 1)",
                    inputs: &[
                        VarDef {
                            symbol: "V_in",
                            name: "Input voltage",
                            unit: "V",
                            default: 1.0,
                        },
                        VarDef {
                            symbol: "V_out",
                            name: "Output voltage",
                            unit: "V",
                            default: 10.0,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[2] * (v[1] / v[0] - 1.0),
                },
                SolveVariant {
                    solves_for: "R_g",
                    expression: "R_g = R_f × V_in / (V_out − V_in)",
                    inputs: &[
                        VarDef {
                            symbol: "V_in",
                            name: "Input voltage",
                            unit: "V",
                            default: 1.0,
                        },
                        VarDef {
                            symbol: "V_out",
                            name: "Output voltage",
                            unit: "V",
                            default: 10.0,
                        },
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[2] * v[0] / (v[1] - v[0]),
                },
                SolveVariant {
                    solves_for: "V_in",
                    expression: "V_in = V_out / (1 + R_f / R_g)",
                    inputs: &[
                        VarDef {
                            symbol: "V_out",
                            name: "Output voltage",
                            unit: "V",
                            default: 10.0,
                        },
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] / (1.0 + v[1] / v[2]),
                },
            ],
        },
        FormulaEntry {
            name: "Inverting Gain",
            variants: &[
                SolveVariant {
                    solves_for: "V_out",
                    expression: "V_out = −V_in × (R_f / R_g)",
                    inputs: &[
                        VarDef {
                            symbol: "V_in",
                            name: "Input voltage",
                            unit: "V",
                            default: 1.0,
                        },
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| -(v[0] * (v[1] / v[2])),
                },
                SolveVariant {
                    solves_for: "R_f",
                    expression: "R_f = −V_out × R_g / V_in",
                    inputs: &[
                        VarDef {
                            symbol: "V_in",
                            name: "Input voltage",
                            unit: "V",
                            default: 1.0,
                        },
                        VarDef {
                            symbol: "V_out",
                            name: "Output voltage",
                            unit: "V",
                            default: -9.0,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| -v[1] * v[2] / v[0],
                },
                SolveVariant {
                    solves_for: "R_g",
                    expression: "R_g = −R_f × V_in / V_out",
                    inputs: &[
                        VarDef {
                            symbol: "V_in",
                            name: "Input voltage",
                            unit: "V",
                            default: 1.0,
                        },
                        VarDef {
                            symbol: "V_out",
                            name: "Output voltage",
                            unit: "V",
                            default: -9.0,
                        },
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| -v[2] * v[0] / v[1],
                },
                SolveVariant {
                    solves_for: "V_in",
                    expression: "V_in = −V_out × R_g / R_f",
                    inputs: &[
                        VarDef {
                            symbol: "V_out",
                            name: "Output voltage",
                            unit: "V",
                            default: -9.0,
                        },
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| -v[0] * v[2] / v[1],
                },
            ],
        },
        FormulaEntry {
            name: "Noise Gain (Closed-Loop)",
            variants: &[
                SolveVariant {
                    solves_for: "G_n",
                    expression: "G_n = 1 + R_f / R_g",
                    inputs: &[
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "V/V",
                    compute: |v| 1.0 + v[0] / v[1],
                },
                SolveVariant {
                    solves_for: "R_f",
                    expression: "R_f = R_g × (G_n − 1)",
                    inputs: &[
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "G_n",
                            name: "Noise gain",
                            unit: "V/V",
                            default: 10.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[0] * (v[1] - 1.0),
                },
                SolveVariant {
                    solves_for: "R_g",
                    expression: "R_g = R_f / (G_n − 1)",
                    inputs: &[
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                        VarDef {
                            symbol: "G_n",
                            name: "Noise gain",
                            unit: "V/V",
                            default: 10.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[0] / (v[1] - 1.0),
                },
            ],
        },
        FormulaEntry {
            name: "Op-Amp Bandwidth",
            variants: &[
                SolveVariant {
                    solves_for: "BW",
                    expression: "BW = GBP / (1 + R_f / R_g)",
                    inputs: &[
                        VarDef {
                            symbol: "GBP",
                            name: "Gain-bandwidth product",
                            unit: "Hz",
                            default: 1e6,
                        },
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| v[0] / (1.0 + v[1] / v[2]),
                },
                SolveVariant {
                    solves_for: "R_f",
                    expression: "R_f = R_g × (GBP / BW − 1)",
                    inputs: &[
                        VarDef {
                            symbol: "BW",
                            name: "Bandwidth",
                            unit: "Hz",
                            default: 100e3,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "GBP",
                            name: "Gain-bandwidth product",
                            unit: "Hz",
                            default: 1e6,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[1] * (v[2] / v[0] - 1.0),
                },
                SolveVariant {
                    solves_for: "R_g",
                    expression: "R_g = R_f × BW / (GBP − BW)",
                    inputs: &[
                        VarDef {
                            symbol: "BW",
                            name: "Bandwidth",
                            unit: "Hz",
                            default: 100e3,
                        },
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                        VarDef {
                            symbol: "GBP",
                            name: "Gain-bandwidth product",
                            unit: "Hz",
                            default: 1e6,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[1] * v[0] / (v[2] - v[0]),
                },
                SolveVariant {
                    solves_for: "GBP",
                    expression: "GBP = BW × (1 + R_f / R_g)",
                    inputs: &[
                        VarDef {
                            symbol: "BW",
                            name: "Bandwidth",
                            unit: "Hz",
                            default: 100e3,
                        },
                        VarDef {
                            symbol: "R_f",
                            name: "Feedback resistor",
                            unit: "Ω",
                            default: 9000.0,
                        },
                        VarDef {
                            symbol: "R_g",
                            name: "Gain resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| v[0] * (1.0 + v[1] / v[2]),
                },
            ],
        },
        FormulaEntry {
            name: "Small-Signal Rise Time",
            variants: &[
                SolveVariant {
                    solves_for: "t_rise",
                    expression: "t_rise = 0.35 / BW",
                    inputs: &[VarDef {
                        symbol: "BW",
                        name: "Bandwidth",
                        unit: "Hz",
                        default: 1e6,
                    }],
                    output_unit: "s",
                    compute: |v| 0.35 / v[0],
                },
                SolveVariant {
                    solves_for: "BW",
                    expression: "BW = 0.35 / t_rise",
                    inputs: &[VarDef {
                        symbol: "t_rise",
                        name: "Rise time",
                        unit: "s",
                        default: 350e-9,
                    }],
                    output_unit: "Hz",
                    compute: |v| 0.35 / v[0],
                },
            ],
        },
        FormulaEntry {
            name: "Max Output Voltage (Slew Rate Limit)",
            variants: &[
                SolveVariant {
                    solves_for: "V_peak",
                    expression: "V_peak = SR / (2π × f)",
                    inputs: &[
                        VarDef {
                            symbol: "SR",
                            name: "Slew rate",
                            unit: "V/s",
                            default: 1e6,
                        },
                        VarDef {
                            symbol: "f",
                            name: "Frequency",
                            unit: "Hz",
                            default: 10e3,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] / (2.0 * std::f64::consts::PI * v[1]),
                },
                SolveVariant {
                    solves_for: "SR",
                    expression: "SR = 2π × f × V_peak",
                    inputs: &[
                        VarDef {
                            symbol: "f",
                            name: "Frequency",
                            unit: "Hz",
                            default: 10e3,
                        },
                        VarDef {
                            symbol: "V_peak",
                            name: "Peak output voltage",
                            unit: "V",
                            default: 15.915,
                        },
                    ],
                    output_unit: "V/s",
                    compute: |v| 2.0 * std::f64::consts::PI * v[0] * v[1],
                },
                SolveVariant {
                    solves_for: "f",
                    expression: "f = SR / (2π × V_peak)",
                    inputs: &[
                        VarDef {
                            symbol: "SR",
                            name: "Slew rate",
                            unit: "V/s",
                            default: 1e6,
                        },
                        VarDef {
                            symbol: "V_peak",
                            name: "Peak output voltage",
                            unit: "V",
                            default: 15.915,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| v[0] / (2.0 * std::f64::consts::PI * v[1]),
                },
            ],
        },
        FormulaEntry {
            name: "Offset Voltage RTI",
            variants: &[SolveVariant {
                solves_for: "V_os_RTI",
                expression: "V_os_RTI = √(V_os² + (I_b × R_f‖R_g)² + (I_b × R_ni)²)",
                inputs: &[
                    VarDef {
                        symbol: "V_os",
                        name: "Input offset voltage",
                        unit: "V",
                        default: 1e-4,
                    },
                    VarDef {
                        symbol: "I_b",
                        name: "Bias current",
                        unit: "A",
                        default: 1e-9,
                    },
                    VarDef {
                        symbol: "R_f",
                        name: "Feedback resistor",
                        unit: "Ω",
                        default: 9000.0,
                    },
                    VarDef {
                        symbol: "R_g",
                        name: "Gain resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "R_ni",
                        name: "Non-inv input resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                ],
                output_unit: "V",
                compute: |v| {
                    let rf_rg_par = (v[2] * v[3]) / (v[2] + v[3]);
                    let v_inv = v[1] * rf_rg_par;
                    let v_noninv = v[1] * v[4];
                    (v[0] * v[0] + v_inv * v_inv + v_noninv * v_noninv).sqrt()
                },
            }],
        },
        FormulaEntry {
            name: "Offset Voltage RTO",
            variants: &[SolveVariant {
                solves_for: "V_os_RTO",
                expression: "V_os_RTO = V_os_RTI × (1 + R_f / R_g)",
                inputs: &[
                    VarDef {
                        symbol: "V_os",
                        name: "Input offset voltage",
                        unit: "V",
                        default: 1e-4,
                    },
                    VarDef {
                        symbol: "I_b",
                        name: "Bias current",
                        unit: "A",
                        default: 1e-9,
                    },
                    VarDef {
                        symbol: "R_f",
                        name: "Feedback resistor",
                        unit: "Ω",
                        default: 9000.0,
                    },
                    VarDef {
                        symbol: "R_g",
                        name: "Gain resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "R_ni",
                        name: "Non-inv input resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                ],
                output_unit: "V",
                compute: |v| {
                    let rf_rg_par = (v[2] * v[3]) / (v[2] + v[3]);
                    let v_inv = v[1] * rf_rg_par;
                    let v_noninv = v[1] * v[4];
                    let rti = (v[0] * v[0] + v_inv * v_inv + v_noninv * v_noninv).sqrt();
                    rti * (1.0 + v[2] / v[3])
                },
            }],
        },
        FormulaEntry {
            name: "Resistor Thermal Noise",
            variants: &[
                SolveVariant {
                    solves_for: "V_n",
                    expression: "V_n ≈ √(R / 1kΩ) × 4 nV/√Hz",
                    inputs: &[VarDef {
                        symbol: "R",
                        name: "Resistance",
                        unit: "Ω",
                        default: 1000.0,
                    }],
                    output_unit: "V/√Hz",
                    compute: |v| (v[0] / 1000.0).sqrt() * 4.0e-9,
                },
                SolveVariant {
                    solves_for: "R",
                    expression: "R = (V_n / 4 nV/√Hz)² × 1 kΩ",
                    inputs: &[VarDef {
                        symbol: "V_n",
                        name: "Noise density",
                        unit: "V/√Hz",
                        default: 4e-9,
                    }],
                    output_unit: "Ω",
                    compute: |v| (v[0] / 4.0e-9) * (v[0] / 4.0e-9) * 1000.0,
                },
            ],
        },
        FormulaEntry {
            name: "Op-Amp RTI Noise (RMS)",
            variants: &[SolveVariant {
                solves_for: "V_n_RTI",
                expression: "V_n_RTI = √(V_n² + V_Rf‖Rg² + V_Rni² + (I_n×R_f‖R_g)² + (I_n×R_ni)²) × √BW",
                inputs: &[
                    VarDef {
                        symbol: "V_n",
                        name: "Voltage noise density",
                        unit: "V/√Hz",
                        default: 5e-9,
                    },
                    VarDef {
                        symbol: "I_n",
                        name: "Current noise density",
                        unit: "A/√Hz",
                        default: 1e-12,
                    },
                    VarDef {
                        symbol: "R_f",
                        name: "Feedback resistor",
                        unit: "Ω",
                        default: 9000.0,
                    },
                    VarDef {
                        symbol: "R_g",
                        name: "Gain resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "R_ni",
                        name: "Non-inv input resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "BW",
                        name: "Noise bandwidth",
                        unit: "Hz",
                        default: 10e3,
                    },
                ],
                output_unit: "V_rms",
                compute: |v| {
                    let rf_par_rg = (v[2] * v[3]) / (v[2] + v[3]);
                    let vn_rms = v[0] * v[5].sqrt() * (1.0 + v[2] / v[3]);
                    let in_inv = v[1] * v[5].sqrt() * v[2];
                    let in_noninv = v[1] * v[5].sqrt() * v[4] * (1.0 + v[2] / v[3]);
                    let vr_ni = (v[4] / 1000.0).sqrt() * 4.0e-9 * v[5].sqrt() * (1.0 + v[2] / v[3]);
                    let vr_rg = (rf_par_rg / 1000.0).sqrt() * 4.0e-9 * v[5].sqrt() * v[2];
                    let vr_f = (v[2] / 1000.0).sqrt() * 4.0e-9 * v[5].sqrt();
                    (vn_rms * vn_rms
                        + in_inv * in_inv
                        + in_noninv * in_noninv
                        + vr_ni * vr_ni
                        + vr_rg * vr_rg
                        + vr_f * vr_f)
                        .sqrt()
                },
            }],
        },
        FormulaEntry {
            name: "Op-Amp RTO Noise (RMS)",
            variants: &[SolveVariant {
                solves_for: "V_n_RTO",
                expression: "V_n_RTO = V_n_RTI × (1 + R_f / R_g)",
                inputs: &[
                    VarDef {
                        symbol: "V_n",
                        name: "Voltage noise density",
                        unit: "V/√Hz",
                        default: 5e-9,
                    },
                    VarDef {
                        symbol: "I_n",
                        name: "Current noise density",
                        unit: "A/√Hz",
                        default: 1e-12,
                    },
                    VarDef {
                        symbol: "R_f",
                        name: "Feedback resistor",
                        unit: "Ω",
                        default: 9000.0,
                    },
                    VarDef {
                        symbol: "R_g",
                        name: "Gain resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "R_ni",
                        name: "Non-inv input resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "BW",
                        name: "Noise bandwidth",
                        unit: "Hz",
                        default: 10e3,
                    },
                ],
                output_unit: "V_rms",
                compute: |v| {
                    let rf_par_rg = (v[2] * v[3]) / (v[2] + v[3]);
                    let vn_rms = v[0] * v[5].sqrt() * (1.0 + v[2] / v[3]);
                    let in_inv = v[1] * v[5].sqrt() * v[2];
                    let in_noninv = v[1] * v[5].sqrt() * v[4] * (1.0 + v[2] / v[3]);
                    let vr_ni = (v[4] / 1000.0).sqrt() * 4.0e-9 * v[5].sqrt() * (1.0 + v[2] / v[3]);
                    let vr_rg = (rf_par_rg / 1000.0).sqrt() * 4.0e-9 * v[5].sqrt() * v[2];
                    let vr_f = (v[2] / 1000.0).sqrt() * 4.0e-9 * v[5].sqrt();
                    let rti = (vn_rms * vn_rms
                        + in_inv * in_inv
                        + in_noninv * in_noninv
                        + vr_ni * vr_ni
                        + vr_rg * vr_rg
                        + vr_f * vr_f)
                        .sqrt();
                    rti * (1.0 + v[2] / v[3])
                },
            }],
        },
        FormulaEntry {
            name: "Op-Amp Power Dissipation (given V_out)",
            variants: &[SolveVariant {
                solves_for: "P_diss",
                expression: "P = (|I_load| + |I_fb|)×(|V_s|−|V_out|) + V_s×I_q",
                inputs: &[
                    VarDef {
                        symbol: "V_out",
                        name: "Output voltage",
                        unit: "V",
                        default: 2.5,
                    },
                    VarDef {
                        symbol: "R_load",
                        name: "Load resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "R_f",
                        name: "Feedback resistor",
                        unit: "Ω",
                        default: 9000.0,
                    },
                    VarDef {
                        symbol: "R_g",
                        name: "Gain resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "V_s",
                        name: "Supply voltage",
                        unit: "V",
                        default: 5.0,
                    },
                    VarDef {
                        symbol: "I_q",
                        name: "Quiescent current",
                        unit: "A",
                        default: 1e-3,
                    },
                ],
                output_unit: "W",
                compute: |v| {
                    let i_load = v[0] / v[1];
                    let i_fb = v[0] / (v[2] + v[3]);
                    (i_load.abs() + i_fb.abs()) * (v[4].abs() - v[0].abs()) + v[4] * v[5]
                },
            }],
        },
        FormulaEntry {
            name: "Op-Amp Max Power Dissipation",
            variants: &[SolveVariant {
                solves_for: "P_max",
                expression: "P_max = V_s² / (4 × R_eq)   [DC output]",
                inputs: &[
                    VarDef {
                        symbol: "V_s",
                        name: "Supply voltage",
                        unit: "V",
                        default: 5.0,
                    },
                    VarDef {
                        symbol: "R_load",
                        name: "Load resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "R_f",
                        name: "Feedback resistor",
                        unit: "Ω",
                        default: 9000.0,
                    },
                    VarDef {
                        symbol: "R_g",
                        name: "Gain resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                ],
                output_unit: "W",
                compute: |v| {
                    let r_eq = (v[1] * (v[2] + v[3])) / (v[1] + v[2] + v[3]);
                    v[0] * v[0] / (4.0 * r_eq)
                },
            }],
        },
        FormulaEntry {
            name: "Op-Amp Junction Temperature",
            variants: &[SolveVariant {
                solves_for: "T_j",
                expression: "T_j = θ_ja × P + T_amb",
                inputs: &[
                    VarDef {
                        symbol: "θ_ja",
                        name: "Thermal resistance θja",
                        unit: "°C/W",
                        default: 100.0,
                    },
                    VarDef {
                        symbol: "P",
                        name: "Power dissipated",
                        unit: "W",
                        default: 0.1,
                    },
                    VarDef {
                        symbol: "T_amb",
                        name: "Ambient temperature",
                        unit: "°C",
                        default: 25.0,
                    },
                ],
                output_unit: "°C",
                compute: |v| v[0] * v[1] + v[2],
            }],
        },
        FormulaEntry {
            name: "In-Amp Common-Mode Filter Cutoff",
            variants: &[
                SolveVariant {
                    solves_for: "f_cm",
                    expression: "f_cm = 1 / (2π × R_in × C_cm)",
                    inputs: &[
                        VarDef {
                            symbol: "R_in",
                            name: "Input resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "C_cm",
                            name: "Common-mode cap",
                            unit: "F",
                            default: 1e-9,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| 1.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
                SolveVariant {
                    solves_for: "R_in",
                    expression: "R_in = 1 / (2π × f_cm × C_cm)",
                    inputs: &[
                        VarDef {
                            symbol: "f_cm",
                            name: "Cutoff freq",
                            unit: "Hz",
                            default: 159.15e3,
                        },
                        VarDef {
                            symbol: "C_cm",
                            name: "Common-mode cap",
                            unit: "F",
                            default: 1e-9,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| 1.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
                SolveVariant {
                    solves_for: "C_cm",
                    expression: "C_cm = 1 / (2π × R_in × f_cm)",
                    inputs: &[
                        VarDef {
                            symbol: "R_in",
                            name: "Input resistor",
                            unit: "Ω",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "f_cm",
                            name: "Cutoff freq",
                            unit: "Hz",
                            default: 159.15e3,
                        },
                    ],
                    output_unit: "F",
                    compute: |v| 1.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
            ],
        },
        FormulaEntry {
            name: "In-Amp Differential Filter Cutoff",
            variants: &[SolveVariant {
                solves_for: "f_diff",
                expression: "f_diff = 1 / (2π × 2R_in × (C_diff + C_cm/2))",
                inputs: &[
                    VarDef {
                        symbol: "R_in",
                        name: "Input resistor",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "C_cm",
                        name: "Common-mode cap",
                        unit: "F",
                        default: 1e-9,
                    },
                    VarDef {
                        symbol: "C_diff",
                        name: "Differential cap",
                        unit: "F",
                        default: 10e-9,
                    },
                ],
                output_unit: "Hz",
                compute: |v| 1.0 / (2.0 * std::f64::consts::PI * 2.0 * v[0] * (v[2] + 0.5 * v[1])),
            }],
        },
    ]
}
