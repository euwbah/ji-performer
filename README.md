# JI Performer

Convert 12 edo MIDI files into JI-tuned realtime MPE output + visualization (sends WebSocket messages compatible with https://github.com/euwbah/n-edo-lattice-visualiser)

This works by playing back a MIDI file (in real-time) by putting each of the 12 semitones on a separate MIDI channel (A &rarr; 1, Bb &rarr; 2, etc...), and sending channel pitch bend messages to each channel. For compatibility's sake, since I can't think of any existing DAWs that can "import" a file with MIDI messages in the MPE format, so the MPE-like playback is to be recorded in real-time by a DAW.

## Demo

- [(Youtube) Ondine (Gaspard de la nuit) --- M. Ravel](https://youtu.be/Ck33YZt5Mf0)

### Usage

- Set up a virtual MIDI port (https://help.ableton.com/hc/en-us/articles/209774225-Setting-up-a-virtual-MIDI-bus)
  - If the virtual MIDI port you want to use has a consistent fixed name, you can set `MIDI_PLAYBACK_DEVICE_NAME` in [`main.rs`](./src/main.rs) to match that name, so the program won't need to ask you which MIDI device to output to every time it runs.
- You'll need a VST that supports pitch bend messages on independent MIDI channels (I use Pianoteq).
  - Otherwise, you'll need a MIDI message splitter (you can use Max, Pure Data, FL Studio Patcher, etc...), and MIDI channels 1-12 need to be routed. CC messages will only be sent on channel 1, so you'll need to forward them to all the separate VST instances.
  - For my setup, I used Ableton to record the real-time output by creating 12 MIDI tracks, assigning them to receive input from each of the 12 channels, then sending them to a 'aggregate' VST instrument track under that respective midi channel. For Pianoteq, it suffices to send CC on any single channel, and independent pitch-bend per channel works fine.
- Near the top of [`main.rs`](./src/main.rs), configure the constant `PB_RANGE` to match the configured pitch bend range of your VST. If the tunings exceed this range, this program will immediately exit with an error, and you'll have to increase the pitch bend range.
- Install [Rust compiler & toolchain](https://rustup.rs/) to download packages & compile the code:
- In [`main.rs`](./src/main.rs), configure the path `MIDI_FILE` to point to the location of your MIDI file to playback. This path can be absolute or relative to the project root directory.

Run the program with:
```sh
cargo run --release
```

To save CPU, set `ACTIVATE_VISUALIZER = false` in [`main.rs`](./src/main.rs) to disable the visualizer if you only want MIDI output.

### Activating the [visualizer](https://github.com/euwbah/n-edo-lattice-visualiser)

I retrofitted my visualizer that was originally meant for EDOs to actually work with arbitrary JI information now. Run ji-performer first to start the websocket server, then load the visualizer website to connect to the server.

Ensure the [correct configuration](https://github.com/euwbah/n-edo-lattice-visualiser?tab=readme-ov-file#set-up--config-file) is set for the visualizer, and that `USE_OCT_RED_MONZOS` in [`tuner.rs`](./src/tuner.rs) is set the same as `USE_OCTAVE_REDUCED_PRIMES` in [`configs.js`](https://github.com/euwbah/n-edo-lattice-visualiser/blob/3d/configs.js) of the visualizer.

To save CPU, set `ACTIVATE_MIDI = false` in [`main.rs`](./src/main.rs) to disable midi output if you only want visual output.

### Accurate sleeping

Windows has particularly horrible sleep timing resolution of &approx; 15.6ms. This project uses the `spin_sleep` crate which calls `winapi`'s `timeBeginPeriod` and `timeEndPeriod` functions from `winmm.dll` to set the system timer resolution to 1ms, and on top of that, it does a spinning lock for the final fraction of a millisecond, which gives very accurate sleep times (at the expense of CPU?).
