#!/usr/bin/env python3
# Based on Texas Instruments Analog Engineer's Pocket Reference
# slyw038c
import numpy as np

# Conversions
# Full_scale is voltage input range of ADC, double that for bipolar ADC


def error_percent(measured, ideal):
    "Returns percent of error based on measured vs ideal."
    error_percent = ((measured - ideal) / ideal) * 100
    return error_percent


def error_percent_fs(measured, ideal, fs):
    "Error in percent of full-scale range (fs)"
    error_percent_fs = ((measured - ideal) / fs) * 100
    return error_percent_fs


def ppm_to_percent(ppm):
    "Part per million to percent"
    percent = (ppm / 10e6) * 100
    return percent


def ppm_to_milli_percent(ppm):
    """Part per million to milli-percent"""
    milli_percent = (ppm / 10e6) * 100 * 1000
    return milli_percent


def percent_to_ppm(percent):
    """Percent to part per million"""
    ppm = percent * 10e4
    return ppm


def milli_percent_to_ppm(milli_percent):
    """Milli=percent to part per million"""
    ppm = milli_percent * 10
    return ppm


def millivolts_to_code(millivolt, full_scale, bits):
    """Millivolts to codes"""
    codes = millivolt * (2**bits / (full_scale * 1000))
    return codes


def percent_to_code(percent, bits):
    """Percentage of full scale to codes"""
    codes = percent * (2**bits / 100)
    return codes


def ppm_to_code(ppm, bits):
    """Parts per million to  code"""
    codes = ppm * (2**bits / 10e6)
    return codes


def code_to_millivolts(codes, full_scale, bits):
    """Code to millivolts"""
    mV = codes * (full_scale / 2**bits) * 1000
    return mV


def percentage_to_millivolts(percentage, full_scale):
    """Millivolts to percentage"""
    mV = percentage * ((full_scale * 1000) / 100)
    return mV


def ppm_to_millivolts(ppm, full_scale):
    """Ppm to millivolts"""
    mV = ppm * ((full_scale * 1000) / 10e6)
    return mV


def code_to_percentage(codes, bits):
    """Codes to percentage"""
    percentage = codes * (1/2**bits) * 100
    return percentage


def millivolts_to_percentage(millivolts, full_scale):
    """Millivolts to percentage"""
    percentage = millivolts * (1 / (full_scale * 1000) * 100)
    return percentage


def ppm_to_percentage(ppm):
    """Ppm to percentage"""
    percentage = ppm * (1/10e6) * 100
    return percentage


def code_to_ppm(codes, bits):
    """Codes to ppm"""
    ppm = codes * (1/2**bits) * 10e6
    return ppm


def millivolts_to_ppm(millivolts, full_scale):
    """Millivolts to ppm"""
    ppm = millivolts * (1 / (full_scale * 1000)) * 10e6
    return ppm


def percentage_to_ppm(percentage):
    """Percentage to ppm"""
    ppm = percentage * (10e6 / 100)
    return ppm


# Completely unnecessry
def code_to_decimal(code):
    """Converts binray code to decimal."""
    return int(code)


def decimal_to_code(number):
    """Converts decimal integer to binary code."""
    return bin(number)


def peak_peak_to_rms(peak_to_peak):
    """Peak_to_peak voltage/current to rms."""
    rms = peak_to_peak / (2 * np.sqrt(2))
    return rms


def peak_to_rms(peak):
    """Peak (amplitude) voltage/current to rms."""
    rms = peak / np.sqrt(2)
    return rms


def rms_to_peak_peak(rms):
    """RMS to peak_to_peak voltage/current."""
    peak_to_peak = (2 * np.sqrt(2)) * rms
    return peak_to_peak

def rms_to_peak(rms):
    """RMS to peak voltage/current."""
    peak = np.sqrt(2) * rms
    return peak


def gain_to_dB(gain):
    """Voltage gain (Vout/Vin) to decibels."""
    dB = 20 * np.log10(gain)
    return dB


def dB_to_gain(dB):
    """Decibels to gain (Vout/Vin)."""
    gain = 10**(dB/20)
    return gain


def power_gain_to_db(gain):
    """Power gain to decibels."""
    power_dB = 10 * np.log10(gain)
    return power_dB


def dB_to_power_gain(power_dB):
    """Power in decibels to gain."""
    power_gain = 10**(power_dB/10)
    return power_gain


def time_to_phase(time_shift, period):
    """Phase shit in degrees."""
    phase_shift = (time_shift / period) * 360  # degrees
    return phase_shift


def phase_to_time(phase, period):
    """Phase shift in seconds."""
    time_shift = (phase / 360) * period
    return time_shift


