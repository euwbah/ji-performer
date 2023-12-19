use broadcaster::BroadcastChannel;
use futures::executor;
use midir::MidiOutput;
use midly::live::LiveEvent;
use midly::num::{u4, u7};
use midly::{self, MetaMessage, MidiMessage, PitchBend, Smf, TrackEventKind};
use rational::Rational;
use spin_sleep::{SpinSleeper, SpinStrategy};
use std::fs;
use std::io::stdin;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::server::{start_websocket_server, VisualizerMessage};
use crate::tuner::{JIRatio, Monzo, PRIMES, SEMITONE_NAMES};

#[macro_use]
extern crate lazy_static;

mod ondine;
mod server;
mod tuner;

/// Pitch bend range in +/- semitones. (Make sure PianoTeq is set to same PB value)
pub const PB_RANGE: u16 = 4;

/// Start playing from this time (in seconds).
///
/// Other meta messages (non note/cc) like tempo change, track name, etc. will still be
/// parsed, but notes will not be played and no waiting will be done until this time is reached.
const START_FROM: f64 = 0.0;

const MIDI_FILE: &str = "ondine.mid";

/// Playback speed multiplier. 1.0 is normal speed.
const PLAYBACK_SPEED: f64 = 1.0;

const MIDI_PLAYBACK_DEVICE_NAME: &str = "31edo";

/// Turn off when recording video/midi to save CPU.
const DEBUG_PRINT: bool = false;

/// Turn off when recording MIDI to save CPU.
const ACTIVATE_VISUALIZER: bool = true;

/// Turn off when recording video to save CPU.
const ACTIVATE_MIDI: bool = false;

