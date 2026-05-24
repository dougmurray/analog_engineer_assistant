use super::{FormulaEntry, SolveVariant, VarDef};

pub fn formulas() -> Vec<FormulaEntry> {
    vec![
        FormulaEntry {
            name: "I²C Pull-Up Resistor (Minimum)",
            variants: &[SolveVariant {
                solves_for: "R_min",
                expression: "R_min = (V_DD − V_OL_max) / I_sink_max",
                inputs: &[
                    VarDef { symbol: "V_DD",      name: "Supply voltage",      unit: "V", default: 3.3  },
                    VarDef { symbol: "V_OL_max",  name: "Max output low volt", unit: "V", default: 0.4  },
                    VarDef { symbol: "I_sink_max",name: "Max sink current",    unit: "A", default: 3e-3 },
                ],
                output_unit: "Ω",
                compute: |v| (v[0] - v[1]) / v[2],
            }],
        },
        FormulaEntry {
            name: "I²C Pull-Up Resistor (Maximum)",
            variants: &[SolveVariant {
                solves_for: "R_max",
                expression: "R_max = t_r / (0.8473 × C_b)",
                inputs: &[
                    VarDef { symbol: "t_r", name: "Rise time",       unit: "s", default: 300e-9 },
                    VarDef { symbol: "C_b", name: "Bus capacitance", unit: "F", default: 100e-12 },
                ],
                output_unit: "Ω",
                compute: |v| v[0] / (0.8473 * v[1]),
            }],
        },
    ]
}