# The basics
def resistor(resistance, freq=0):
    """Resistance of resistor in Ohms, complex."""
    freq = np.array(freq)
    # res = np.full(freq.size, esr)
    res = np.full(freq.size, resistance).astype(complex)
    return res


def ind_imp(inductance, freq=0.1, esr=0):
    """Impedance of inductor in Ohms, complex."""
    # res = np.full(freq.size, esr)
    # reactance = 2 * np.pi * inductance * freq
    # impedance = np.array((res, reactance)).T # non-complex array
    freq = np.array(freq)
    res = np.full(freq.size, esr).astype(complex)
    reactance = 2 * np.pi * inductance * freq * 1.0j
    impedance = res + reactance  # complex array
    # equivalent_impedance = np.sqrt(np.real(impedance)**2
    # + np.imag(impedance)**2)
    return impedance


def cap_imp(capacitance, freq=0.1, esr=0):
    """Impedance of capacitor in Ohms, complex."""
    # res = np.full(freq.size, esr)
    # reactance = -1 / (2 * np.pi * capacitance * freq)
    # impedance = np.array((res, reactance)).T # non-complex array
    freq = np.array(freq)
    res = np.full(freq.size, esr).astype(complex)
    reactance = -1 / (2 * np.pi * capacitance * freq) * 1.0j
    impedance = res + reactance  # complex array
    # equivalent_impedance = np.sqrt(np.real(impedance)**2
    # + np.imag(impedance)**2)
    return impedance


def total_impedance(impedances):
    # equivalent_impedance = np.sqrt(np.real(impedances)**2
    # + np.imag(impedances)**2)
    equivalent_impedance = np.abs(impedances)
    return equivalent_impedance


def ohms_volt(current, resistance):
    """Ohm's law"""
    volt = current * resistance
    return volt


def ohms_current(volt, resistance):
    """Ohm's law"""
    current = volt / resistance
    return current


def ohms_resistance(volt, current):
    """Ohm's law"""
    resistance = volt / current
    return resistance


def voltage_divider(Vsup, res_one, res_two):
    """Classic voltage divider, outputs voltage"""
    vout = (res_two / (res_one + res_two)) * Vsup
    return vout


def parallel_resistance_equivalent(res_one, res_two):
    res_equivalent = (res_one * res_two) / (res_one + res_two)
    return res_equivalent


def series_resistance_equivalent(res_one, res_two):
    res_equivalent = (res_one * res_two)
    return res_equivalent


def rc_filter(res, cap):
    corner_freq = 1 / (2 * np.pi * res * cap)
    return corner_freq


def lc_filter(ind, cap):
    corner_freq = 1 / (2 * np.pi * np.sqrt(ind * cap))
    return corner_freq


def power_iv(current, volt):
    """Power from volt, current"""
    power = current * volt
    return power


def power_v2r(volt, resistance):
    """Power from volt, resistance"""
    power = volt**2 / resistance
    return power


def power_i2r(current, resistance):
    """Power from current, resistance, used for power dissipation (heat)"""
    power = current**2 * resistance
    return power


def rc_time_constant(resistor, capacitor):
    """The time constant is for charging/discharging of capacitor.
       Note that one time_constant (in seconds) is equal to 63%/36.8%
       charged/discharged respectively.
       It is generally consider that the capacitor is fully
       charged/dischaged after 5 time constants."""
    seconds = resistor * capacitor
    return seconds


def rc_charging_voltage(source_voltage, time, resistor, capacitor):
    """Voltage across capacitor after time (in seconds), charging.
       Note that in one time constant (R * C) capacitor is only 63% charged.
       Only after 5 time constants is capacitor considered
       fully charged (99.3%)."""
    tau = resistor * capacitor
    voltage_across_cap = source_voltage * (1 - np.e**(-time / tau))
    return voltage_across_cap


def rc_discharging_voltage(initial_voltage, time, resistor, capacitor):
    """Voltage across capacitor after time (in seconds), discharging.
       Note that in one time constant (R * C) capacitor is only 36.8%
       discharged. Only after 5 time constants is capacitor considered
       fully discharged (0.7%)."""
    tau = resistor * capacitor
    voltage_across_cap = initial_voltage * (np.e**(-time / tau))
    return voltage_across_cap


def snub_cap(oscillation_freq, snub_res):
    """Returns ideal cap value for snubbing newtork.
       To get snub_res value increase load with resistance and see
       which value reduces the oscillation enough. That value in Ohms
       is the snub_res value.
    """
    c_snub = 3 / (2 * np.pi * oscillation_freq * snub_res)
    return c_snub