fn main() {
    println!("JI Performer v0.1");
    println!("------------");

    // Initialize lazy_statics
    println!("Initialized {} primes", PRIMES.len());
    println!(
        "Initialized {} tunings",
        ondine::TUNER.lock().unwrap().len()
    );

    let mut broadcast_channel = start_websocket_server();

    // -----------------------------------------------------------------------------------------------------------------

    println!("Select a MIDI output port:");
    let midi_out = MidiOutput::new("JI Performer").unwrap();

    let mut midi_idx = None;

    for (idx, port) in midi_out.ports().iter().enumerate() {
        let port_name = midi_out.port_name(port).unwrap();
        if port_name.contains(MIDI_PLAYBACK_DEVICE_NAME) {
            midi_idx = Some(idx);
            println!("[{idx}] {port_name} <Device Found>");
        } else {
            println!("[{idx}] {port_name}");
        }
    }

    if let None = midi_idx {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        midi_idx = Some(input.trim().parse().unwrap());
    }

    let out_port = &midi_out.ports()[midi_idx.unwrap()];
    let mut midi_conn = midi_out.connect(out_port, "JI Performer").unwrap();

    let exit_flag = Arc::new(Mutex::new(false));

    {
        let exit_flag = exit_flag.clone();
        let res = ctrlc::set_handler(move || {
            if let Ok(mut exit_flag) = exit_flag.lock() {
                *exit_flag = true;
            }
        });
        if let Err(e) = res {
            println!("WARN: Failed to set Ctrl-C interrupt handler: {}", e);
        }
    }

    // -----------------------------------------------------------------------------------------------------------------

    let midi_file_raw_bytes = fs::read(MIDI_FILE).unwrap();
    let smf = Smf::parse(&midi_file_raw_bytes).unwrap();

    println!("Loaded MIDI file: {MIDI_FILE}");
    println!("smf tracks: {}", smf.tracks.len());

    assert!(
        smf.tracks.len() == 1,
        "Only single-track MIDI files are supported at this time"
    );

    let ppqn = match smf.header.timing {
        midly::Timing::Metrical(ppqn) => {
            println!("Ticks per quarter note: {}", ppqn);
            ppqn.as_int()
        }
        midly::Timing::Timecode(_frame_per_second, _subframes) => {
            panic!("Timecode MIDI files are not supported at this time");
        }
    };

    println!("Press enter to start playing...");

    let mut _void = String::new();
    stdin().read_line(&mut _void).unwrap();
    drop(_void);

    let track = &smf.tracks[0];

    let mut curr_tick = 0;
    let mut curr_bpm = 120f64;

    // Expected curernt time of the current track event.
    let mut expected_curr_time = 0f64;

    // Instant when the file starts playing back.
    // If we want to start playing halfway, this value is initialized to the time when the first event
    // that we want to play back is reached.
    let mut start: Option<Instant> = None;

    // On windows, these are the default settings for SpinSleeper::default(), which are using.
    //
    let spin_sleeper =
        // This crate requests 1ms native accuracy from Windows using timeBeginPeriod/timeEndPeriod,
        // which should, by right, have 1ms accuracy. Just to be safe, use 2ms.
        // reduce cpu % (and accuracy) by reducing the number below to like <= 1e6 or sth.
        SpinSleeper::new(1_000_000)
        // use x86 PAUSE instruction to notify the CPU that we are in a spin loop
        .with_spin_strategy(SpinStrategy::SpinLoopHint);

    // No need to make any custom config as the default already works fine.

    // before starting to play, send all notes off, reset all controllers, and reset pitch bend.
    reset(&mut midi_conn, &mut broadcast_channel);

    let mut tuner = ondine::TUNER.lock().unwrap();

    // Contains the current tuning. We keep track of this for debug purposes (so we can print the curr tuning as
    // formatted rationals)
    // Initialized to dummy values of 1/1 first, will be updated according to tuning data.
    let mut curr_tuning = [Rational::new(1, 1); 12];

    // Contains current tuning as monzos. Necessary to memoize monzo() calls to prevent repeated
    // prime decomposition at the speed of light.
    // The first element is for A, second Bb, etc...
    let mut curr_monzos: [Monzo; 12] = curr_tuning.map(|x| x.monzo().unwrap());

    // println!("Using default monzos: {:?}", monzos); should be array of 12 empty arrays, since 1/1 has no prime factors.

    // -----------------------------------------------------------------------------------------------------------------

    // MAIN PLAYBACK LOOP

    for event in track.iter() {
        let delta = event.delta.as_int(); // how many midi ticks after the previous event should this event occur.
        curr_tick += delta;
        let delta_crochets = (delta as f64) / (ppqn as f64); // delta in terms of quarter notes
        expected_curr_time += delta_crochets * (60f64 / curr_bpm); // crochets * (seconds / crochets) = seconds

        let tuning_data = tuner.update(expected_curr_time);

        // Memoize new tuning data.
        if let Some(tuning_data) = tuning_data {
            for (i, ratio) in tuning_data.tuning.iter().enumerate() {
                if *ratio != Rational::zero() {
                    curr_tuning[i] = *ratio;
                }
            }
            for (i, monzo) in tuning_data.monzos.iter().enumerate() {
                if let Some(monzo) = monzo {
                    curr_monzos[i] = monzo.clone();
                }
            }
        }

        if let Ok(exit_flag) = exit_flag.lock() {
            if *exit_flag {
                break;
            }
        }

        if expected_curr_time >= START_FROM && start.is_none() {
            if let TrackEventKind::Midi {
                channel: _,
                message: _,
            } = event.kind
            {
                // Start counting time from the first actual midi event (ignore metadata).
                start = Some(Instant::now());
            }
        }

        if let Some(start_instant) = start {
            // only sleep if we have reached where we want to start playing.
            let curr_time = (start_instant.elapsed().as_secs_f64() * PLAYBACK_SPEED) + START_FROM;
            let time_diff = expected_curr_time - curr_time;
            if time_diff > 0f64 {
                spin_sleeper.sleep(Duration::from_secs_f64(time_diff));
            } else if time_diff < -0.001f64 {
                println!("WARN: Falling behind by {:.3} ms", -time_diff * 1000.0);
            }
        }

        // Send new pitch bends if current tuning is to be modified.
        if let Some(tuning_data) = tuning_data {
            for pb_raw_msg in &tuning_data.midi_messages {
                if let Some(pb_raw_msg) = pb_raw_msg {
                    midi_conn.send(pb_raw_msg).unwrap();
                }
            }
            if DEBUG_PRINT {
                print!("[{curr_tick:>7}, {expected_curr_time:7.3}s] ");
                println!(
                    "Tuning:\n
                    A:  ({:.3}c) {}
                    Bb: ({:.3}c) {}
                    B:  ({:.3}c) {}
                    C:  ({:.3}c) {}
                    C#: ({:.3}c) {}
                    D:  ({:.3}c) {}
                    D#: ({:.3}c) {}
                    E:  ({:.3}c) {}
                    F:  ({:.3}c) {}
                    F#: ({:.3}c) {}
                    G:  ({:.3}c) {}
                    G#: ({:.3}c) {}
                    ",
                    curr_tuning[0].cents().unwrap(),
                    curr_tuning[0],
                    curr_tuning[1].cents().unwrap() - 100.0,
                    curr_tuning[1],
                    curr_tuning[2].cents().unwrap() - 200.0,
                    curr_tuning[2],
                    curr_tuning[3].cents().unwrap() - 300.0,
                    curr_tuning[3],
                    curr_tuning[4].cents().unwrap() - 400.0,
                    curr_tuning[4],
                    curr_tuning[5].cents().unwrap() - 500.0,
                    curr_tuning[5],
                    curr_tuning[6].cents().unwrap() - 600.0,
                    curr_tuning[6],
                    curr_tuning[7].cents().unwrap() - 700.0,
                    curr_tuning[7],
                    curr_tuning[8].cents().unwrap() - 800.0,
                    curr_tuning[8],
                    curr_tuning[9].cents().unwrap() - 900.0,
                    curr_tuning[9],
                    curr_tuning[10].cents().unwrap() - 1000.0,
                    curr_tuning[10],
                    curr_tuning[11].cents().unwrap() - 1100.0,
                    curr_tuning[11],
                );
            }
        }

        let is_midi_event = matches!(event.kind, TrackEventKind::Midi { .. });

        if (is_midi_event && start.is_some()) || !is_midi_event {
            // print!("[{curr_tick:>7}, {expected_curr_time:7.3}s] ");
        }

        match event.kind {
            TrackEventKind::Meta(MetaMessage::Tempo(tempo)) => {
                curr_bpm = 60_000_000f64 / (tempo.as_int() as f64);
                println!("Tempo: {tempo} microseconds/quarter note, {curr_bpm} bpm");
            }
            TrackEventKind::Meta(MetaMessage::EndOfTrack) => {
                println!("End of Track");
            }
            TrackEventKind::Meta(MetaMessage::Text(text)) => {
                println!("|> {}", std::str::from_utf8(&text).unwrap());
            }
            TrackEventKind::Meta(MetaMessage::TrackName(text)) => {
                println!("Track name: {}", std::str::from_utf8(&text).unwrap());
            }
            TrackEventKind::Midi { message, .. } => {
                if start.is_some() {
                    // Only send Note on/off messages if we have reached where we want to start playing.
                    // println!("MIDI Event: Channel: {}, Message: {:?}", channel, message);

                    if let MidiMessage::NoteOn { key, vel } = message {
                        // FUTURE REMINDER: a NoteOn with 0 velocity is equivalent to a NoteOff, and should
                        // be treated as such. Right now everything is ok as is, as the visualizer handles
                        // this as well. But if there's some specific on/off behaviour within this program
                        // itself, make sure to amend this!

                        let edosteps_from_a4: i32 = key.as_int() as i32 - 69;
                        let channel = edosteps_from_a4.rem_euclid(12) as u8;

                        if ACTIVATE_MIDI {
                            send_note_on(&mut midi_conn, channel, key, vel);
                        }

                        // 0 is A, 1 is Bb, etc...
                        let semitone_mod12 = (key.as_int() + 3) as usize % 12;

                        let mut monzo = curr_monzos[semitone_mod12].clone();

                        // Monzos are relative to A4, so we need to shift the octave to match
                        let octaves_from_a4 = edosteps_from_a4.div_euclid(12);

                        if monzo.len() == 0 {
                            monzo.push(octaves_from_a4);
                        } else {
                            monzo[0] += octaves_from_a4;
                        }

                        if DEBUG_PRINT {
                            print!("[{curr_tick:>7}, {expected_curr_time:7.3}s] ");
                            let note_name = SEMITONE_NAMES[semitone_mod12];
                            let octaves = (key.as_int() as i32 / 12) - 1;
                            println!("Note on: {}{}, vel: {vel}. {:?}", note_name, octaves, monzo);
                        }

                        if ACTIVATE_VISUALIZER {
                            let res = executor::block_on(broadcast_channel.send(
                                &VisualizerMessage::NoteOn {
                                    edosteps_from_a4,
                                    velocity: vel,
                                    monzo,
                                },
                            ));

                            if let Err(e) = res {
                                println!(
                                    "WARN: Failed to send message to visualizer broadcast channel: {}",
                                    e
                                );
                            }
                        }
                    } else if let MidiMessage::NoteOff { key, vel } = message {
                        let edosteps_from_a4 = key.as_int() as i32 - 69;
                        let channel = edosteps_from_a4.rem_euclid(12) as u8;

                        if ACTIVATE_MIDI {
                            send_note_off(&mut midi_conn, channel, key, vel);
                        }

                        if ACTIVATE_VISUALIZER {
                            let res = executor::block_on(broadcast_channel.send(
                                &VisualizerMessage::NoteOff {
                                    edosteps_from_a4,
                                    velocity: vel,
                                },
                            ));
                            if let Err(e) = res {
                                println!(
                                    "WARN: Failed to send message to visualizer broadcast channel: {}",
                                    e
                                );
                            }
                        }
                    }
                }

                // Send all cc messages, that come before the start time, so that existing state
                // (e.g. sustain pedal) is set correctly for the start point.
                if let MidiMessage::Controller { controller, value } = message {
                    // REMINDER: depending on the synth implementation, we may need to duplicate
                    // CC messages on to all channels. According to Pianoteq, sending
                    send_cc(&mut midi_conn, 0, controller, value);

                    let res = executor::block_on(
                        broadcast_channel.send(&VisualizerMessage::CC { controller, value }),
                    );
                    if let Err(e) = res {
                        println!("WARN: Failed to send message to vis1ualizer: {}", e);
                    }
                }
            }
            _ => {
                // TODO: remove unnecessary println once debugging is done.
                println!("Unhandled event: {:?}", event);
            }
        }
    }

    println!("Reset & closing connection...");
    reset(&mut midi_conn, &mut broadcast_channel);
    midi_conn.close();
    exit(0);
}

