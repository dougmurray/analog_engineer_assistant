use super::{FormulaEntry, SolveVariant, VarDef};

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        // ── Error ────────────────────────────────────────────────────────────
        FormulaEntry {
            name: "Error Percent",
            variants: &[SolveVariant {
                solves_for: "error%",
                expression: "error% = ((measured − ideal) / ideal) × 100",
                inputs: &[
                    VarDef { symbol: "measured", name: "Measured value", unit: "", default: 1.01 },
                    VarDef { symbol: "ideal",    name: "Ideal value",    unit: "", default: 1.0  },
                ],
                output_unit: "%",
                compute: |v| ((v[0] - v[1]) / v[1]) * 100.0,
            }],
        },
        FormulaEntry {
            name: "Error Percent of Full Scale",
            variants: &[SolveVariant {
                solves_for: "error%FS",
                expression: "error%FS = ((measured − ideal) / FS) × 100",
                inputs: &[
                    VarDef { symbol: "measured", name: "Measured value",   unit: "", default: 1.01 },
                    VarDef { symbol: "ideal",    name: "Ideal value",      unit: "", default: 1.0  },
                    VarDef { symbol: "FS",       name: "Full-scale range", unit: "", default: 5.0  },
                ],
                output_unit: "%",
                compute: |v| ((v[0] - v[1]) / v[2]) * 100.0,
            }],
        },
        // ── PPM / % / m% ──────────────────────────────────────────────────
        FormulaEntry {
            name: "PPM ↔ Percent",
            variants: &[
                SolveVariant {
                    solves_for: "%",
                    expression: "% = ppm / 10⁴",
                    inputs: &[VarDef { symbol: "ppm", name: "Parts per million", unit: "ppm", default: 100.0 }],
                    output_unit: "%",
                    compute: |v| v[0] / 1e4,
                },
                SolveVariant {
                    solves_for: "ppm",
                    expression: "ppm = % × 10⁴",
                    inputs: &[VarDef { symbol: "%", name: "Percent", unit: "%", default: 0.01 }],
                    output_unit: "ppm",
                    compute: |v| v[0] * 1e4,
                },
            ],
        },
        FormulaEntry {
            name: "PPM ↔ Milli-Percent",
            variants: &[
                SolveVariant {
                    solves_for: "m%",
                    expression: "m% = ppm / 10",
                    inputs: &[VarDef { symbol: "ppm", name: "Parts per million", unit: "ppm", default: 100.0 }],
                    output_unit: "m%",
                    compute: |v| v[0] / 10.0,
                },
                SolveVariant {
                    solves_for: "ppm",
                    expression: "ppm = m% × 10",
                    inputs: &[VarDef { symbol: "m%", name: "Milli-percent", unit: "m%", default: 10.0 }],
                    output_unit: "ppm",
                    compute: |v| v[0] * 10.0,
                },
            ],
        },
        // ── Code conversions ──────────────────────────────────────────────
        FormulaEntry {
            name: "Millivolts ↔ Codes",
            variants: &[
                SolveVariant {
                    solves_for: "codes",
                    expression: "codes = mV × (2ⁿ / (FS × 1000))",
                    inputs: &[
                        VarDef { symbol: "mV", name: "Millivolts",     unit: "mV",   default: 100.0 },
                        VarDef { symbol: "FS", name: "Full-scale (V)", unit: "V",    default: 5.0   },
                        VarDef { symbol: "n",  name: "Resolution",     unit: "bits", default: 12.0  },
                    ],
                    output_unit: "codes",
                    compute: |v| v[0] * (2f64.powi(v[2] as i32) / (v[1] * 1000.0)),
                },
                SolveVariant {
                    solves_for: "mV",
                    expression: "mV = codes × (FS / 2ⁿ) × 1000",
                    inputs: &[
                        VarDef { symbol: "codes", name: "Code value",     unit: "",     default: 819.0 },
                        VarDef { symbol: "FS",    name: "Full-scale (V)", unit: "V",    default: 5.0   },
                        VarDef { symbol: "n",     name: "Resolution",     unit: "bits", default: 12.0  },
                    ],
                    output_unit: "mV",
                    compute: |v| v[0] * (v[1] / 2f64.powi(v[2] as i32)) * 1000.0,
                },
            ],
        },
        FormulaEntry {
            name: "Percent ↔ Codes",
            variants: &[
                SolveVariant {
                    solves_for: "codes",
                    expression: "codes = % × (2ⁿ / 100)",
                    inputs: &[
                        VarDef { symbol: "%", name: "Percent of FS", unit: "%",    default: 50.0 },
                        VarDef { symbol: "n", name: "Resolution",   unit: "bits", default: 12.0 },
                    ],
                    output_unit: "codes",
                    compute: |v| v[0] * (2f64.powi(v[1] as i32) / 100.0),
                },
                SolveVariant {
                    solves_for: "%",
                    expression: "% = codes × (100 / 2ⁿ)",
                    inputs: &[
                        VarDef { symbol: "codes", name: "Code value",  unit: "",     default: 2048.0 },
                        VarDef { symbol: "n",     name: "Resolution",  unit: "bits", default: 12.0   },
                    ],
                    output_unit: "%",
                    compute: |v| v[0] * (100.0 / 2f64.powi(v[1] as i32)),
                },
            ],
        },
        FormulaEntry {
            name: "PPM ↔ Codes",
            variants: &[
                SolveVariant {
                    solves_for: "codes",
                    expression: "codes = ppm × (2ⁿ / 10⁶)",
                    inputs: &[
                        VarDef { symbol: "ppm", name: "Parts per million", unit: "ppm",  default: 100.0 },
                        VarDef { symbol: "n",   name: "Resolution",        unit: "bits", default: 12.0  },
                    ],
                    output_unit: "codes",
                    compute: |v| v[0] * (2f64.powi(v[1] as i32) / 1e6),
                },
                SolveVariant {
                    solves_for: "ppm",
                    expression: "ppm = codes × (10⁶ / 2ⁿ)",
                    inputs: &[
                        VarDef { symbol: "codes", name: "Code value",  unit: "",     default: 0.4096 },
                        VarDef { symbol: "n",     name: "Resolution",  unit: "bits", default: 12.0   },
                    ],
                    output_unit: "ppm",
                    compute: |v| v[0] * (1e6 / 2f64.powi(v[1] as i32)),
                },
            ],
        },
        // ── Signal (peak / RMS) ───────────────────────────────────────────
        FormulaEntry {
            name: "Peak-to-Peak ↔ RMS",
            variants: &[
                SolveVariant {
                    solves_for: "V_rms",
                    expression: "V_rms = V_pp / (2√2)",
                    inputs: &[VarDef { symbol: "V_pp", name: "Peak-to-peak", unit: "V", default: 2.0 }],
                    output_unit: "V",
                    compute: |v| v[0] / (2.0 * 2f64.sqrt()),
                },
                SolveVariant {
                    solves_for: "V_pp",
                    expression: "V_pp = V_rms × 2√2",
                    inputs: &[VarDef { symbol: "V_rms", name: "RMS voltage", unit: "V", default: 0.707 }],
                    output_unit: "V",
                    compute: |v| v[0] * 2.0 * 2f64.sqrt(),
                },
            ],
        },
        FormulaEntry {
            name: "Peak ↔ RMS",
            variants: &[
                SolveVariant {
                    solves_for: "V_rms",
                    expression: "V_rms = V_peak / √2",
                    inputs: &[VarDef { symbol: "V_peak", name: "Peak voltage", unit: "V", default: 1.0 }],
                    output_unit: "V",
                    compute: |v| v[0] / 2f64.sqrt(),
                },
                SolveVariant {
                    solves_for: "V_peak",
                    expression: "V_peak = V_rms × √2",
                    inputs: &[VarDef { symbol: "V_rms", name: "RMS voltage", unit: "V", default: 0.707 }],
                    output_unit: "V",
                    compute: |v| v[0] * 2f64.sqrt(),
                },
            ],
        },
        // ── Gain / dB ─────────────────────────────────────────────────────
        FormulaEntry {
            name: "Voltage Gain ↔ dB",
            variants: &[
                SolveVariant {
                    solves_for: "dB",
                    expression: "dB = 20 × log₁₀(V_out / V_in)",
                    inputs: &[VarDef { symbol: "gain", name: "Voltage gain (V_out/V_in)", unit: "V/V", default: 10.0 }],
                    output_unit: "dB",
                    compute: |v| 20.0 * v[0].log10(),
                },
                SolveVariant {
                    solves_for: "gain",
                    expression: "gain = 10^(dB / 20)",
                    inputs: &[VarDef { symbol: "dB", name: "Decibels", unit: "dB", default: 20.0 }],
                    output_unit: "V/V",
                    compute: |v| 10f64.powf(v[0] / 20.0),
                },
            ],
        },
        FormulaEntry {
            name: "Power Gain ↔ dB",
            variants: &[
                SolveVariant {
                    solves_for: "dB",
                    expression: "dB = 10 × log₁₀(P_out / P_in)",
                    inputs: &[VarDef { symbol: "gain", name: "Power gain (P_out/P_in)", unit: "W/W", default: 10.0 }],
                    output_unit: "dB",
                    compute: |v| 10.0 * v[0].log10(),
                },
                SolveVariant {
                    solves_for: "gain",
                    expression: "gain = 10^(dB / 10)",
                    inputs: &[VarDef { symbol: "dB", name: "Decibels", unit: "dB", default: 10.0 }],
                    output_unit: "W/W",
                    compute: |v| 10f64.powf(v[0] / 10.0),
                },
            ],
        },
        // ── Time / Phase ─────────────────────────────────────────────────
        FormulaEntry {
            name: "Time ↔ Phase",
            variants: &[
                SolveVariant {
                    solves_for: "phase",
                    expression: "phase° = (t_shift / period) × 360",
                    inputs: &[
                        VarDef { symbol: "t_shift", name: "Time shift", unit: "s", default: 1e-6 },
                        VarDef { symbol: "period",  name: "Period",     unit: "s", default: 1e-5 },
                    ],
                    output_unit: "°",
                    compute: |v| (v[0] / v[1]) * 360.0,
                },
                SolveVariant {
                    solves_for: "t_shift",
                    expression: "t_shift = (phase° / 360) × period",
                    inputs: &[
                        VarDef { symbol: "phase",  name: "Phase shift", unit: "°", default: 36.0  },
                        VarDef { symbol: "period", name: "Period",      unit: "s", default: 1e-5  },
                    ],
                    output_unit: "s",
                    compute: |v| (v[0] / 360.0) * v[1],
                },
            ],
        },
    ]
}