# Opamp and Inamp calcs
def opamp_noninverting(vin, res_f, res_g):
    """Standard gain of non-inverting opamp.
       Gain true up to cutoff freq, then -20 dB/decade."""
    gain = (res_f / res_g) + 1
    vout = vin * gain
    return vout


def opamp_inverting(vin, res_f, res_g):
    """Standard gain of inverting opamp.
       Gain true up to cutoff freq, then -20 dB/decade."""
    gain = -(res_f / res_g)
    vout = vin * gain
    return vout


def opamp_noninverting_offset_voltage(opamp_offset_voltage, opamp_bias_current,
                                      res_feedback, res_g, res_noninverting):
    """Opamp offset voltage based on opamp properties and feedback elements."""
    # parallel equivalent
    res_feedback_equivalent = parallel_resistance_equivalent(res_feedback,
                                                             res_g)
    volt_offset_inverting = opamp_bias_current * res_feedback_equivalent
    volt_offset_noninverting = opamp_bias_current * res_noninverting
    offset_voltage = np.sqrt(opamp_offset_voltage**2 + volt_offset_inverting**2
                             + volt_offset_noninverting**2)
    return offset_voltage


def opamp_noise_gain(res_feedback, res_g):
    """Closed loop noise gain of opamp (always based on noninverting gain)"""
    noise_gain = res_feedback / res_g + 1
    return noise_gain


def opamp_offset_voltage_output(opamp_offset_voltage, opamp_bias_current,
                                res_feedback, res_g, res_noninverting):
    """Opamp offset voltage referred to the output."""
    volt_offset_output = (opamp_noninverting_offset_voltage(opamp_offset_voltage,
                                                            opamp_bias_current,
                                                            res_feedback,
                                                            res_g,
                                                            res_noninverting)
                          * opamp_noise_gain(res_feedback, res_g))
    return volt_offset_output


def opamp_bandwidth(opamp_gain_bandwidth_product, res_feedback, res_g):
    """Bandwidth (closed-loop) of opamp in Hz,
       based on open-loop gain vs frequency plot."""
    bandwidth = opamp_gain_bandwidth_product / opamp_noise_gain(res_feedback, res_g)
    return bandwidth


def opamp_rise_time_small_signal(opamp_gain_bandwidth_product, res_feedback, res_g):
    """Small signal rise time of opamp based on closed loop bandwidth."""
    rise_time = 0.35 / opamp_bandwidth(opamp_gain_bandwidth_product, res_feedback,
                                       res_g)
    return rise_time


def opamp_max_slew_rate_volt_output(opamp_slew_rate, freq):
    """Maximum output voltage without slew rate limit.
       Slew rate in V/us.
       Multiple by 2 for peak to peak."""
    volt_peak = opamp_slew_rate / (2 * np.pi * freq)
    return volt_peak


def inamp_common_mode_filter_cutoff(res_in, cap_common_mode):
    """Simple RC filter is made with inamp. Filter is the common mode filter."""
    cutoff_freq = 1 / (2 * np.pi * res_in * cap_common_mode)
    return cutoff_freq


def inamp_differential_filter_cutoff(res_in, cap_common_mode, cap_differential):
    """Filter cutoff of a differential inamp.
       Ideally cap_differential should be 10 times cap_common_mode.
       Note the cap_differential is the cap between the inputs."""
    cutoff_freq = 1 / ((2 * np.pi * 2 * res_in) * (cap_differential + 0.5 * cap_common_mode))
    return cutoff_freq


def opamp_noninverting_power_vout(vout, res_load, res_feedback, res_g, vsupply,
                                  opamp_supply_current):
    current_through_load = vout / res_load
    current_through_feedback = vout / (res_feedback + res_g)
    power_diss_from_load_current = ((np.abs(current_through_load)
                                     + np.abs(current_through_feedback))
                                    * (np.abs(vsupply) - np.abs(vout)))
    power_from_opamp_quiescent_current = vsupply * opamp_supply_current  # worst case
    total_power_dissipated_inside_opamp = (power_diss_from_load_current
                                           + power_from_opamp_quiescent_current)
    return total_power_dissipated_inside_opamp


def opamp_noninverting_power_max(res_feedback, res_g, res_load, vsupply, dc_out=True):
    res_equivalent_load = parallel_resistance_equivalent(res_load, (res_feedback + res_g))
    if dc_out is True:
        max_power_dissipated_inside_opamp = vsupply**2 / (4 * res_equivalent_load)
    else:
        # ac average power
        max_power_dissipated_inside_opamp = (2 * vsupply**2) / (np.pi**2 * res_equivalent_load)
    return max_power_dissipated_inside_opamp


