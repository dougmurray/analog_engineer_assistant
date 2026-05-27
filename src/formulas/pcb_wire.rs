use super::{FormulaEntry, SolveVariant, VarDef};

const PI: f64 = std::f64::consts::PI;

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        FormulaEntry {
            name: "PCB Parallel Plate Capacitance",
            variants: &[SolveVariant {
                solves_for: "C",
                expression: "C(pF) = k × l × w × εr / h  (k=8.854×10⁻³ pF/mm)",
                inputs: &[
                    VarDef {
                        symbol: "l",
                        name: "Length (mm)",
                        unit: "mm",
                        default: 5.08,
                    },
                    VarDef {
                        symbol: "w",
                        name: "Width (mm)",
                        unit: "mm",
                        default: 12.7,
                    },
                    VarDef {
                        symbol: "h",
                        name: "Separation (mm)",
                        unit: "mm",
                        default: 1.575,
                    },
                    VarDef {
                        symbol: "εr",
                        name: "Dielectric const",
                        unit: "",
                        default: 4.5,
                    },
                ],
                output_unit: "pF",
                compute: |v| 8.854e-3 * v[0] * v[1] * v[3] / v[2],
            }],
        },
        FormulaEntry {
            name: "PCB Microstrip Inductance",
            variants: &[SolveVariant {
                solves_for: "L",
                expression: "L(nH) = kL × l × ln(5.98h / (0.8w + t))  (kL=2 nH/cm)",
                inputs: &[
                    VarDef {
                        symbol: "l",
                        name: "Length (cm)",
                        unit: "cm",
                        default: 2.54,
                    },
                    VarDef {
                        symbol: "h",
                        name: "Separation (mm)",
                        unit: "mm",
                        default: 0.8,
                    },
                    VarDef {
                        symbol: "w",
                        name: "Width (mm)",
                        unit: "mm",
                        default: 0.254,
                    },
                    VarDef {
                        symbol: "t",
                        name: "Cu thickness (mm)",
                        unit: "mm",
                        default: 0.0356,
                    },
                ],
                output_unit: "nH",
                compute: |v| 2.0 * v[0] * (5.98 * v[1] / (0.8 * v[2] + v[3])).ln(),
            }],
        },
        FormulaEntry {
            name: "PCB Microstrip Capacitance",
            variants: &[SolveVariant {
                solves_for: "C",
                expression: "C(pF) = kC × l × (εr+1.41) / ln(5.98h/(0.8w+t))  (kC=0.264 pF/cm)",
                inputs: &[
                    VarDef {
                        symbol: "l",
                        name: "Length (cm)",
                        unit: "cm",
                        default: 2.54,
                    },
                    VarDef {
                        symbol: "h",
                        name: "Separation (mm)",
                        unit: "mm",
                        default: 0.8,
                    },
                    VarDef {
                        symbol: "w",
                        name: "Width (mm)",
                        unit: "mm",
                        default: 0.254,
                    },
                    VarDef {
                        symbol: "t",
                        name: "Cu thickness (mm)",
                        unit: "mm",
                        default: 0.0356,
                    },
                    VarDef {
                        symbol: "εr",
                        name: "Dielectric const",
                        unit: "",
                        default: 4.5,
                    },
                ],
                output_unit: "pF",
                compute: |v| {
                    0.264 * v[0] * (v[4] + 1.41) / (5.98 * v[1] / (0.8 * v[2] + v[3])).ln()
                },
            }],
        },
        FormulaEntry {
            name: "PCB Microstrip Characteristic Impedance",
            variants: &[SolveVariant {
                solves_for: "Z₀",
                expression: "Z₀ = (87 / √(εr+1.41)) × ln(5.98h / (0.8w+t))",
                inputs: &[
                    VarDef {
                        symbol: "h",
                        name: "Separation (mm)",
                        unit: "mm",
                        default: 0.8,
                    },
                    VarDef {
                        symbol: "w",
                        name: "Width (mm)",
                        unit: "mm",
                        default: 0.254,
                    },
                    VarDef {
                        symbol: "t",
                        name: "Cu thickness (mm)",
                        unit: "mm",
                        default: 0.0356,
                    },
                    VarDef {
                        symbol: "εr",
                        name: "Dielectric const",
                        unit: "",
                        default: 4.5,
                    },
                ],
                output_unit: "Ω",
                compute: |v| {
                    (87.0 / (v[3] + 1.41).sqrt()) * (5.98 * v[0] / (0.8 * v[1] + v[2])).ln()
                },
            }],
        },
        FormulaEntry {
            name: "PCB Adjacent Traces Cap (Same Layer)",
            variants: &[SolveVariant {
                solves_for: "C",
                expression: "C(pF) ≈ k × t × l / d  (k=8.854×10⁻³ pF/mm)",
                inputs: &[
                    VarDef {
                        symbol: "l",
                        name: "Trace length (mm)",
                        unit: "mm",
                        default: 25.4,
                    },
                    VarDef {
                        symbol: "t",
                        name: "Trace thickness (mm)",
                        unit: "mm",
                        default: 0.0348,
                    },
                    VarDef {
                        symbol: "d",
                        name: "Distance between traces (mm)",
                        unit: "mm",
                        default: 0.254,
                    },
                ],
                output_unit: "pF",
                compute: |v| 8.854e-3 * v[1] * v[0] / v[2],
            }],
        },
        FormulaEntry {
            name: "PCB Adjacent Traces Cap (Different Layers)",
            variants: &[SolveVariant {
                solves_for: "C",
                expression: "C(pF) ≈ k × εr × w × l / h  (k=8.854×10⁻³ pF/mm)",
                inputs: &[
                    VarDef {
                        symbol: "l",
                        name: "Trace length (mm)",
                        unit: "mm",
                        default: 25.4,
                    },
                    VarDef {
                        symbol: "w",
                        name: "Trace width (mm)",
                        unit: "mm",
                        default: 0.635,
                    },
                    VarDef {
                        symbol: "h",
                        name: "Layer separation (mm)",
                        unit: "mm",
                        default: 1.6,
                    },
                    VarDef {
                        symbol: "εr",
                        name: "Dielectric const",
                        unit: "",
                        default: 4.5,
                    },
                ],
                output_unit: "pF",
                compute: |v| 8.854e-3 * v[3] * v[1] * v[0] / v[2],
            }],
        },
        FormulaEntry {
            name: "PCB Via Inductance",
            variants: &[SolveVariant {
                solves_for: "L",
                expression: "L(nH) ≈ kL × h × [1 + ln(4h/d)]  (kL=0.2 nH/mm)",
                inputs: &[
                    VarDef {
                        symbol: "h",
                        name: "Via height (mm)",
                        unit: "mm",
                        default: 1.6,
                    },
                    VarDef {
                        symbol: "d",
                        name: "Via hole diameter (mm)",
                        unit: "mm",
                        default: 0.4,
                    },
                ],
                output_unit: "nH",
                compute: |v| 0.2 * v[0] * (1.0 + (4.0 * v[0] / v[1]).ln()),
            }],
        },
        FormulaEntry {
            name: "PCB Via Capacitance",
            variants: &[SolveVariant {
                solves_for: "C",
                expression: "C(pF) ≈ kC × εr × h × d₁ / (d₂−d₁)  (kC=0.0555 pF/mm)",
                inputs: &[
                    VarDef {
                        symbol: "h",
                        name: "Separation (mm)",
                        unit: "mm",
                        default: 1.6,
                    },
                    VarDef {
                        symbol: "d",
                        name: "Via hole diameter (mm)",
                        unit: "mm",
                        default: 0.4,
                    },
                    VarDef {
                        symbol: "d1",
                        name: "Pad diameter (mm)",
                        unit: "mm",
                        default: 0.8,
                    },
                    VarDef {
                        symbol: "d2",
                        name: "GND clearance (mm)",
                        unit: "mm",
                        default: 1.5,
                    },
                    VarDef {
                        symbol: "εr",
                        name: "Dielectric const",
                        unit: "",
                        default: 4.5,
                    },
                ],
                output_unit: "pF",
                compute: |v| 0.0555 * v[4] * v[0] * v[2] / (v[3] - v[2]),
            }],
        },
        FormulaEntry {
            name: "Coaxial Cable Capacitance per Length",
            variants: &[SolveVariant {
                solves_for: "C/l",
                expression: "C/l = 2πε / ln(D/d)",
                inputs: &[
                    VarDef {
                        symbol: "D",
                        name: "Shield inner diameter",
                        unit: "m",
                        default: 0.00495,
                    },
                    VarDef {
                        symbol: "d",
                        name: "Inner conductor dia",
                        unit: "m",
                        default: 0.000913,
                    },
                    VarDef {
                        symbol: "εr",
                        name: "Dielectric const",
                        unit: "",
                        default: 2.3,
                    },
                ],
                output_unit: "F/m",
                compute: |v| {
                    let eps = v[2] * 8.854e-12;
                    2.0 * PI * eps / (v[0] / v[1]).ln()
                },
            }],
        },
        FormulaEntry {
            name: "Coaxial Cable Inductance per Length",
            variants: &[SolveVariant {
                solves_for: "L/l",
                expression: "L/l = μ/(2π) × ln(D/d)",
                inputs: &[
                    VarDef {
                        symbol: "D",
                        name: "Shield inner diameter",
                        unit: "m",
                        default: 0.00495,
                    },
                    VarDef {
                        symbol: "d",
                        name: "Inner conductor dia",
                        unit: "m",
                        default: 0.000913,
                    },
                    VarDef {
                        symbol: "μr",
                        name: "Relative permeability",
                        unit: "",
                        default: 1.0,
                    },
                ],
                output_unit: "H/m",
                compute: |v| {
                    let mu = v[2] * 4.0 * PI * 1e-7;
                    mu / (2.0 * PI) * (v[0] / v[1]).ln()
                },
            }],
        },
        FormulaEntry {
            name: "Coaxial Cable Characteristic Impedance",
            variants: &[SolveVariant {
                solves_for: "Z₀",
                expression: "Z₀ = (1/2π) × √(μ/ε) × ln(D/d)",
                inputs: &[
                    VarDef {
                        symbol: "D",
                        name: "Shield inner diameter",
                        unit: "m",
                        default: 0.00495,
                    },
                    VarDef {
                        symbol: "d",
                        name: "Inner conductor dia",
                        unit: "m",
                        default: 0.000913,
                    },
                    VarDef {
                        symbol: "εr",
                        name: "Dielectric const",
                        unit: "",
                        default: 2.3,
                    },
                    VarDef {
                        symbol: "μr",
                        name: "Relative permeability",
                        unit: "",
                        default: 1.0,
                    },
                ],
                output_unit: "Ω",
                compute: |v| {
                    let eps = v[2] * 8.854e-12;
                    let mu = v[3] * 4.0 * PI * 1e-7;
                    (1.0 / (2.0 * PI)) * (mu / eps).sqrt() * (v[0] / v[1]).ln()
                },
            }],
        },
    ]
}
