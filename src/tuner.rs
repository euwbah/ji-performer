//! Tuning is implemented by separating each 12 edo pitch into one of 12 midi channels, and applying MPE-like pitch bend
//! to each channel.

use std::{collections::HashMap, fmt::Display, ops::Index};

use midly::{
    live::LiveEvent,
    num::{u14, u4},
    MidiMessage, PitchBend,
};
use primefactor::PrimeFactors;
use primes::{PrimeSet, Sieve};
use rational::Rational;

use crate::PB_RANGE;

pub static SEMITONE_NAMES: [&str; 12] = [
    "A", "Bb", "B", "C", "C#", "D", "Eb", "E", "F", "F#", "G", "G#",
];

/// Whether to use octave reduced monzos.
/// E.g., 5/4 will simply be [0, 0, 1> instead of [-2, 0, 1>.
const USE_OCT_RED_MONZOS: bool = true;

lazy_static! {
    /// Mapping of prime numbers to their index in the list of primes.
    /// 2 -> 0
    /// 3 -> 1
    /// 5 -> 2
    /// etc...
    ///
    /// Keys of a hash map are not sorted.
    pub static ref PRIMES: HashMap<u32, usize> = {
        let mut pset = Sieve::new();
        pset.iter().take(1000).enumerate().map(|(i, p)| (p as u32, i as usize)).collect()
    };

    /// Mapping of primes to their octaves.
    ///
    /// 2 -> 1
    /// 3 -> 2
    /// 5 -> 3
    /// 7 -> 3
    /// 11 -> 4
    /// etc...
    ///
    /// Keys are not sorted.
    pub static ref PRIMES_OCTAVES: HashMap<u32, i32> = {
        PRIMES.keys().map(|p| (*p, (*p as f64).log2().floor() as i32)).collect()
    };
}

pub type Monzo = Vec<i32>;

/// Trait for just intonation ratios.
pub trait JIRatio {
    fn monzo(&self) -> Option<Monzo>;
    fn cents(&self) -> Option<f64>;
}

impl JIRatio for Rational {
    /// Converts a rational number to monzo form. The length of the returned vector is proportional to the the prime limit
    /// of the rational.
    ///
    /// Returns [`None`] if the rational is 0.
    fn monzo(&self) -> Option<Monzo> {
        if *self == 0 {
            return None;
        }

        let mut monzo: Vec<i32> = Vec::new();
        monzo.push(0); // init monzo with at least the powers of 2.

        let num: u128 = self
            .numerator()
            .try_into()
            .expect("No negative fractions allowed");
        let den: u128 = self
            .denominator()
            .try_into()
            .expect("No negative fractions allowed");

        let num_factors = PrimeFactors::from(num);

        // For octave reduced monzos compensation.
        let mut oct_offset = 0;

        for fac in num_factors.iter() {
            let p = fac.integer;
            let exp = fac.exponent;

            let p_idx = *PRIMES
                .get(&(p as u32))
                .expect("Prime not found in PRIMES map");
            if p_idx >= monzo.len() {
                monzo.resize(p_idx + 1, 0);
            }

            monzo[p_idx] += exp as i32;

            if USE_OCT_RED_MONZOS && p != 2 {
                let prime_octs = *PRIMES_OCTAVES.get(&(p as u32)).expect("Prime not found in PRIMES_OCTAVES map");
                oct_offset += prime_octs * exp as i32;
            }
        }

        let den_factors = PrimeFactors::from(den);

        for fac in den_factors.iter() {
            let p = fac.integer;
            let exp = fac.exponent;

            let p_idx = *PRIMES
                .get(&(p as u32))
                .expect("Prime not found in PRIMES map");
            if p_idx >= monzo.len() {
                monzo.resize(p_idx + 1, 0);
            }

            monzo[p_idx] -= exp as i32;

            if USE_OCT_RED_MONZOS && p != 2 {
                let prime_octs = *PRIMES_OCTAVES.get(&(p as u32)).expect("Prime not found in PRIMES_OCTAVES map");
                oct_offset -= prime_octs * exp as i32;
            }
        }

        monzo[0] += oct_offset;

        Some(monzo)
    }