def opamp_inverting_power_vout(vout, res_load, res_feedback, res_g, vsupply,
                               opamp_supply_current):
    current_through_load = vout / res_load
    # this is really the only difference between inverting/noninverting
    current_through_feedback = vout / res_feedback
    power_diss_from_load_current = ((np.abs(current_through_load)
                                     + np.abs(current_through_feedback))
                                    * (np.abs(vsupply) - np.abs(vout)))
    power_from_opamp_quiescent_current = vsupply * opamp_supply_current  # worst case
    total_power_dissipated_inside_opamp = (power_diss_from_load_current
                                           + power_from_opamp_quiescent_current)
    return total_power_dissipated_inside_opamp


def opamp_inverting_power_max(res_feedback, res_g, res_load, vsupply, dc_out=True):
    res_equivalent_load = parallel_resistance_equivalent(res_load, (res_feedback + res_g))
    if dc_out is True:
        max_power_dissipated_inside_opamp = vsupply**2 / (4 * res_equivalent_load)
    else:
        # ac average power
        max_power_dissipated_inside_opamp = (2 * vsupply**2) / (np.pi**2 * res_equivalent_load)
    return max_power_dissipated_inside_opamp


def opamp_power_junction_temp(theta_ja, power, room_temp=25):
    """Gives temp of opamp in C based on power dissipated.

    Keyword arguments:
    theta_ja -- junction-to-ambient thermal resistance (Ohm)
    power -- power dissipated (W)
    room_temp -- ambient temp, default 25 (C)
    """
    opamp_juntion_temp = (theta_ja * power) + room_temp
    return opamp_juntion_temp


# Noise calcs
def resistor_vnoise(resistance):
    """Very simplified, quick thermal voltage noise calculation.
       Based on the fact: 1 kOhm = 4 nV/sqrt(Hz)."""
    volt_noise = np.sqrt(resistance / 1000) * 4.0e-9
    return volt_noise


def noise_bandwidth(freq, brick_wall=1.57):
    """Noise bandwidth in Hz. Brick wall can be 1st order or...
       2nd order 1.22, 3rd order 1.13, 4th order 1.12."""
    noise_bandwidth = brick_wall * freq
    return noise_bandwidth


def opamp_noise_rti(res_f, res_g, res_in, opamp_current_noise,
                    opamp_voltage_noise, freq, brick_wall=1.57):
    """Total opamp RTI rms noise (Vrms).
       You can only add up rms noise, not spectral densisty noise (V/sqrt(Hz)).

       Keyword arguments:
       res_f -- feedback resistance (Ohm)
       res_g -- resistance to ground in feedback (Ohm)
       res_in -- noninverting input resistance (Ohm)
       opamp_current_noise -- current noise of opamp (A/sqrt(Hz))
       opamp_voltage_noise -- voltage noise of opamp (V/sqrt(Hz))
       freq -- frequency bandwidth of interest (Hz)
       brick_wall -- filter correction factor, default = 1.57 for 1st order
    """
    # parallel equivalent
    res_feedback_equivalent = parallel_resistance_equivalent(res_f, res_g)
    opamp_feedback_noise = (opamp_current_noise * res_feedback_equivalent
                            * np.sqrt(noise_bandwidth(freq, brick_wall)))
    opamp_noninverting_input_noise = (opamp_current_noise * res_in
                                      * np.sqrt(noise_bandwidth(freq, brick_wall)))
    feedback_res_noise = (resistor_vnoise(res_feedback_equivalent)
                          * np.sqrt(noise_bandwidth(freq, brick_wall)))
    res_in_noise = resistor_vnoise(res_in) * np.sqrt(noise_bandwidth(freq, brick_wall))
    total_rti_noise = np.sqrt(opamp_voltage_noise**2 + res_in_noise**2
                              + feedback_res_noise**2 + opamp_feedback_noise**2
                              + opamp_noninverting_input_noise**2)
    return total_rti_noise


