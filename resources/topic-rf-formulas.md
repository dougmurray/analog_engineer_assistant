# Topic RF

Below are the formulas to include for RF topic.

- L-network matching
    - User inputs the source resistance (R_s) and the load resistance (R_p) and the frequency (freq). From that the Q is calculated:
        - "Q = sqrt(max(R_p, R_s) / min(R_p, R_s) - 1)"
    - Display the calculated impedance of the series X_s and the parallel X_p:
        - "X_s = Q * min(R_p, R_s)"
        - "X_p = max(R_p, R_s) / Q"
    - Display what these would be in terms of capacitance and inductance:
        - High-pass:
            - "C_s = 1 / (2 * pi * freq * X_s)"
            - "L_p = X_p / (2 * pi * freq)"
        - Low-pass:
            - "L_s = X_s / (2 * pi * freq)"
            - "C_p = 1 / (2 * pi * freq * X_p)"
    - See the below example Rust:
        ```rust
        fn cap_from_impedance(f: f32, z: f32) -> f32 {
            // Capacitor which equals Z impedance at f frequency.
            1.0 / (2.0 * PI * f * z)
        }

        fn ind_from_impedance(f: f32, z: f32) -> f32 {
            // Inductor which equals Z impedance at f frequency.
            z / (2.0 * PI * f)
        }

        fn matched_q_from_load_source(r_p: f32, r_s: f32) -> f32 {
            // Outputs matched Q based on load and source impedances
            let pre_q  = (f32::max(r_p, r_s) / f32::min(r_p, r_s) ) - 1.0;
            pre_q.sqrt()
        }

        fn matched_q_impedances(r_s: f32, r_p: f32) -> (f32, f32) {
            // This matches the Q for impedance matching circuits
            let q = matched_q_from_load_source(r_p, r_s);
            let x_s = q * f32::min(r_p, r_s);
            let x_p = f32::max(r_p, r_s) / q;
            (x_s, x_p)
        }
        ```
- Pi-network matching
    - Please display text below the equation stating:
        - "Use this network between high-value impedances (> 50 Ω). R is the virtual resistance, smaller than R_p and R_s."
    - User inputs the source resistance (R_s) and the load resistance (R_p)
    - "Q = sqrt((max(R_p, R_s) / R) - 1)"
        - SolveVariant: "R = max(R_p, R_s) / (Q^2 + 1)"
- T-network matching
    - Please display text below the equation stating:
        - "Use this network between low-value impedances (< 50 Ω). R is the virtual resistance, larger than R_p and R_s."
    - User inputs the source resistance (R_s) and the load resistance (R_p)
    - "Q = sqrt((R / min(R_p, R_s)) - 1)"
        - SolveVariant: "R = min(R_p, R_s) * (Q^2 + 1)"
- transformer matching
    - Please display text below the equation stating:
        - "Z_p represents the primary impedance, designed to be the same as the source (R_s). The Z_s is the secondary impedance, designed to be the same as the load (R_l)."
    - "Z_s = Z_p * (N_s / N_p)^2"
        - SolveVariants:
            - "Z_p = Z_s / ((N_s / N_p)^2)"
            - "N_p = N_s / (sqrt(Z_s / Z_p))"
            - "N_s = N_p * sqrt(Z_s / Z_p)"
- λ/4 Q-section transmission line matching
    - Please display text below the equation stating:
        - "Z_o is the characteristic impedance of the input transmission line from the source (R_s). Z_l is the load impedance."
    - "Z_q = sqrt(Z_o * Z_l)"
