use super::{FormulaEntry, SolveVariant, VarDef};

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        // ── Ohm's Law ─────────────────────────────────────────────────────
        FormulaEntry {
            name: "Ohm's Law",
            variants: &[
                SolveVariant {
                    solves_for: "V",
                    expression: "V = I × R",
                    inputs: &[
                        VarDef {
                            symbol: "I",
                            name: "Current",
                            unit: "A",
                            default: 0.01,
                        },
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] * v[1],
                },
                SolveVariant {
                    solves_for: "I",
                    expression: "I = V / R",
                    inputs: &[
                        VarDef {
                            symbol: "V",
                            name: "Voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "A",
                    compute: |v| v[0] / v[1],
                },
                SolveVariant {
                    solves_for: "R",
                    expression: "R = V / I",
                    inputs: &[
                        VarDef {
                            symbol: "V",
                            name: "Voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "I",
                            name: "Current",
                            unit: "A",
                            default: 0.01,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[0] / v[1],
                },
            ],
        },
        // ── Voltage Divider ───────────────────────────────────────────────
        FormulaEntry {
            name: "Voltage Divider",
            variants: &[
                SolveVariant {
                    solves_for: "V_out",
                    expression: "V_out = V_in × R₂ / (R₁ + R₂)",
                    inputs: &[
                        VarDef {
                            symbol: "V_in",
                            name: "Input voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "R1",
                            name: "Top resistor",
                            unit: "Ω",
                            default: 10000.0,
                        },
                        VarDef {
                            symbol: "R2",
                            name: "Bottom resistor",
                            unit: "Ω",
                            default: 10000.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] * v[2] / (v[1] + v[2]),
                },
                SolveVariant {
                    solves_for: "R1",
                    expression: "R1 = R2 × (V_in / V_out − 1)",
                    inputs: &[
                        VarDef {
                            symbol: "V_in",
                            name: "Input voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "V_out",
                            name: "Output voltage",
                            unit: "V",
                            default: 2.5,
                        },
                        VarDef {
                            symbol: "R2",
                            name: "Bottom resistor",
                            unit: "Ω",
                            default: 10000.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[2] * (v[0] / v[1] - 1.0),
                },
                SolveVariant {
                    solves_for: "R2",
                    expression: "R2 = V_out × R1 / (V_in − V_out)",
                    inputs: &[
                        VarDef {
                            symbol: "V_in",
                            name: "Input voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "V_out",
                            name: "Output voltage",
                            unit: "V",
                            default: 2.5,
                        },
                        VarDef {
                            symbol: "R1",
                            name: "Top resistor",
                            unit: "Ω",
                            default: 10000.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[1] * v[2] / (v[0] - v[1]),
                },
            ],
        },
        // ── Resistor combinations ─────────────────────────────────────────
        FormulaEntry {
            name: "Parallel Resistance",
            variants: &[SolveVariant {
                solves_for: "R_eq",
                expression: "R_eq = (R₁ × R₂) / (R₁ + R₂)",
                inputs: &[
                    VarDef {
                        symbol: "R1",
                        name: "Resistor 1",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "R2",
                        name: "Resistor 2",
                        unit: "Ω",
                        default: 1000.0,
                    },
                ],
                output_unit: "Ω",
                compute: |v| (v[0] * v[1]) / (v[0] + v[1]),
            }],
        },
        FormulaEntry {
            name: "Series Resistance",
            variants: &[SolveVariant {
                solves_for: "R_eq",
                expression: "R_eq = R₁ + R₂",
                inputs: &[
                    VarDef {
                        symbol: "R1",
                        name: "Resistor 1",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "R2",
                        name: "Resistor 2",
                        unit: "Ω",
                        default: 1000.0,
                    },
                ],
                output_unit: "Ω",
                compute: |v| v[0] + v[1],
            }],
        },
        // ── Impedance ─────────────────────────────────────────────────────
        FormulaEntry {
            name: "Inductor Impedance",
            variants: &[
                SolveVariant {
                    solves_for: "|Z_L|",
                    expression: "|Z_L| = 2π × f × L",
                    inputs: &[
                        VarDef {
                            symbol: "f",
                            name: "Frequency",
                            unit: "Hz",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "L",
                            name: "Inductance",
                            unit: "H",
                            default: 1e-3,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| 2.0 * std::f64::consts::PI * v[0] * v[1],
                },
                SolveVariant {
                    solves_for: "L",
                    expression: "L = |Z_L| / (2π × f)",
                    inputs: &[
                        VarDef {
                            symbol: "|Z_L|",
                            name: "Impedance",
                            unit: "Ω",
                            default: 6.283,
                        },
                        VarDef {
                            symbol: "f",
                            name: "Frequency",
                            unit: "Hz",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "H",
                    compute: |v| v[0] / (2.0 * std::f64::consts::PI * v[1]),
                },
                SolveVariant {
                    solves_for: "f",
                    expression: "f = |Z_L| / (2π × L)",
                    inputs: &[
                        VarDef {
                            symbol: "|Z_L|",
                            name: "Impedance",
                            unit: "Ω",
                            default: 6.283,
                        },
                        VarDef {
                            symbol: "L",
                            name: "Inductance",
                            unit: "H",
                            default: 1e-3,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| v[0] / (2.0 * std::f64::consts::PI * v[1]),
                },
            ],
        },
        FormulaEntry {
            name: "Capacitor Impedance",
            variants: &[
                SolveVariant {
                    solves_for: "|Z_C|",
                    expression: "|Z_C| = 1 / (2π × f × C)",
                    inputs: &[
                        VarDef {
                            symbol: "f",
                            name: "Frequency",
                            unit: "Hz",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "C",
                            name: "Capacitance",
                            unit: "F",
                            default: 1e-6,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| 1.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
                SolveVariant {
                    solves_for: "C",
                    expression: "C = 1 / (|Z_C| × 2π × f)",
                    inputs: &[
                        VarDef {
                            symbol: "|Z_C|",
                            name: "Impedance",
                            unit: "Ω",
                            default: 159.15,
                        },
                        VarDef {
                            symbol: "f",
                            name: "Frequency",
                            unit: "Hz",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "F",
                    compute: |v| 1.0 / (v[0] * 2.0 * std::f64::consts::PI * v[1]),
                },
                SolveVariant {
                    solves_for: "f",
                    expression: "f = 1 / (|Z_C| × 2π × C)",
                    inputs: &[
                        VarDef {
                            symbol: "|Z_C|",
                            name: "Impedance",
                            unit: "Ω",
                            default: 159.15,
                        },
                        VarDef {
                            symbol: "C",
                            name: "Capacitance",
                            unit: "F",
                            default: 1e-6,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| 1.0 / (v[0] * 2.0 * std::f64::consts::PI * v[1]),
                },
            ],
        },
        // ── RC / LC Filters ───────────────────────────────────────────────
        FormulaEntry {
            name: "RC Filter Corner Frequency",
            variants: &[
                SolveVariant {
                    solves_for: "f_c",
                    expression: "f_c = 1 / (2π × R × C)",
                    inputs: &[
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "C",
                            name: "Capacitance",
                            unit: "F",
                            default: 1e-6,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| 1.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
                SolveVariant {
                    solves_for: "R",
                    expression: "R = 1 / (2π × f_c × C)",
                    inputs: &[
                        VarDef {
                            symbol: "f_c",
                            name: "Corner freq",
                            unit: "Hz",
                            default: 159.15,
                        },
                        VarDef {
                            symbol: "C",
                            name: "Capacitance",
                            unit: "F",
                            default: 1e-6,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| 1.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
                SolveVariant {
                    solves_for: "C",
                    expression: "C = 1 / (2π × f_c × R)",
                    inputs: &[
                        VarDef {
                            symbol: "f_c",
                            name: "Corner freq",
                            unit: "Hz",
                            default: 159.15,
                        },
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "F",
                    compute: |v| 1.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
            ],
        },
        FormulaEntry {
            name: "LC Filter Corner Frequency",
            variants: &[
                SolveVariant {
                    solves_for: "f_c",
                    expression: "f_c = 1 / (2π × √(L × C))",
                    inputs: &[
                        VarDef {
                            symbol: "L",
                            name: "Inductance",
                            unit: "H",
                            default: 1e-3,
                        },
                        VarDef {
                            symbol: "C",
                            name: "Capacitance",
                            unit: "F",
                            default: 1e-6,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| 1.0 / (2.0 * std::f64::consts::PI * (v[0] * v[1]).sqrt()),
                },
                SolveVariant {
                    solves_for: "C",
                    expression: "C = 1 / ((2π × f_c)² × L)",
                    inputs: &[
                        VarDef {
                            symbol: "f_c",
                            name: "Corner freq",
                            unit: "Hz",
                            default: 5032.92,
                        },
                        VarDef {
                            symbol: "L",
                            name: "Inductance",
                            unit: "H",
                            default: 1e-3,
                        },
                    ],
                    output_unit: "F",
                    compute: |v| {
                        let two_pi_f = 2.0 * std::f64::consts::PI * v[0];
                        1.0 / (two_pi_f * two_pi_f * v[1])
                    },
                },
                SolveVariant {
                    solves_for: "L",
                    expression: "L = 1 / ((2π × f_c)² × C)",
                    inputs: &[
                        VarDef {
                            symbol: "f_c",
                            name: "Corner freq",
                            unit: "Hz",
                            default: 5032.92,
                        },
                        VarDef {
                            symbol: "C",
                            name: "Capacitance",
                            unit: "F",
                            default: 1e-6,
                        },
                    ],
                    output_unit: "H",
                    compute: |v| {
                        let two_pi_f = 2.0 * std::f64::consts::PI * v[0];
                        1.0 / (two_pi_f * two_pi_f * v[1])
                    },
                },
            ],
        },
        // ── Power ─────────────────────────────────────────────────────────
        FormulaEntry {
            name: "Power P = V × I",
            variants: &[
                SolveVariant {
                    solves_for: "P",
                    expression: "P = V × I",
                    inputs: &[
                        VarDef {
                            symbol: "V",
                            name: "Voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "I",
                            name: "Current",
                            unit: "A",
                            default: 0.1,
                        },
                    ],
                    output_unit: "W",
                    compute: |v| v[0] * v[1],
                },
                SolveVariant {
                    solves_for: "V",
                    expression: "V = P / I",
                    inputs: &[
                        VarDef {
                            symbol: "P",
                            name: "Power",
                            unit: "W",
                            default: 0.5,
                        },
                        VarDef {
                            symbol: "I",
                            name: "Current",
                            unit: "A",
                            default: 0.1,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| v[0] / v[1],
                },
                SolveVariant {
                    solves_for: "I",
                    expression: "I = P / V",
                    inputs: &[
                        VarDef {
                            symbol: "P",
                            name: "Power",
                            unit: "W",
                            default: 0.5,
                        },
                        VarDef {
                            symbol: "V",
                            name: "Voltage",
                            unit: "V",
                            default: 5.0,
                        },
                    ],
                    output_unit: "A",
                    compute: |v| v[0] / v[1],
                },
            ],
        },
        FormulaEntry {
            name: "Power P = V² / R",
            variants: &[
                SolveVariant {
                    solves_for: "P",
                    expression: "P = V² / R",
                    inputs: &[
                        VarDef {
                            symbol: "V",
                            name: "Voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 50.0,
                        },
                    ],
                    output_unit: "W",
                    compute: |v| v[0] * v[0] / v[1],
                },
                SolveVariant {
                    solves_for: "V",
                    expression: "V = √(P × R)",
                    inputs: &[
                        VarDef {
                            symbol: "P",
                            name: "Power",
                            unit: "W",
                            default: 0.5,
                        },
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 50.0,
                        },
                    ],
                    output_unit: "V",
                    compute: |v| (v[0] * v[1]).sqrt(),
                },
                SolveVariant {
                    solves_for: "R",
                    expression: "R = V² / P",
                    inputs: &[
                        VarDef {
                            symbol: "V",
                            name: "Voltage",
                            unit: "V",
                            default: 5.0,
                        },
                        VarDef {
                            symbol: "P",
                            name: "Power",
                            unit: "W",
                            default: 0.5,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[0] * v[0] / v[1],
                },
            ],
        },
        FormulaEntry {
            name: "Power P = I² × R",
            variants: &[
                SolveVariant {
                    solves_for: "P",
                    expression: "P = I² × R",
                    inputs: &[
                        VarDef {
                            symbol: "I",
                            name: "Current",
                            unit: "A",
                            default: 0.1,
                        },
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 50.0,
                        },
                    ],
                    output_unit: "W",
                    compute: |v| v[0] * v[0] * v[1],
                },
                SolveVariant {
                    solves_for: "I",
                    expression: "I = √(P / R)",
                    inputs: &[
                        VarDef {
                            symbol: "P",
                            name: "Power",
                            unit: "W",
                            default: 0.5,
                        },
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 50.0,
                        },
                    ],
                    output_unit: "A",
                    compute: |v| (v[0] / v[1]).sqrt(),
                },
                SolveVariant {
                    solves_for: "R",
                    expression: "R = P / I²",
                    inputs: &[
                        VarDef {
                            symbol: "P",
                            name: "Power",
                            unit: "W",
                            default: 0.5,
                        },
                        VarDef {
                            symbol: "I",
                            name: "Current",
                            unit: "A",
                            default: 0.1,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[0] / (v[1] * v[1]),
                },
            ],
        },
        // ── RC Time Constant ──────────────────────────────────────────────
        FormulaEntry {
            name: "RC Time Constant",
            variants: &[
                SolveVariant {
                    solves_for: "τ",
                    expression: "τ = R × C",
                    inputs: &[
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "C",
                            name: "Capacitance",
                            unit: "F",
                            default: 1e-6,
                        },
                    ],
                    output_unit: "s",
                    compute: |v| v[0] * v[1],
                },
                SolveVariant {
                    solves_for: "R",
                    expression: "R = τ / C",
                    inputs: &[
                        VarDef {
                            symbol: "τ",
                            name: "Time constant",
                            unit: "s",
                            default: 1e-3,
                        },
                        VarDef {
                            symbol: "C",
                            name: "Capacitance",
                            unit: "F",
                            default: 1e-6,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[0] / v[1],
                },
                SolveVariant {
                    solves_for: "C",
                    expression: "C = τ / R",
                    inputs: &[
                        VarDef {
                            symbol: "τ",
                            name: "Time constant",
                            unit: "s",
                            default: 1e-3,
                        },
                        VarDef {
                            symbol: "R",
                            name: "Resistance",
                            unit: "Ω",
                            default: 1000.0,
                        },
                    ],
                    output_unit: "F",
                    compute: |v| v[0] / v[1],
                },
            ],
        },
        FormulaEntry {
            name: "RC Charging Voltage",
            variants: &[SolveVariant {
                solves_for: "V_C",
                expression: "V_C = V_s × (1 − e^(−t/RC))",
                inputs: &[
                    VarDef {
                        symbol: "V_s",
                        name: "Source voltage",
                        unit: "V",
                        default: 5.0,
                    },
                    VarDef {
                        symbol: "t",
                        name: "Time",
                        unit: "s",
                        default: 1e-3,
                    },
                    VarDef {
                        symbol: "R",
                        name: "Resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "C",
                        name: "Capacitance",
                        unit: "F",
                        default: 1e-6,
                    },
                ],
                output_unit: "V",
                compute: |v| v[0] * (1.0 - (-v[1] / (v[2] * v[3])).exp()),
            }],
        },
        FormulaEntry {
            name: "RC Discharging Voltage",
            variants: &[SolveVariant {
                solves_for: "V_C",
                expression: "V_C = V₀ × e^(−t/RC)",
                inputs: &[
                    VarDef {
                        symbol: "V0",
                        name: "Initial voltage",
                        unit: "V",
                        default: 5.0,
                    },
                    VarDef {
                        symbol: "t",
                        name: "Time",
                        unit: "s",
                        default: 1e-3,
                    },
                    VarDef {
                        symbol: "R",
                        name: "Resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "C",
                        name: "Capacitance",
                        unit: "F",
                        default: 1e-6,
                    },
                ],
                output_unit: "V",
                compute: |v| v[0] * (-v[1] / (v[2] * v[3])).exp(),
            }],
        },
        // ── Snubber ───────────────────────────────────────────────────────
        FormulaEntry {
            name: "Snubber Capacitor",
            variants: &[
                SolveVariant {
                    solves_for: "C_snub",
                    expression: "C_snub = 3 / (2π × f_osc × R_snub)",
                    inputs: &[
                        VarDef {
                            symbol: "f_osc",
                            name: "Oscillation freq",
                            unit: "Hz",
                            default: 100e3,
                        },
                        VarDef {
                            symbol: "R_snub",
                            name: "Snubber resistor",
                            unit: "Ω",
                            default: 10.0,
                        },
                    ],
                    output_unit: "F",
                    compute: |v| 3.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
                SolveVariant {
                    solves_for: "f_osc",
                    expression: "f_osc = 3 / (2π × R_snub × C_snub)",
                    inputs: &[
                        VarDef {
                            symbol: "R_snub",
                            name: "Snubber resistor",
                            unit: "Ω",
                            default: 10.0,
                        },
                        VarDef {
                            symbol: "C_snub",
                            name: "Snubber capacitor",
                            unit: "F",
                            default: 47.75e-9,
                        },
                    ],
                    output_unit: "Hz",
                    compute: |v| 3.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
                SolveVariant {
                    solves_for: "R_snub",
                    expression: "R_snub = 3 / (2π × f_osc × C_snub)",
                    inputs: &[
                        VarDef {
                            symbol: "f_osc",
                            name: "Oscillation freq",
                            unit: "Hz",
                            default: 100e3,
                        },
                        VarDef {
                            symbol: "C_snub",
                            name: "Snubber capacitor",
                            unit: "F",
                            default: 47.75e-9,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| 3.0 / (2.0 * std::f64::consts::PI * v[0] * v[1]),
                },
            ],
        },
    ]
}