def opamp_noise_rms_simple(res_f, res_g, res_in, opamp_vn, opamp_in, bandwidth):
    """Simple total opamp RTI rms noise (Vrms).
       You can only add up rms noise, not spectral densisty noise (V/sqrt(Hz)).

       Keyword arguments:
       res_f -- feedback resistance (Ohm)
       res_g -- resistance to ground in feedback (Ohm)
       res_in -- noninverting input resistance (Ohm)
       opamp_voltage_noise -- voltage noise of opamp (V/sqrt(Hz))
       opamp_current_noise -- current noise of opamp (A/sqrt(Hz))
       freq -- frequency bandwidth of interest (Hz)
    """
    opamp_vn_contribution_rms = opamp_vn * np.sqrt(bandwidth) * (1 + (res_f / res_g))
    opamp_inverting_input_contribution_rms = opamp_in * np.sqrt(bandwidth) * res_f
    opamp_noninverting_input_contribution_rms = opamp_in * np.sqrt(bandwidth) * (res_in * (1 + (res_f / res_g)))
    res_in_contribution_rms = (np.sqrt(res_in / 1000) * 4.0e-9) * np.sqrt(bandwidth) * (1 + (res_f / res_g))
    res_g_contribution_rms = (np.sqrt(res_g / 1000) * 4.0e-9) * np.sqrt(bandwidth) * res_f  # yes, res_f is correct
    res_f_contribution_rms = (np.sqrt(res_f / 1000) * 4.0e-9) * np.sqrt(bandwidth)
    total_noise_rms = np.sqrt(opamp_vn_contribution_rms**2
                              + opamp_inverting_input_contribution_rms**2
                              + opamp_noninverting_input_contribution_rms**2
                              + res_in_contribution_rms**2 + res_g_contribution_rms**2
                              + res_f_contribution_rms**2)
    return total_noise_rms


def opamp_noise_simple(res_f, res_g, res_in, opamp_vn, opamp_in, bandwidth):
    """Simple total opamp RTI noise (V/sqrt(Hz)).
       Vrms noise / sqrt(bandwidth) to get  spectral densisty noise (V/sqrt(Hz)).

       Keyword arguments:
       res_f -- feedback resistance (Ohm)
       res_g -- resistance to ground in feedback (Ohm)
       res_in -- noninverting input resistance (Ohm)
       opamp_voltage_noise -- voltage noise of opamp (V/sqrt(Hz))
       opamp_current_noise -- current noise of opamp (A/sqrt(Hz))
       freq -- frequency bandwidth of interest (Hz)
    """
    total_noise_rms = opamp_noise_rms_simple(res_f, res_g, res_in, opamp_vn, opamp_in, bandwidth)
    total_noise = total_noise_rms / np.sqrt(bandwidth)
    return total_noise


def opamp_noise_rto(total_rti_noise, res_feedback, res_g):
    """Total opamp RTO rms noise (Vrms)."""
    total_rto_noise = total_rti_noise * opamp_noise_gain(res_feedback, res_g)
    return total_rto_noise


def noise_to_noise_rms(noise, freq, brick_wall=1.57):
    """Converts spectral density noise (V/sqrt(Hz)) into rms noise (Vrms)."""
    noise_rms = noise * np.sqrt(noise_bandwidth(freq, brick_wall))
    return noise_rms


def noise_rms_to_noise_spectral_density(noise_rms, freq, brick_wall=1.57):
    spectral_density_noise = noise_rms / np.sqrt(noise_bandwidth(freq, brick_wall))
    return spectral_density_noise


# ADC/DAC Converters
def full_scale_to_lsb(full_scale, bits):
    """Voltage step size of each LSB (least significant bit)."""
    lsb = full_scale / 2**bits
    return lsb


def adc_vref_to_full_scale(vref, pga_gain):
    """Full-scale range (FSR), in volts, based on reference voltage and adc_gain."""
    full_scale = vref / pga_gain
    return full_scale


def adc_input_to_code(voltage_input, bits, full_scale):
    """Outputs binary code based on input voltage"""
    output_code = np.round(voltage_input / (full_scale / 2**bits))
    return output_code


def adc_max_rms_full_scale(full_scale, bits):
    """The rms equivalent, voltage, of the ADC's full-scale input.
       This can also be caluated by: (full_scale / 2) / sqrt(2)"""
    max_rms_signal = (full_scale_to_lsb(full_scale, bits) * 2**(bits - 1)) / np.sqrt(2)
    return max_rms_signal


def adc_rms_noise(full_scale, bits):
    """This is the rms voltage noise from quantization only.
       This is basically: 1 LSB / sqrt(12)"""
    rms_noise = full_scale_to_lsb(full_scale, bits) * np.sqrt(12)
    return rms_noise


def adc_snr(bits):
    """Generic SNR in dB based on bits (resolution)."""
    snr = 6.02 * bits + 1.76
    return snr

# Pg 130 last left off


def dac_code_to_output(code, full_scale, bits):
    """Outputs output voltage based on binary code."""
    dac_vout = code * (full_scale / 2**bits)
    return dac_vout


def dac_max_output(full_scale, bits):
    max_vout = (2**bits - 1) * full_scale_to_lsb(full_scale, bits)
    return max_vout