    /// Converts a rational JI interval into cents.
    ///
    /// Returns [`None`] if the rational is 0.
    fn cents(&self) -> Option<f64> {
        if *self == 0 {
            return None;
        }
        Some(self.decimal_value().log2() * 1200.0)
    }
}

/// Represents a particular tuning config to be applied starting from a given `time`
#[derive(Clone)]
pub struct TuningData {
    /// the JI tunings of each of the 12 semitones starting from A.
    ///
    /// Each element is a [`Rational`] which denotes the JI interval tuning of the i-th semitone relative to the
    /// next lowest A.
    ///
    /// E.g. if A4 = 1/1, then we can set the 8th element (fifth) to 3/2 to make E5 = 3/2 of A4.
    /// This will also make E6 3/2 of A5, E4 3/2 of A3, etc...
    ///
    /// If the rational is 0-valued, leave the previous tuning unchanged.
    pub tuning: [Rational; 12],

    /// Time to start applying this tuning config.
    pub time: f64,

    /// The ratios in monzo form (prime factorized to powers of primes), starting from A.
    ///
    /// If an element is [`None`], keep the previous tuning for this semitone.
    pub monzos: [Option<Monzo>; 12],

    /// MIDI pitch bend information for each semitone starting from A.
    ///
    /// Don't use this directly, instead use [`midi_messages`] for pre-computed messages to send.
    ///
    /// This value depends on [`PB_RANGE`]. 0x2000 is 0 bend, 0x0000 is -PB_RANGE, 0x3FFF is +PB_RANGE.
    ///
    /// If an element is [`None`], keep the previous tuning for this semitone.
    pitch_bends: [Option<PitchBend>; 12],

    /// Raw MIDI messages to be sent to the synth to apply the tuning, starting from channel 0.
    ///
    /// If an element is [`None`], keep the previous tuning for this semitone.
    pub midi_messages: [Option<Vec<u8>>; 12],
}

impl TuningData {
    /// Create a new tuning data at given time.
    ///
    /// Don't use this function directly, use the [`td`] helper function instead.
    ///
    /// `tuning` is an array of [`Rational`]s, each representing the JI tuning of the i-th semitone relative to the
    /// next lowest A. If an element of `tuning` is 0-valued, leave the tuning for that semitone unchanged.
    pub fn new(tuning: [Rational; 12], time: f64) -> Self {
        let mut monzos = tuning.map(|r| r.monzo());
        let mut pitch_bend_percents: [Option<f64>; 12] = [None; 12];

        let mut prev_cents = f64::MIN;
        for i in 0..12 {
            monzos[i] = tuning[i].monzo();

            if let Some(cents) = tuning[i].cents() {
                if cents < prev_cents && i >= 1 {
                    println!(
                        "WARN: Tuning data @ {time}s not in increasing order: {}, {}\nCheck for typos.",
                        tuning[i-1],
                        tuning[i]
                    );
                }
                prev_cents = cents;
                let cents_offset = cents - 100.0 * (i as f64);

                // from -1 to 1 (where extrema is +/- PB_RANGE semitones)
                let pb_range_percent = cents_offset / 100.0 / PB_RANGE as f64;

                if pb_range_percent > 1.0 || pb_range_percent < -1.0 {
                    panic!(
                        "ERROR for Tuning data @ {time}s. \
                    Pitch bend range ({PB_RANGE}) exceeded, unable to bend {cents_offset:.1} \
                    cents for absolute interval {}/{} assigned to note {}.\n
                    Check that this note is specified in correct octave.
                    Is this a typo? Otherwise increase PB_RANGE in src/tuner.rs.",
                        tuning[i].numerator(),
                        tuning[i].denominator(),
                        SEMITONE_NAMES[i],
                    );
                }

                pitch_bend_percents[i] = Some(pb_range_percent);
            }
        }

        let pitch_bends = pitch_bend_percents.map(|pb| {
            if let Some(pb) = pb {
                Some(PitchBend::from_f64(pb))
            } else {
                None
            }
        });

        let midi_messages: [Option<Vec<u8>>; 12] = pitch_bends
            .iter()
            .enumerate()
            .map(|(i, pb)| {
                if let Some(pb) = pb {
                    let ev = LiveEvent::Midi {
                        channel: u4::try_from(i as u8).expect("Channel out of range"),
                        message: MidiMessage::PitchBend { bend: pb.clone() },
                    };

                    let mut raw = vec![];
                    ev.write(&mut raw).unwrap();
                    Some(raw)
                } else {
                    None
                }
            })
            .collect::<Vec<Option<Vec<u8>>>>()
            .try_into()
            .unwrap();

        TuningData {
            tuning,
            time,
            monzos,
            pitch_bends,
            midi_messages,
        }
    }
}

