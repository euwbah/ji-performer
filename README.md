# JI Performer

Convert 12 edo MIDI files into JI-tuned realtime MPE output + visualization (sends WebSocket messages compatible with https://github.com/euwbah/n-edo-lattice-visualiser)

### Accurate sleeping

Windows has particularly horrible sleep timing resolution of &approx; 15.6ms. This project uses the `spin_sleep` crate which calls `winapi`'s `timeBeginPeriod` and `timeEndPeriod` functions from `winmm.dll` to set the system timer resolution to 1ms, and on top of that, it does a spinning lock for the final fraction of a millisecond, which gives very accurate sleep times (at the expense of CPU?).