/// Resets all controllers, turns off all notes, reset visualizer.
fn reset(
    midi_conn: &mut midir::MidiOutputConnection,
    broadcast_channel: &mut BroadcastChannel<VisualizerMessage>,
) {
    // before starting to play, send all notes off, reset all controllers, and reset pitch bend.
    for c in 0..=15 {
        // send CC 121 (reset all controllers)
        send_cc(midi_conn, c, 121, 0);

        // send CC 123 (all notes off)
        send_cc(midi_conn, c, 123, 0);

        // send pitch bend reset
        send_pitch_bend(midi_conn, c, PitchBend::from_int(0));
    }
    // Sending the visualizer these messages once will do.
    executor::block_on(broadcast_channel.send(&VisualizerMessage::CC {
        controller: 121.into(),
        value: 0.into(),
    }))
    .unwrap();
    executor::block_on(broadcast_channel.send(&VisualizerMessage::CC {
        controller: 123.into(),
        value: 0.into(),
    }))
    .unwrap();
}

fn send_pitch_bend<T: Into<u4>>(
    midi_conn: &mut midir::MidiOutputConnection,
    channel: T,
    bend: PitchBend,
) {
    let ev = LiveEvent::Midi {
        channel: channel.try_into().expect("Channel out of range"),
        message: MidiMessage::PitchBend { bend },
    };

    let mut raw = vec![];
    ev.write(&mut raw).unwrap();
    midi_conn.send(&raw).unwrap();
}