impl Display for TuningData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..12 {
            write!(f, "{}", self.tuning[i])?;
            if i != 11 {
                write!(f, ", ")?;
            }
        }
        write!(f, "] @ {}", self.time)
    }
}

/// Helper method for creating a [`TuningData`].
///
/// - `time` is the time the tuning is applied in seconds.
///
/// - `root` ranges from 0-11 referring to A to G#, specifies which semitone the first element of the tuning array pertains to.
///
/// - `offset` is the global interval offset applied to all elements of the tuning array.
///   Use 1/1 to specify no additional offset. Use this parameter to denote comma shifts.
///
/// - `tuning` is an array of [`Rational`]s, each representing the JI tuning of the i-th semitone starting from
///   `root`, building upwards the octave. If an element of `tuning` is 0-valued, leave the tuning for that semitone unchanged.
pub fn td(time: f64, root: u8, offset: Rational, tuning: [Rational; 12]) -> TuningData {
    assert!(root < 12, "Root must be in range [0, 11]");

    let mut new_tuning = [Rational::from(0); 12];
    for i in 0..12 {
        let semitone = i + root as usize;
        new_tuning[semitone % 12] = tuning[i] * offset;

        if semitone >= 12 {
            // since tuning is specified in increasing order of pitch, when we wrap around the octave after applying
            // artificial root, we need to halve the frequency (lower an octave).
            new_tuning[semitone % 12] /= 2;
        }
    }

    TuningData::new(new_tuning, time)
}

pub struct Tuner {
    /// The current index in the `tunings` list that we're at.
    curr_tuning_idx: isize,

    /// List of tunings to be applied at given times.
    /// This must be sorted by increasing time.
    tunings: Vec<TuningData>,
}

impl Tuner {
    pub fn new(tunings: Vec<TuningData>) -> Self {
        let mut curr_time = 0.0;
        let mut sorted_tunings = tunings.clone();

        assert!(tunings.len() >= 1, "Must have at least one tuning!");

        if tunings[0].tuning.iter().any(|x| *x == Rational::zero()) {
            panic!("First tuning data cannot use 0-value elements! (No way to reference a previous tuning of this semitone)");
        }

        for td in &tunings {
            assert!(td.time >= 0.0, "Tuning time must be non-negative");
            if td.time < curr_time {
                println!(
                    "WARN: Tuning data not sorted by increasing time: {}",
                    td.to_string()
                );
                println!("Check for typo errors. Sorting automatically now...");
                sorted_tunings.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
                break;
            }
            curr_time = td.time;
        }

        Tuner {
            curr_tuning_idx: -1,
            tunings: sorted_tunings,
        }
    }

    /// Query the tuner with the current playback time. If a new tuning is to be applied.
    ///
    /// Returns the new [`TuningData`] to be applied, otherwise, returns [`None`].
    pub fn update(&mut self, time: f64) -> Option<&TuningData> {
        if self.curr_tuning_idx == -1 {
            // First tuning, apply when the first tuning time is reached.
            if time >= self.tunings[0].time {
                self.curr_tuning_idx += 1;
                return Some(&self.tunings[0]);
            }
        }

        let curr_t_idx = self.curr_tuning_idx as usize;

        if curr_t_idx == self.tunings.len() - 1 {
            // Last tuning, no more tunings to apply.
            return None;
        }

        if time < self.tunings[curr_t_idx].time {
            panic!("Time went backwards! Make sure tunings are sorted by increasing time.");
        }

        if time >= self.tunings[curr_t_idx + 1].time {
            self.curr_tuning_idx += 1;
            return Some(&self.tunings[curr_t_idx + 1]);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.tunings.len()
    }
}

impl Index<usize> for Tuner {
    type Output = TuningData;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tunings[index]
    }
}
