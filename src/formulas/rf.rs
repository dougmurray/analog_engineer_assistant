use super::{FormulaEntry, SolveVariant, VarDef};

const PI: f64 = std::f64::consts::PI;

/// Shared L-network Q calculation: Q = sqrt(max(R_p, R_s) / min(R_p, R_s) - 1)
fn l_network_q(r_s: f64, r_p: f64) -> f64 {
    let lo = r_s.min(r_p);
    let hi = r_s.max(r_p);
    ((hi / lo) - 1.0).sqrt()
}

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        FormulaEntry {
            name: "L-Network Matching — Q",
            note: None,
            variants: &[SolveVariant {
                solves_for: "Q",
                expression: "Q = sqrt(max(R_p, R_s) / min(R_p, R_s) - 1)",
                inputs: &[
                    VarDef {
                        symbol: "R_s",
                        name: "Source resistance",
                        unit: "Ω",
                        default: 50.0,
                    },
                    VarDef {
                        symbol: "R_p",
                        name: "Load resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                ],
                output_unit: "",
                compute: |v| l_network_q(v[0], v[1]),
            }],
        },
        FormulaEntry {
            name: "L-Network Matching — Series Reactance",
            note: None,
            variants: &[SolveVariant {
                solves_for: "X_s",
                expression: "X_s = Q × min(R_p, R_s)",
                inputs: &[
                    VarDef {
                        symbol: "R_s",
                        name: "Source resistance",
                        unit: "Ω",
                        default: 50.0,
                    },
                    VarDef {
                        symbol: "R_p",
                        name: "Load resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                ],
                output_unit: "Ω",
                compute: |v| l_network_q(v[0], v[1]) * v[0].min(v[1]),
            }],
        },
        FormulaEntry {
            name: "L-Network Matching — Parallel Reactance",
            note: None,
            variants: &[SolveVariant {
                solves_for: "X_p",
                expression: "X_p = max(R_p, R_s) / Q",
                inputs: &[
                    VarDef {
                        symbol: "R_s",
                        name: "Source resistance",
                        unit: "Ω",
                        default: 50.0,
                    },
                    VarDef {
                        symbol: "R_p",
                        name: "Load resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                ],
                output_unit: "Ω",
                compute: |v| v[0].max(v[1]) / l_network_q(v[0], v[1]),
            }],
        },
        FormulaEntry {
            name: "L-Network Matching — High-Pass Series Capacitor",
            note: None,
            variants: &[SolveVariant {
                solves_for: "C_s",
                expression: "C_s = 1 / (2π × freq × X_s)",
                inputs: &[
                    VarDef {
                        symbol: "R_s",
                        name: "Source resistance",
                        unit: "Ω",
                        default: 50.0,
                    },
                    VarDef {
                        symbol: "R_p",
                        name: "Load resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "freq",
                        name: "Frequency",
                        unit: "Hz",
                        default: 100e6,
                    },
                ],
                output_unit: "F",
                compute: |v| {
                    let x_s = l_network_q(v[0], v[1]) * v[0].min(v[1]);
                    1.0 / (2.0 * PI * v[2] * x_s)
                },
            }],
        },
        FormulaEntry {
            name: "L-Network Matching — High-Pass Parallel Inductor",
            note: None,
            variants: &[SolveVariant {
                solves_for: "L_p",
                expression: "L_p = X_p / (2π × freq)",
                inputs: &[
                    VarDef {
                        symbol: "R_s",
                        name: "Source resistance",
                        unit: "Ω",
                        default: 50.0,
                    },
                    VarDef {
                        symbol: "R_p",
                        name: "Load resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "freq",
                        name: "Frequency",
                        unit: "Hz",
                        default: 100e6,
                    },
                ],
                output_unit: "H",
                compute: |v| {
                    let x_p = v[0].max(v[1]) / l_network_q(v[0], v[1]);
                    x_p / (2.0 * PI * v[2])
                },
            }],
        },
        FormulaEntry {
            name: "L-Network Matching — Low-Pass Series Inductor",
            note: None,
            variants: &[SolveVariant {
                solves_for: "L_s",
                expression: "L_s = X_s / (2π × freq)",
                inputs: &[
                    VarDef {
                        symbol: "R_s",
                        name: "Source resistance",
                        unit: "Ω",
                        default: 50.0,
                    },
                    VarDef {
                        symbol: "R_p",
                        name: "Load resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "freq",
                        name: "Frequency",
                        unit: "Hz",
                        default: 100e6,
                    },
                ],
                output_unit: "H",
                compute: |v| {
                    let x_s = l_network_q(v[0], v[1]) * v[0].min(v[1]);
                    x_s / (2.0 * PI * v[2])
                },
            }],
        },
        FormulaEntry {
            name: "L-Network Matching — Low-Pass Parallel Capacitor",
            note: None,
            variants: &[SolveVariant {
                solves_for: "C_p",
                expression: "C_p = 1 / (2π × freq × X_p)",
                inputs: &[
                    VarDef {
                        symbol: "R_s",
                        name: "Source resistance",
                        unit: "Ω",
                        default: 50.0,
                    },
                    VarDef {
                        symbol: "R_p",
                        name: "Load resistance",
                        unit: "Ω",
                        default: 1000.0,
                    },
                    VarDef {
                        symbol: "freq",
                        name: "Frequency",
                        unit: "Hz",
                        default: 100e6,
                    },
                ],
                output_unit: "F",
                compute: |v| {
                    let x_p = v[0].max(v[1]) / l_network_q(v[0], v[1]);
                    1.0 / (2.0 * PI * v[2] * x_p)
                },
            }],
        },
        FormulaEntry {
            name: "Pi-Network Matching",
            note: Some(
                "Use this network between high-value impedances (> 50 Ω). R is the virtual resistance, smaller than R_p and R_s.",
            ),
            variants: &[
                SolveVariant {
                    solves_for: "Q",
                    expression: "Q = sqrt(max(R_p, R_s) / R - 1)",
                    inputs: &[
                        VarDef {
                            symbol: "R_p",
                            name: "Load resistance",
                            unit: "Ω",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "R_s",
                            name: "Source resistance",
                            unit: "Ω",
                            default: 50.0,
                        },
                        VarDef {
                            symbol: "R",
                            name: "Virtual resistance",
                            unit: "Ω",
                            default: 10.0,
                        },
                    ],
                    output_unit: "",
                    compute: |v| (v[0].max(v[1]) / v[2] - 1.0).sqrt(),
                },
                SolveVariant {
                    solves_for: "R",
                    expression: "R = max(R_p, R_s) / (Q^2 + 1)",
                    inputs: &[
                        VarDef {
                            symbol: "R_p",
                            name: "Load resistance",
                            unit: "Ω",
                            default: 1000.0,
                        },
                        VarDef {
                            symbol: "R_s",
                            name: "Source resistance",
                            unit: "Ω",
                            default: 50.0,
                        },
                        VarDef {
                            symbol: "Q",
                            name: "Loaded Q",
                            unit: "",
                            default: 3.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[0].max(v[1]) / (v[2] * v[2] + 1.0),
                },
            ],
        },
        FormulaEntry {
            name: "T-Network Matching",
            note: Some(
                "Use this network between low-value impedances (< 50 Ω). R is the virtual resistance, larger than R_p and R_s.",
            ),
            variants: &[
                SolveVariant {
                    solves_for: "Q",
                    expression: "Q = sqrt(R / min(R_p, R_s) - 1)",
                    inputs: &[
                        VarDef {
                            symbol: "R",
                            name: "Virtual resistance",
                            unit: "Ω",
                            default: 200.0,
                        },
                        VarDef {
                            symbol: "R_p",
                            name: "Load resistance",
                            unit: "Ω",
                            default: 50.0,
                        },
                        VarDef {
                            symbol: "R_s",
                            name: "Source resistance",
                            unit: "Ω",
                            default: 10.0,
                        },
                    ],
                    output_unit: "",
                    compute: |v| (v[0] / v[1].min(v[2]) - 1.0).sqrt(),
                },
                SolveVariant {
                    solves_for: "R",
                    expression: "R = min(R_p, R_s) × (Q^2 + 1)",
                    inputs: &[
                        VarDef {
                            symbol: "Q",
                            name: "Loaded Q",
                            unit: "",
                            default: 3.0,
                        },
                        VarDef {
                            symbol: "R_p",
                            name: "Load resistance",
                            unit: "Ω",
                            default: 50.0,
                        },
                        VarDef {
                            symbol: "R_s",
                            name: "Source resistance",
                            unit: "Ω",
                            default: 10.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[1].min(v[2]) * (v[0] * v[0] + 1.0),
                },
            ],
        },
        FormulaEntry {
            name: "Transformer Matching",
            note: Some(
                "Z_p represents the primary impedance, designed to be the same as the source (R_s). The Z_s is the secondary impedance, designed to be the same as the load (R_l).",
            ),
            variants: &[
                SolveVariant {
                    solves_for: "Z_s",
                    expression: "Z_s = Z_p × (N_s / N_p)^2",
                    inputs: &[
                        VarDef {
                            symbol: "Z_p",
                            name: "Primary impedance",
                            unit: "Ω",
                            default: 50.0,
                        },
                        VarDef {
                            symbol: "N_s",
                            name: "Secondary turns",
                            unit: "",
                            default: 2.0,
                        },
                        VarDef {
                            symbol: "N_p",
                            name: "Primary turns",
                            unit: "",
                            default: 1.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[0] * (v[1] / v[2]).powi(2),
                },
                SolveVariant {
                    solves_for: "Z_p",
                    expression: "Z_p = Z_s / (N_s / N_p)^2",
                    inputs: &[
                        VarDef {
                            symbol: "Z_s",
                            name: "Secondary impedance",
                            unit: "Ω",
                            default: 200.0,
                        },
                        VarDef {
                            symbol: "N_s",
                            name: "Secondary turns",
                            unit: "",
                            default: 2.0,
                        },
                        VarDef {
                            symbol: "N_p",
                            name: "Primary turns",
                            unit: "",
                            default: 1.0,
                        },
                    ],
                    output_unit: "Ω",
                    compute: |v| v[0] / (v[1] / v[2]).powi(2),
                },
                SolveVariant {
                    solves_for: "N_p",
                    expression: "N_p = N_s / sqrt(Z_s / Z_p)",
                    inputs: &[
                        VarDef {
                            symbol: "N_s",
                            name: "Secondary turns",
                            unit: "",
                            default: 2.0,
                        },
                        VarDef {
                            symbol: "Z_s",
                            name: "Secondary impedance",
                            unit: "Ω",
                            default: 200.0,
                        },
                        VarDef {
                            symbol: "Z_p",
                            name: "Primary impedance",
                            unit: "Ω",
                            default: 50.0,
                        },
                    ],
                    output_unit: "",
                    compute: |v| v[0] / (v[1] / v[2]).sqrt(),
                },
                SolveVariant {
                    solves_for: "N_s",
                    expression: "N_s = N_p × sqrt(Z_s / Z_p)",
                    inputs: &[
                        VarDef {
                            symbol: "N_p",
                            name: "Primary turns",
                            unit: "",
                            default: 1.0,
                        },
                        VarDef {
                            symbol: "Z_s",
                            name: "Secondary impedance",
                            unit: "Ω",
                            default: 200.0,
                        },
                        VarDef {
                            symbol: "Z_p",
                            name: "Primary impedance",
                            unit: "Ω",
                            default: 50.0,
                        },
                    ],
                    output_unit: "",
                    compute: |v| v[0] * (v[1] / v[2]).sqrt(),
                },
            ],
        },
        FormulaEntry {
            name: "λ/4 Q-Section Transmission Line Matching",
            note: Some(
                "Z_o is the characteristic impedance of the input transmission line from the source (R_s). Z_l is the load impedance.",
            ),
            variants: &[SolveVariant {
                solves_for: "Z_q",
                expression: "Z_q = sqrt(Z_o × Z_l)",
                inputs: &[
                    VarDef {
                        symbol: "Z_o",
                        name: "Source line impedance",
                        unit: "Ω",
                        default: 50.0,
                    },
                    VarDef {
                        symbol: "Z_l",
                        name: "Load impedance",
                        unit: "Ω",
                        default: 200.0,
                    },
                ],
                output_unit: "Ω",
                compute: |v| (v[0] * v[1]).sqrt(),
            }],
        },
    ]
}