fn send_note_on<T: Into<u4>, S: Into<u7>, U: Into<u7>>(
    midi_conn: &mut midir::MidiOutputConnection,
    channel: T,
    note: S,
    velocity: U,
) {
    let ev = LiveEvent::Midi {
        channel: channel.try_into().expect("Channel out of range"),
        message: MidiMessage::NoteOn {
            key: note.try_into().expect("Note out of range"),
            vel: velocity.try_into().expect("Velocity out of range"),
        },
    };

    let mut raw = vec![];
    ev.write(&mut raw).unwrap();
    midi_conn.send(&raw).unwrap();
}

fn send_note_off<T: Into<u4>, S: Into<u7>, U: Into<u7>>(
    midi_conn: &mut midir::MidiOutputConnection,
    channel: T,
    note: S,
    velocity: U,
) {
    let ev = LiveEvent::Midi {
        channel: channel.try_into().expect("Channel out of range"),
        message: MidiMessage::NoteOff {
            key: note.try_into().expect("Note out of range"),
            vel: velocity.try_into().expect("Velocity out of range"),
        },
    };

    let mut raw = vec![];
    ev.write(&mut raw).unwrap();
    midi_conn.send(&raw).unwrap();
}

fn send_cc<T: Into<u4>, S: Into<u7>, U: Into<u7>>(
    midi_conn: &mut midir::MidiOutputConnection,
    channel: T,
    controller: S,
    value: U,
) {
    let ev = LiveEvent::Midi {
        channel: channel.try_into().expect("Channel out of range"),
        message: MidiMessage::Controller {
            controller: controller.try_into().expect("Controller out of range"),
            value: value.try_into().expect("Value out of range"),
        },
    };

    let mut raw = vec![];
    ev.write(&mut raw).unwrap();
    midi_conn.send(&raw).unwrap();
}
