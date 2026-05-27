pub mod adc;
pub mod amplifiers;
pub mod basics;
pub mod conversions;
pub mod dac;
pub mod digital;
pub mod multiplexer;
pub mod pcb_wire;
pub mod sensor;

#[derive(Clone)]
pub struct VarDef {
    pub symbol: &'static str,
    pub name: &'static str,
    pub unit: &'static str,
    pub default: f64,
}

#[derive(Clone)]
pub struct SolveVariant {
    pub solves_for: &'static str,
    pub expression: &'static str,
    pub inputs: &'static [VarDef],
    pub output_unit: &'static str,
    pub compute: fn(&[f64]) -> f64,
}

#[derive(Clone)]
pub struct FormulaEntry {
    pub name: &'static str,
    pub variants: &'static [SolveVariant],
}

#[derive(Clone)]
pub struct Topic {
    pub name: &'static str,
    pub formulas: Vec<FormulaEntry>,
}

pub fn all_topics() -> Vec<Topic> {
    vec![
        Topic {
            name: "Conversions",
            formulas: conversions::formulas(),
        },
        Topic {
            name: "The Basics",
            formulas: basics::formulas(),
        },
        Topic {
            name: "Amplifiers",
            formulas: amplifiers::formulas(),
        },
        Topic {
            name: "PCB and Wire",
            formulas: pcb_wire::formulas(),
        },
        Topic {
            name: "Sensor",
            formulas: sensor::formulas(),
        },
        Topic {
            name: "Digital",
            formulas: digital::formulas(),
        },
        Topic {
            name: "ADC",
            formulas: adc::formulas(),
        },
        Topic {
            name: "DAC",
            formulas: dac::formulas(),
        },
        Topic {
            name: "Multiplexer",
            formulas: multiplexer::formulas(),
        },
    ]
}
