use crate::{INFINITY, KnobSpec};
use egui::{lerp, remap, remap_clamp};
use std::ops::RangeInclusive;

// ----------------------------------------------------------------------------

// Helpers for converting knob range to/from normalized [0-1] range.

// Always clamps.

// Logarithmic knobs are allowed to include zero and infinity,

// even though mathematically it doesn't make sense.

// Code for from egui Slider.

// https://docs.rs/egui/latest/egui/widgets/struct.Slider.html

/// When the user asks for an infinitely large range (e.g. logarithmic from zero),
///
/// give a scale that this many orders of magnitude in size.
const INF_RANGE_MAGNITUDE: f32 = 10.0;

pub fn value_from_normalised(normalised: f32, range: RangeInclusive<f32>, spec: &KnobSpec) -> f32 {
    let (min, max) = (*range.start(), *range.end());
    if min.is_nan() || max.is_nan() {
        f32::NAN
    } else if min == max {
        min
    } else if min > max {
        value_from_normalised(1.0 - normalised, max..=min, spec)
    } else if normalised <= 0.0 {
        min
    } else if normalised >= 1.0 {
        max
    } else if spec.logarithmic {
        if max <= 0.0 {
            // non-positive range
            -value_from_normalised(normalised, -min..=-max, spec)
        } else if 0.0 <= min {
            let (min_log, max_log) = range_log10(min, max, spec);
            let log = lerp(min_log..=max_log, normalised);
            10f32.powf(log)
        } else {
            assert!(min < 0.0 && 0.0 < max);
            let zero_cutoff = logarithmic_zero_cutoff(min, max);
            if normalised < zero_cutoff {
                // negative
                value_from_normalised(
                    remap(normalised, 0.0..=zero_cutoff, 0.0..=1.0),
                    min..=0.0,
                    spec,
                )
            } else {
                // positive
                value_from_normalised(
                    remap(normalised, zero_cutoff..=1.0, 0.0..=1.0),
                    0.0..=max,
                    spec,
                )
            }
        }
    } else {
        debug_assert!(
            min.is_finite() && max.is_finite(),
            "Use a logarithmic range."
        );
        lerp(range, normalised.clamp(0.0, 1.0))
    }
}

pub fn normalised_from_value(value: f32, range: RangeInclusive<f32>, spec: &KnobSpec) -> f32 {
    let (min, max) = (*range.start(), *range.end());

    if min.is_nan() || max.is_nan() {
        f32::NAN
    } else if min == max {
        0.5 // empty range, show centre of slider. 
    } else if min > max {
        1.0 - normalised_from_value(value, max..=min, spec)
    } else if value <= min {
        0.0
    } else if value >= max {
        1.0
    } else if spec.logarithmic {
        if max <= 0.0 {
            // non-positive range
            normalised_from_value(-value, -min..=-max, spec)
        } else if 0.0 <= min {
            let (min_log, max_log) = range_log10(min, max, spec);
            let value_log = value.log10();
            remap(value_log, min_log..=max_log, 0.0..=1.0)
        } else {
            assert!(min < 0.0 && 0.0 < max);
            let zero_cutoff = logarithmic_zero_cutoff(min, max);
            if value < zero_cutoff {
                // negative
                remap(
                    normalised_from_value(value, min..=0.0, spec),
                    0.0..=1.0,
                    0.0..=zero_cutoff,
                )
            } else {
                // positive
                remap(
                    normalised_from_value(value, 0.0..=max, spec),
                    0.0..=1.0,
                    zero_cutoff..=1.0,
                )
            }
        }
    } else {
        debug_assert!(
            min.is_finite() && max.is_finite(),
            "Use a logarithmic range."
        );
        remap_clamp(value, range, 0.0..=1.0)
    }
}

fn range_log10(min: f32, max: f32, spec: &KnobSpec) -> (f32, f32) {
    assert!(spec.logarithmic);
    assert!(min <= max);

    if min == 0.0 && max == INFINITY {
        (spec.smallest_positive.log10(), INF_RANGE_MAGNITUDE)
    } else if min == 0.0 {
        if spec.smallest_positive < max {
            (spec.smallest_positive.log10(), max.log10())
        } else {
            (max.log10() - INF_RANGE_MAGNITUDE, max.log10())
        }
    } else if max == f32::INFINITY {
        if min < spec.largest_finite {
            (min.log10(), spec.largest_finite.log10())
        } else {
            (min.log10(), min.log10() + INF_RANGE_MAGNITUDE)
        }
    } else {
        (min.log10(), max.log10())
    }
}

fn logarithmic_zero_cutoff(min: f32, max: f32) -> f32 {
    assert!(min < 0.0 && 0.0 < max);

    let min_magnitude = if min == -INFINITY {
        INF_RANGE_MAGNITUDE
    } else {
        min.abs().log10().abs()
    };
    let max_magnitude = if max == INFINITY {
        INF_RANGE_MAGNITUDE
    } else {
        max.log10().abs()
    };

    let cutoff = min_magnitude / (min_magnitude + max_magnitude);
    debug_assert!(
        (0.0..=1.0).contains(&cutoff),
        "Bad cutoff {cutoff:?} for min {min:?} and max {max:?}"
    );
    cutoff
}
