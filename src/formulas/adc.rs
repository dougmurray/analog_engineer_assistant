use super::{FormulaEntry, SolveVariant, VarDef};

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        FormulaEntry {
            name: "Full-Scale Range",
            note: None,
            variants: &[
                SolveVariant {
                    solves_for: "FSR",
                    expression: "FSR = V_REF / PGA",
                    inputs: &[
                        VarDef {
                            symbol: "V_REF",
                            name: "Reference voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "PGA",
                            name: "PGA gain",
                            unit: "V/V",
                            default: 1.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] / v[1],
                },
                SolveVariant {
                    solves_for: "V_REF",
                    expression: "V_REF = PGA × FSR",
                    inputs: &[
                        VarDef {
                            symbol: "PGA",
                            name: "PGA gain",
                            unit: "V/V",
                            default: 1.0,
                        },
                        VarDef {
                            symbol: "FSR",
                            name: "Full-scale range",
                            unit: "V",
                            default: 5.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] * v[1],
                },
                SolveVariant {
                    solves_for: "PGA",
                    expression: "PGA = V_REF / FSR",
                    inputs: &[
                        VarDef {
                            symbol: "V_REF",
                            name: "Reference voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "FSR",
                            name: "Full-scale range",
                            unit: "V",
                            default: 5.0,
                        },
                    ],
                    output_unit: "V/V",
                    compute: |v| v[0] / v[1],
                },
            ],
        },
        FormulaEntry {
            name: "LSB Size",
            note: None,
            variants: &[
                SolveVariant {
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
                },
                SolveVariant {
                    solves_for: "FSR",
                    expression: "FSR = 2ⁿ × LSB",
                    inputs: &[
                        VarDef {
                            symbol: "LSB",
                            name: "LSB size",
                            unit: "V",
                            default: 1.221e-3,
                        },
                        VarDef {
                            symbol: "n",
                            name: "Resolution",
                            unit: "bits",
                            default: 12.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] * 2f64.powi(v[1] as i32),
                },
            ],
        },
        FormulaEntry {
            name: "ADC Output Code",
            note: None,
            variants: &[SolveVariant {
                solves_for: "code",
                expression: "code = round(V_in × 2ⁿ / FSR)",
                inputs: &[
                    VarDef {
                        symbol: "V_in",
                        name: "Input voltage",
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
                compute: |v| {
                    let max_code = 2f64.powi(v[2] as i32) - 1.0;
                    (v[0] * 2f64.powi(v[2] as i32) / v[1]).round().min(max_code)
                },
            }],
        },
        FormulaEntry {
            name: "Max RMS Signal",
            note: None,
            variants: &[SolveVariant {
                solves_for: "V_rms_max",
                expression: "V_rms_max = FSR / (2√2)",
                inputs: &[VarDef {
                    symbol: "FSR",
                    name: "Full-scale range",
                    unit: "V",
                    default: 5.0,
                }],
                output_unit: "V",
                compute: |v| v[0] / (2.0 * 2f64.sqrt()),
            }],
        },
        FormulaEntry {
            name: "RMS Quantization Noise",
            note: None,
            variants: &[SolveVariant {
                solves_for: "V_noise",
                expression: "V_noise = LSB / √12",
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
                compute: |v| (v[0] / 2f64.powi(v[1] as i32)) / 12f64.sqrt(),
            }],
        },
        FormulaEntry {
            name: "SNR (Ideal, Quantization Only)",
            note: None,
            variants: &[
                SolveVariant {
                    solves_for: "SNR",
                    expression: "SNR(dB) = 6.02 × N + 1.76",
                    inputs: &[VarDef {
                        symbol: "N",
                        name: "Resolution",
                        unit: "bits",
                        default: 12.0,
                    }],
                    output_unit: "dB",
                    compute: |v| 6.02 * v[0] + 1.76,
                },
                SolveVariant {
                    solves_for: "N",
                    expression: "N = (SNR(dB) − 1.76) / 6.02",
                    inputs: &[VarDef {
                        symbol: "SNR",
                        name: "SNR",
                        unit: "dB",
                        default: 74.0,
                    }],
                    output_unit: "bits",
                    compute: |v| (v[0] - 1.76) / 6.02,
                },
            ],
        },
        FormulaEntry {
            name: "THD (Percent)",
            note: None,
            variants: &[SolveVariant {
                solves_for: "THD%",
                expression: "THD% = (V_distortion / V_signal) × 100",
                inputs: &[
                    VarDef {
                        symbol: "V_dist",
                        name: "RMS distortion",
                        unit: "V",
                        default: 50e-6,
                    },
                    VarDef {
                        symbol: "V_sig",
                        name: "RMS signal",
                        unit: "V",
                        default: 1.76,
                    },
                ],
                output_unit: "%",
                compute: |v| (v[0] / v[1]) * 100.0,
            }],
        },
        FormulaEntry {
            name: "THD (dB)",
            note: None,
            variants: &[SolveVariant {
                solves_for: "THD",
                expression: "THD(dB) = 20 × log(V_distortion / V_signal)",
                inputs: &[
                    VarDef {
                        symbol: "V_dist",
                        name: "RMS distortion",
                        unit: "V",
                        default: 50e-6,
                    },
                    VarDef {
                        symbol: "V_sig",
                        name: "RMS signal",
                        unit: "V",
                        default: 1.76,
                    },
                ],
                output_unit: "dB",
                compute: |v| 20.0 * (v[0] / v[1]).log10(),
            }],
        },
        FormulaEntry {
            name: "SINAD",
            note: None,
            variants: &[SolveVariant {
                solves_for: "SINAD",
                expression: "SINAD(dB) = 20·log(V_sig / √(V_noise² + V_dist²))",
                inputs: &[
                    VarDef {
                        symbol: "V_sig",
                        name: "RMS signal",
                        unit: "V",
                        default: 1.76,
                    },
                    VarDef {
                        symbol: "V_noise",
                        name: "RMS noise",
                        unit: "V",
                        default: 100e-6,
                    },
                    VarDef {
                        symbol: "V_dist",
                        name: "RMS distortion",
                        unit: "V",
                        default: 50e-6,
                    },
                ],
                output_unit: "dB",
                compute: |v| 20.0 * (v[0] / (v[1] * v[1] + v[2] * v[2]).sqrt()).log10(),
            }],
        },
        FormulaEntry {
            name: "ENOB",
            note: None,
            variants: &[
                SolveVariant {
                    solves_for: "ENOB",
                    expression: "ENOB = (SINAD(dB) − 1.76) / 6.02",
                    inputs: &[VarDef {
                        symbol: "SINAD",
                        name: "SINAD",
                        unit: "dB",
                        default: 83.9,
                    }],
                    output_unit: "bits",
                    compute: |v| (v[0] - 1.76) / 6.02,
                },
                SolveVariant {
                    solves_for: "SINAD",
                    expression: "SINAD(dB) = ENOB × 6.02 + 1.76",
                    inputs: &[VarDef {
                        symbol: "ENOB",
                        name: "ENOB",
                        unit: "bits",
                        default: 13.65,
                    }],
                    output_unit: "dB",
                    compute: |v| v[0] * 6.02 + 1.76,
                },
            ],
        },
        FormulaEntry {
            name: "Noise-Free Resolution",
            note: None,
            variants: &[SolveVariant {
                solves_for: "NFR",
                expression: "NFR = log₂(2ᴺ / PeaktoPeakNoise_LSB)",
                inputs: &[
                    VarDef {
                        symbol: "N",
                        name: "ADC resolution",
                        unit: "bits",
                        default: 24.0,
                    },
                    VarDef {
                        symbol: "pp_n",
                        name: "Peak-to-peak noise",
                        unit: "LSB",
                        default: 7.0,
                    },
                ],
                output_unit: "bits",
                compute: |v| (2f64.powi(v[0] as i32) / v[1]).log2(),
            }],
        },
        FormulaEntry {
            name: "Effective Resolution",
            note: None,
            variants: &[SolveVariant {
                solves_for: "ER",
                expression: "ER = log₂(2ᴺ / rmsNoise_LSB)",
                inputs: &[
                    VarDef {
                        symbol: "N",
                        name: "ADC resolution",
                        unit: "bits",
                        default: 24.0,
                    },
                    VarDef {
                        symbol: "rms_n",
                        name: "RMS noise",
                        unit: "LSB",
                        default: 1.06,
                    },
                ],
                output_unit: "bits",
                compute: |v| (2f64.powi(v[0] as i32) / v[1]).log2(),
            }],
        },
        FormulaEntry {
            name: "Settling Accuracy → Time Constants",
            note: None,
            variants: &[SolveVariant {
                solves_for: "N_TC",
                expression: "N_TC = ln(2ᴺ)",
                inputs: &[VarDef {
                    symbol: "N",
                    name: "Bits of accuracy",
                    unit: "bits",
                    default: 12.0,
                }],
                output_unit: "τ",
                compute: |v| (2f64.powi(v[0] as i32)).ln(),
            }],
        },
        FormulaEntry {
            name: "ADC Full-Scale RMS",
            note: None,
            variants: &[SolveVariant {
                solves_for: "V_FSR_RMS",
                expression: "V_FSR_RMS = V_FSR × 0.707 / 2",
                inputs: &[VarDef {
                    symbol: "V_FSR",
                    name: "Full-scale range",
                    unit: "V",
                    default: 5.0,
                }],
                output_unit: "V",
                compute: |v| v[0] * 0.707 / 2.0,
            }],
        },
        FormulaEntry {
            name: "ADC System Total Noise",
            note: None,
            variants: &[SolveVariant {
                solves_for: "V_nT",
                expression: "V_nT = √(V_nADC² + V_nAmp² + V_nRef²)",
                inputs: &[
                    VarDef {
                        symbol: "V_nADC",
                        name: "ADC noise (Vrms)",
                        unit: "V",
                        default: 100e-6,
                    },
                    VarDef {
                        symbol: "V_nAmp",
                        name: "Amp noise (Vrms)",
                        unit: "V",
                        default: 50e-6,
                    },
                    VarDef {
                        symbol: "V_nRef",
                        name: "Ref noise (Vrms)",
                        unit: "V",
                        default: 30e-6,
                    },
                ],
                output_unit: "V",
                compute: |v| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt(),
            }],
        },
        FormulaEntry {
            name: "Clock Jitter SNR",
            note: None,
            variants: &[SolveVariant {
                solves_for: "SNR",
                expression: "SNR = −20·log(2π × f_in × t_jitter)",
                inputs: &[
                    VarDef {
                        symbol: "f_in",
                        name: "Input frequency",
                        unit: "Hz",
                        default: 1e6,
                    },
                    VarDef {
                        symbol: "t_jitter",
                        name: "Clock jitter",
                        unit: "s",
                        default: 1e-12,
                    },
                ],
                output_unit: "dB",
                compute: |v| -20.0 * (2.0 * std::f64::consts::PI * v[0] * v[1]).log10(),
            }],
        },
        FormulaEntry {
            name: "Clock Jitter SNR with Oversampling",
            note: None,
            variants: &[SolveVariant {
                solves_for: "SNR",
                expression: "SNR = −20·log(2π×f_in×t_j) + 10·log(OSR)",
                inputs: &[
                    VarDef {
                        symbol: "f_in",
                        name: "Input frequency",
                        unit: "Hz",
                        default: 1e6,
                    },
                    VarDef {
                        symbol: "t_jitter",
                        name: "Clock jitter",
                        unit: "s",
                        default: 1e-12,
                    },
                    VarDef {
                        symbol: "OSR",
                        name: "Oversampling ratio",
                        unit: "",
                        default: 16.0,
                    },
                ],
                output_unit: "dB",
                compute: |v| {
                    -20.0 * (2.0 * std::f64::consts::PI * v[0] * v[1]).log10() + 10.0 * v[2].log10()
                },
            }],
        },
    ]
}
