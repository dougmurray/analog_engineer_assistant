pub mod conversions;
pub mod basics;
pub mod amplifiers;
pub mod pcb_wire;
pub mod sensor;
pub mod digital;
pub mod adc;
pub mod dac;
pub mod multiplexer;

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
pub struct Chapter {
    pub name: &'static str,
    pub formulas: Vec<FormulaEntry>,
}

pub fn all_chapters() -> Vec<Chapter> {
    vec![
        Chapter { name: "Conversions",  formulas: conversions::formulas() },
        Chapter { name: "The Basics",   formulas: basics::formulas() },
        Chapter { name: "Amplifiers",   formulas: amplifiers::formulas() },
        Chapter { name: "PCB and Wire", formulas: pcb_wire::formulas() },
        Chapter { name: "Sensor",       formulas: sensor::formulas() },
        Chapter { name: "Digital",      formulas: digital::formulas() },
        Chapter { name: "ADC",          formulas: adc::formulas() },
        Chapter { name: "DAC",          formulas: dac::formulas() },
        Chapter { name: "Multiplexer",  formulas: multiplexer::formulas() },
    ]
}
