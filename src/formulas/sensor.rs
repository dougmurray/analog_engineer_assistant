use super::{FormulaEntry, SolveVariant, VarDef};

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        FormulaEntry {
            name: "Thermistor R → T (Steinhart-Hart)",
            note: None,
            variants: &[SolveVariant {
                solves_for: "T",
                expression: "1/T = a + b·ln(R) + c·[ln(R)]³",
                inputs: &[
                    VarDef {
                        symbol: "R",
                        name: "Resistance",
                        unit: "Ω",
                        default: 10000.0,
                    },
                    VarDef {
                        symbol: "a",
                        name: "S-H coeff a",
                        unit: "K⁻¹",
                        default: 1.129e-3,
                    },
                    VarDef {
                        symbol: "b",
                        name: "S-H coeff b",
                        unit: "K⁻¹",
                        default: 2.341e-4,
                    },
                    VarDef {
                        symbol: "c",
                        name: "S-H coeff c",
                        unit: "K⁻¹",
                        default: 8.776e-8,
                    },
                ],
                output_unit: "°C",
                compute: |v| {
                    let ln_r = v[0].ln();
                    let t_k = 1.0 / (v[1] + v[2] * ln_r + v[3] * ln_r.powi(3));
                    t_k - 273.15
                },
            }],
        },
        FormulaEntry {
            name: "Thermistor T → R (Steinhart-Hart)",
            note: None,
            variants: &[SolveVariant {
                solves_for: "R",
                expression: "R = exp[(y − x/2)^(1/3) − (y + x/2)^(1/3)]",
                inputs: &[
                    VarDef {
                        symbol: "T",
                        name: "Temperature",
                        unit: "°C",
                        default: 25.0,
                    },
                    VarDef {
                        symbol: "a",
                        name: "S-H coeff a",
                        unit: "K⁻¹",
                        default: 1.129e-3,
                    },
                    VarDef {
                        symbol: "b",
                        name: "S-H coeff b",
                        unit: "K⁻¹",
                        default: 2.341e-4,
                    },
                    VarDef {
                        symbol: "c",
                        name: "S-H coeff c",
                        unit: "K⁻¹",
                        default: 8.776e-8,
                    },
                ],
                output_unit: "Ω",
                compute: |v| {
                    let t_k = v[0] + 273.15;
                    let x = (v[1] - 1.0 / t_k) / v[3];
                    let y = ((v[2] / (3.0 * v[3])).powi(3) + x * x / 4.0).sqrt();
                    let ln_r = (y - x / 2.0).cbrt() - (y + x / 2.0).cbrt();
                    ln_r.exp()
                },
            }],
        },
        FormulaEntry {
            name: "Diode Voltage vs Temperature",
            note: None,
            variants: &[SolveVariant {
                solves_for: "V_D",
                expression: "V_D = (nkT/q) × ln(I / I_s)",
                inputs: &[
                    VarDef {
                        symbol: "I",
                        name: "Forward current",
                        unit: "A",
                        default: 1e-3,
                    },
                    VarDef {
                        symbol: "I_s",
                        name: "Saturation current",
                        unit: "A",
                        default: 1e-12,
                    },
                    VarDef {
                        symbol: "T",
                        name: "Temperature",
                        unit: "°C",
                        default: 25.0,
                    },
                    VarDef {
                        symbol: "n",
                        name: "Ideality factor",
                        unit: "",
                        default: 1.0,
                    },
                ],
                output_unit: "V",
                compute: |v| {
                    let t_k = v[2] + 273.15;
                    let vt = v[3] * 1.380649e-23 * t_k / 1.602176634e-19;
                    vt * (v[0] / v[1]).ln()
                },
            }],
        },
        FormulaEntry {
            name: "Diode Saturation Current",
            note: None,
            variants: &[SolveVariant {
                solves_for: "I_s",
                expression: "I_s = α × T^(3/n) × exp(qV_G / (nkT))",
                inputs: &[
                    VarDef {
                        symbol: "α",
                        name: "Junction area constant",
                        unit: "",
                        default: 1e-8,
                    },
                    VarDef {
                        symbol: "T",
                        name: "Temperature",
                        unit: "°C",
                        default: 25.0,
                    },
                    VarDef {
                        symbol: "V_G",
                        name: "Diode voltage",
                        unit: "V",
                        default: 0.6,
                    },
                    VarDef {
                        symbol: "n",
                        name: "Ideality factor",
                        unit: "",
                        default: 1.0,
                    },
                ],
                output_unit: "A",
                compute: |v| {
                    let t_k = v[1] + 273.15;
                    let exp_arg = 1.602176634e-19 * v[2] / (v[3] * 1.380649e-23 * t_k);
                    v[0] * t_k.powf(3.0 / v[3]) * exp_arg.exp()
                },
            }],
        },
    ]
}
