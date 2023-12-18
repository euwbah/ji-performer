//! Contains tuning data for Ondine (Gaspard de la Nuit, Ravel).
//!
//! Page number & score references are with respect to the EDITION PETERS publication.
//!
//! This file contains candid comments about my thought process as I was deciding the tuning of this piece,
//! and several alternative options may also be given in comments below. I left in all the failed tuning attempts
//! (commented out of course) and all the little experiments I did along the way to document my process.
//!
//! Tuning is presented in bar-by-bar order.
//!
//! Feel free to modify the code and try them out yourself.

use std::sync::{Arc, Mutex};

use rational::{extras::r, Rational};

use crate::tuner::{td, Tuner};

lazy_static! {
    /// Tuner configuration for Ondine
    pub static ref TUNER: Arc<Mutex<Tuner>> = {
        // TODO: The timings are not finalized, record ondine first, then set tuning timings to exactly
        // match the recording.

        let mut t = Vec::new();

        // Use this value to keep previous setting for this note.
        // Any tuning using `P` can be thought of as a 'common tone' tuning.
        let P: Rational = Rational::zero();

        // Bar 0: C# harmonic scale.
        // C# (root) tuned to 5/4 of A440.
        let c_s = r(1, 1);
        let d_s = r(9, 8);
        let e_s = r(5, 4);
        let f_s = r(4, 3);
        let g_s = r(3, 2);
        let a = r(13, 8);
        let a_s = r(5, 3); // must use 5/3 for compatibility with D# minor later
        let b = r(7, 4);
        let b_s = r(15, 8);
        // (otonal placeholders are for unplayed notes)
        t.push(td(0.0, 4, r(5, 4), [
            c_s, r(17, 16), d_s, r(19, 16),
            e_s, f_s, r(11, 8), g_s,
            a, a_s, b, b_s
        ]));

        // Bar 5: A# harm 7 (A#, E# common)
        let c_x = a_s * r(5, 8); // maj 3rd of A#
        let g_s = a_s * r(7, 8); // h7 of A#
        t.push(td(18.448, 4, r(5, 4), [ // written as C# root
            P, c_x, P, P,
            P, P, P, g_s,
            P, P, P, P, // (B# remains as 9/8 of A#)
        ]));

        // Bar 5:4: C#6 (Reset G#)
        let g_s = c_s * r(3, 2);

        t.push(td(21.328, 4, r(5, 4), [
            P, P, P, P,
            P, P, P, g_s,
            P, P, P, P,
        ]));

        // Bar 6: A#!7
        let g_s = a_s * r(7, 8); // h7 of A#
        t.push(td(22.406, 4, r(5, 4), [
            P, P, P, P,
            P, P, P, g_s,
            P, P, P, P,
        ]));

        // Bar 8: alternating between D#m6 & B9 (later F#m6add4)

        // reset G# as P5 of C# root (note not played, just in case leftover from sustain pedal)
        let g_s = c_s * r(3, 2);

        // For B#, can try using 17/10 (sharpen) or 13/8 (flatten) for a more complicated 6th sonority
        // but it's probably better to use more common tones, especially at the start of the piece
        // let b_s = d_s * r(17, 10);

        // F# (m3 of D#) sounds better in 7-limit, as in 7/6 of D#, otherwise too major-ey
        // (5-limit min 3rds too strongly imply 3rd and 5th of a maj triad)
        // However, using 7/6 for D#-F# m3 won't work well with the B9 chord,
        // as we need the B-F# P5th, making B-D# a 9/7, which is too wide for the
        // root-mediant function (9/7 works better as subdominant/b7-9 function).
        //
        // We can use the mediant of two fractions (a + b) / (c + d) to get one
        // between a/c and b/d, so let's take the mediant of 6/5 and 7/6 to get
        // (6 + 7) / (5 + 6) = 13/11.
        //
        // This is a higher complexity m3. To some, it defeats the purpose of JI which is to
        // have nice concordant (buzz-ily in-tune) structures all around. However, Ondine was written
        // with the symmetrical tempered commas of Z/12Z (integers mod 12) in mind, evidenced by
        // the use of subgroup symmetries:
        // - Z/3Z (Maj3rd symmetries; climax @ bar 66),
        // - Z/4Z (min3rd symmetries; between recurrence of the "Mon pere bat l'eau..." whipping water theme
        //   introduced/hinted at in bar 45, theme reappears in min3rd symmetries)
        // - Z/2Z (tritone symmetry; the whipping water theme itself makes use of the tritone sub)
        //
        // In the spirit of NEJI/Zheanism proimodal theory, we can use tunings that can 'pretend' to evoke
        // these 'constant structure' symmetries with moderately higher-complexity intervals, while still
        // being able to maintain the non-irrational 'buzziness' of JI.
        let f_s = d_s * r(13, 11);

        // Other chord has B as functional root (we construct the F#m6 using B9 since there's a B
        // hidden in the voicing)
        // use F# as common tone (F# = P5 of B)
        // Again, don't use D# as 5-limit Maj3 common tone as consonance is not intended and
        // it ruins the fifth.
        let b = f_s * r(4, 3); // aka 52/33 of D#

        // The original root note C# gets comma pumped, it has to function as 3/2 of F#
        // C# -> C# * 9/8 * 13/11 * 3/4 = 351/352 (minthma: https://en.xen.wiki/w/352/351)
        // approx -4.9 cents.
        //
        // Alternative would be to not pump this and make F#-C# a 'wolf' fifth from bars
        // 8-13 in order to preserve tuning of C# when C# tonality comes back in bar 14.
        // But C# and F# are used a lot in 8-13, so nah, comma pump -4.9c it is.
        let c_s = f_s * r(3, 4);

        // A = h7 of B, for preparing F# primodal-6 in bars 10-13
        let a = b * r(7, 8); // aka 91/66 of D# (woo scary)

        t.push(td(28.578, 4, r(5, 4), [
            c_s, P, P, P,
            P, f_s, P, g_s,
            a, P, b, P,
        ]));

        /*
        BARS 1-9 XENPAPER:
        https://xenpaper.com/
(1/4)
{r220hz}
{r5/4}
# Bar 0-7 (ish)
[7/8, 1/1, 5/4, 3/2, 13/8]-
[15/16, 1/1, 5/4, 3/2, 13/8]
[5/6, 1/1, 5/4, 3/2, 5/3]
{r5/6}
[1/1, 5/4, 3/2, 7/4, 9/4]
{r6/5}
{r9/8}
# Bar 8
{r220hz}
{r5/4}
{r9/8}
(1)
[1/1, 10/9, 13/11, 3/2, 5/3]-
[39/44, 1/1, 13/11, 3/2, 5/3]--
[3/4, 1/1, 13/11, 3/2, 5/3]
[39/44, 1/1, 13/11, 91/66, 52/33]
[1/1, 13/11, 91/66, 52/33]
        */

        // -----------------------------------
        // PAGE 2

        // Bar 10: F#m(add4) (same tuning as Bar 8, so A-B is 8/7)
        // Functional root is now F# (= 9/8*13/11 = 117/88 of C#, ~ 493.11c),
        // Chord is a very buzzy 7/6 minor chord with non-fundamental tonic F#
        // that is 3/2 of the fundamental of the otonal stack starting on B

        // Bar 11: F# primodal-6 minor (6:7:8:9:10:11), the full otonal stack

        // Since the tessitura of this section is high, take advantage of highly
        // concordant JI stacks to evoke super strong combination tones & virtual fundamental.

        // the 11th harmonic (w.r.t B) for the note E# is hauntingly appropriate for
        // "Ondine's melody". Also reminiscent of maqam Rast.
        let e_s = b * r(11, 16); // 11/8 of B, 11/6 of F#

        // Since Ravel avoids D# in bar 10, we can safely pump it as 5/6 of F# to achieve
        // the otonal stack (previously it was 11/13 of F#)
        let d_s = f_s * r(5, 6);

        t.push(td(39.480, 4, r(5, 4), [
            P, P, d_s, P,
            e_s, P, P, P,
            P, P, P, P,
        ]));

        // Bar 14: C# otonal returns.
        // Ravel avoids C# and F# in bars 14-15, and D# in previous bar 13 (intentionally?)
        // However, C# was used very recently in bar 13, so if we un-pump it, while C# itself
        // does not appear in the coming bars, the minthma +4.9c change is still noticeable
        // as a 'JND-like' timbral change. Interesting effect. E.g.:

        /* https://xenpaper.com/
# bar 10
{r220hz}
{r5/4}
{r9/8}
{r13/11}
(1)
# 1/1 is F#
[3/4, 1/1, 7/6, 4/3, 3/2]-
[3/4, 1/1, 1/1, 7/6]-
[3/4, 1/1, 7/6, 7/6]
[3/4, 1/1, 7/6, 4/3, 3/2]
[5/6, 1/1, 7/6, 4/3, 5/3]----
[3/4, 1/1, 7/6, 4/3, 3/2]
[1/1, 7/6, 4/3, 2/1]
[11/12, 1/1, 7/6, 4/3, 11/6]
[5/6, 1/1, 7/6, 4/3, 5/3]-
[3/4, 1/1, 7/6, 4/3, 3/2]
[5/6, 1/1, 7/6, 4/3, 5/3]
[1/1, 7/6, 4/3, 2/1]
[11/12, 1/1, 7/6, 4/3, 11/6]
[3/4, 1/1, 7/6, 4/3, 3/2]-
[2/3, 1/1, 7/6, 4/3]
[3/4, 1/1, 7/6, 4/3, 3/2]

        Compare the following 2 continuations (comment/uncomment the {r351/352} line)

# bar 14 (unpumped, back to original)
{r220hz}
{r5/4}
[9/8, 5/4, 3/2, 13/8, 9/4]-
[7/8, 5/4, 3/2, 13/8, 7/4]--


# bar 14 (pumped minthma)
{r220hz}
{r5/4}
{r351/352}
[9/8, 5/4, 3/2, 13/8, 9/4]-
[7/8, 5/4, 3/2, 13/8, 7/4]--
         */

        // To me, the pumped version sounds like a smoother transition, more like the original
        // way I hear the song in 12edo.
        //
        // However, I really liked how the unpumped version has a 'disjointed' timbre between
        // sections. (My guess is the timbre 'difference' is caused between slight differences in
        // short term memory and experienced stimuli)
        //
        // Whatever it is, the disjointedness made E# (5/4 of C#) and A (13/8 of C#) stand out stronger as
        // a more 'augmented' sound, which is in alignment with the Z/3Z symmetry theme
        // of this piece. It brought out something new to these two bars that I didn't really
        // experience practicing the original over and over again in 12edo.

        let c_s = r(1, 1); // unpump C#

        // no need to worry about the D# pump also, since this note wasn't recently used.
        let d_s = r(9, 8);
        let e_s = r(5, 4); // revert E# (very safe, not recently used)
        let f_s = r(4, 3); // unused, but revert for good measure
        // G# wasn't modified.
        let a = r(13, 8);
        let a_s = r(5, 3);
        let b = r(7, 4);

        t.push(td(47.969, 4, r(5, 4), [
            c_s, P, d_s, P,
            e_s, f_s, P, P,
            a, a_s, b, P,
        ]));

        // Bar 16: alternating between F#9(13) and A#7#11(no3)
        // This part makes use of an E augmented chord in 2nd inversion (B#-E-G#) that is constant
        // between the two alternating chords (the E and G# are constant, but the triad can function over
        // both chords)
        //
        // Keeping with the harmonic series theme

        // the 'temporary root' of this part is 4/3 (F#), we build otonally from here.

        let e = f_s * r(7, 8); // functions as 7/4 of F#
        // F# remains 4/3, A# remains as 5/4 of F# (5/3), G# remains as 9/8 of F#
        // D# remains as 27/16 of F#
        // B remains as 21/16 of F# (to form the fifth between E and B beat 3.5)

        t.push(td(56.076, 4, r(5, 4), [
            P, P, P, e,
            P, P, P, P,
            P, P, P, P,
        ]));

        // Bar 17: A#7#11(no3) voiced as inversion of F#13#11
        // Can still use F# otonal stack for this chord, and the 11th harmonic B# in the chromatically
        // ascending melody follows a nice isoharmonic sequence from the previous bar (20, 21, 22)
        //
        // However, we need to decide between several possible tunings for D#:

        /* https://xenpaper.com
# bar 16
[1/1, 7/6, 4/3, 3/2, 5/3, 9/2]
[1/1, 7/6, 4/3, 3/2, 5/3, 4/1]
[1/1, 7/6, 4/3, 3/2, 5/3, 3/1]-
[1/1, 4/3, 3/2, 5/3, 10/3]
[1/1, 4/3, 3/2, 5/3, 7/2]

            1. Using unchanged tuning for D#. The immediate transition from F#9(13) to A#7#11 is smooth,
            but the 27/16 6th is harshly sharp sounding. It melds with the other notes, but doesn't bring out
            otonal-ness, but instead uses relies on the perception of simplicity of 3-limit intervals.

# 17 (A#-D#: P4)
[5/6, 11/12, 3/2, 11/6, 7/3, 3/1, 11/3]
[5/6, 11/12, 9/8, 3/2, 11/6, 9/4]
[5/6, 11/12, 9/8, 3/2, 11/6, 9/4, 3/1]-

            2. Using D# = 13/8 of F#, we get a more obvious otonal stack, but the 13th harmonic is octave
            displaced, so the effect of the otonal structure is not clear. The drop in pitch between the
            D# of the previous bar and this one is very obvious (9/8 of C# vs 13/12 of C#, -65.3c drop),
            making each bar sound like its own tonality, non-contiguous

# 17 (F#-D#: 13/8)
[5/6, 11/12, 3/2, 11/6, 7/3, 3/1, 11/3]
[5/6, 11/12, 13/12, 3/2, 11/6, 13/6]
[5/6, 11/12, 13/12, 3/2, 11/6, 13/6, 3/1]-

            3. Using D# = 6/5 of B# brings out the melody, since the maj 6th melodic leap downwards
            from B# to D# is made clearer due to the simpler interval relationship. However, the
            otonalness is completely lost, and now this sound is heavily 11-centric
            (very 'iron(II) oxide' sounding). The difference between the two D#s is still
            noticeable, (lowered by -38.9c).

            Note that this tuning is also the mediant of the first two tunings:
            (9+13) / (4+6) = 22/10

# 17 (B#-D#: 6/5)
[5/6, 11/12, 3/2, 11/6, 7/3, 3/1, 11/3]
[5/6, 11/12, 11/10, 3/2, 11/6, 22/10]
[5/6, 11/12, 11/10, 3/2, 11/6, 22/10, 3/1]-

            4. Using D# = mediant of 11-limit version and 13-limit version (yields a 7-limit version).
            It lowers D# by -48.7c, but sounds like a stable discordance, evokes 22edo-like sounds.

# 17 (mediant of 22/10 and 13/6)
[5/6, 11/12, 3/2, 11/6, 7/3, 3/1, 11/3]
[5/6, 11/12, 35/32, 3/2, 11/6, 35/16]
[5/6, 11/12, 35/32, 3/2, 11/6, 35/16, 3/1]-

            5. Using D# = mediant of 11-limit version (3rd) and 3-limit version (first tuning).

            This sounds very stable, not too sharp, not too flat. The D# melds, perhaps a bit too
            well, until it no longer stands out as a melodic accent. This is the best sounding one
            thus far, let's use this for now. Flattens D# by -27.7c.

# 17 (mediant of 22/10 and 9/4)
[5/6, 11/12, 3/2, 11/6, 7/3, 3/1, 11/3]
[5/6, 11/12, 31/28, 3/2, 11/6, 31/14]
[5/6, 11/12, 31/28, 3/2, 11/6, 31/14, 3/1]-

         */

        // mediant of 11-limit version and 3-limit tunings
        //
        let d_s = c_s * r(31, 28);

        t.push(td(59.141, 4, r(5, 4), [
            P, P, d_s, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        // Bar 18: F#9(13)/G# (Same as bar 16)
        let d_s = r(9, 8); // back to normal
        t.push(td(61.109, 4, r(5, 4), [
            P, P, d_s, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        // Bar 19: A#7#11/E (same 31 limit tuning as bar 17)
        let d_s = c_s * r(31, 28);
        t.push(td(64.188, 4, r(5, 4), [
            P, P, d_s, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        // -----------------------------------
        // PAGE 3

        // Bar 20: F#9(13)/C# (as C#m6add11)
        let d_s = r(9, 8); // back to normal
        t.push(td(66.438, 4, r(5, 4), [
            P, P, d_s, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        // Bar 21: A#m11b5 (slightly different sonority here)
        // No more D# here, and the function of D# on beat 3 of this bar
        // is different, we can use the 13 limit D# to bring out the full
        // primodal-3 stack: [5, 6, 7, 9, 11, 13]/3
        let d_s = f_s * r(13, 16);
        t.push(td(69.338, 4, r(5, 4), [
            P, P, d_s, P,
            P, P, P, P,
            P, P, P, P,
        ]));


        /* https://xenpaper.com

        BARS 16-21
{r220hz}
{r5/4}
(1)
# bar 16
[1/1, 7/6, 4/3, 3/2, 5/3, 9/2]
[1/1, 7/6, 4/3, 3/2, 5/3, 4/1]
[1/1, 7/6, 4/3, 3/2, 5/3, 3/1]-
[1/1, 4/3, 3/2, 5/3, 10/3]
[1/1, 4/3, 3/2, 5/3, 7/2]

# bar 17 (mediant of 22/10 and 9/4)
[5/6, 11/12, 3/2, 11/6, 7/3, 3/1, 11/3]
[5/6, 11/12, 31/28, 3/2, 11/6, 31/14]
[5/6, 11/12, 31/28, 3/2, 11/6, 31/14, 3/1]-

# 18 (same as 16, G# bass)
[3/4, 5/6, 1/1, 7/6, 4/3, 9/2]
[3/4, 7/6, 4/3, 3/2, 5/3, 4/1]
[3/4, 7/6, 4/3, 3/2, 5/3, 3/1]-
[3/4, 5/6, 1/1, 7/6, 10/3]
[5/6, 1/1, 7/6, 7/4, 7/2]

# bar 19 (same as 17, over E bass)
[7/12, 11/12, 3/2, 11/6, 7/3, 3/1, 11/3]
[7/12, 5/6, 11/12, 31/28, 3/2, 11/6, 31/14]
[7/12, 5/6, 11/12, 31/28, 3/2, 11/6, 31/14, 3/1]-

# bar 20 (same as 16, low C# bass)
[1/2, 2/3, 3/4, 5/6, 7/6, 4/3, 9/2]
[1/2, 7/6, 4/3, 3/2, 5/3, 4/1]
[2/3, 7/6, 4/3, 3/2, 5/3, 3/1]-
[1/2, 2/3, 3/4, 5/6, 7/6, 10/6, 10/3]
[2/3, 3/4, 5/6, 7/6, 7/4, 7/2]

# bar 21 (sim. 17, low A# bass)
[5/12, 7/12, 3/4, 11/12, 3/1, 11/3, 6/1]
[11/12, 7/6, 3/2, 11/6, 7/3, 14/3]
[11/12, 7/6, 3/2, 11/6, 7/3, 11/3]-
[3/4, 5/6, 1/1, 13/12, 2/1]
[7/12, 3/4, 5/6, 1/1, 13/12, 13/6]
[5/12, 6/12, 7/12, 3/4, 7/6, 7/3]
         */

        // Bar 22: New root: G#. G# major scale, using 5-limit tuning, the P4 (C#) is set to 11/8 of G#
        // for stronger shimmering effect, and because the function of C# is not to act as the 4th
        // that resolves down to 3 via clausula tenorizans molle.

        let g = g_s * r(15, 16); // G = 5-lim maj7th of G#
        let d_s = g_s * r(3, 4); // D# = P5 of G#
        let c_s = g_s * r(11, 16); // C# = 11th harmonic of G#

        t.push(td(74.063, 4, r(5, 4), [
            c_s, P, d_s, P,
            P, P, g, P,
            P, P, P, P,
        ]));

        // The very last note of bar 22 (C#) should be tuned as 4/3 of G# instead of 11/8 of G#, as
        // this note resolves tenorizans molle to B# eventually, functioning as dom 7th of D#9sus4
        // coming up in bar 23.

        // HOWEVER, in bar 24, we see that C# is also used as 5/4 of an A major triad upper structure
        // which requires us to premptively pump this down by 16245/16384 (-14.7c).
        // We set it to 5415/8192 of G# instead (explanation in Bars 23-24 below)

        let c_s = g_s * r(5415, 8192); // reset P4 of G# as P4 function
        t.push(td(77.100, 4, r(5, 4), [ // last note of the LH scale
            c_s, P, P, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        // Bar 23: D#9sus4(add10)
        // Bar 23:3: A7(13)
        // Bar 23:3.5: C7(13)

        // Notice the m3rd symmetries being used here (Ravel picks notes from
        // the octatonic "diminished"/half-whole scale)

        // The C# (dom7) of D#9sus4 functions as 5/4 of A7, and the C7(13), while appearing
        // to be a completely separate constant structure shifted chord, adds on to the short
        // term memory of pitches, making the relatively simpler A7(13) change sonority to A13b9#11.
        // We can also put C7(13) into context and see that it is a C13b9.

        // The use of Z/4Z symmetries in the diminished tetrad: D#-F#-A-C is very clear here,
        // so let's use a tuning that brings out the tempered symmetry of (6/5)^4 ~= 2/1, by
        // using a m3 close to 300 cents. (in comparison, the 6/5 m3 is 315.6c)

        // Consider the the interval between D# and C, where D# is the root of the first chord,
        // and C is the 'root' of the last chord in this bar, C7(13). It would be nice if this
        // C and Fx (5/4 of D#) in bar 24 was as close to a P5 as possible so that going from
        // enharmonics G (bar 23) to Fx (24) is not jarring.
        // Going by Ravel's choice of enharmonic spelling, it would be alright if C and Fx is a wolf fifth,
        // but we can in fact minimize the wolfness by finding a minor third that can 'equally divide' the
        // diminished 7th between C and D# such that C is as close as possible to 2/3 of Fx.
        //
        // Choosing to 'temper' D# = Eb, Fx = G, we try to map the D#-C interval as a simple 5/3
        // which is 884.3c, divided into 3 equal minor thirds, each one will be 294.7c.

        // We have 19/16 = 297.5c, 13/11 = 289.2c, and 32/27 = 294.1c.
        // Hence, if we want to highlight the Z/4Z symmetry here, we can use 19/16 as it's closer to 300c,
        // but we may need to separate Fx and G as two separate tunings (as per score) as the wolf-ing is strong.
        // if we want to reduce the wolfness of C-G vs C-Fx, we can use 32/27, which gets very close to 3 equal
        // divisions of 5/3, which lets us use the same tuning for Fx and G.

        /*

# bar 22
{r220hz}
{r5/4}
{r3/2}
(env:0999)
(4)
1/2 9/16 [10/16, 5/4, 3/2, 15/8, 5/2] 11/16 12/16 5/6 15/16
1/1 9/8 5/4 4/3

            This is the first option, using 32/27 as a min 3rd generator for the chain D#-F#-A-C,
            falsely equating tunings for A# = Bb, Fx = G. This one prioritises less wolf between C and G
            in the 3rd chord. C-G is +1.95c sharp from just 3/2.

(bpm:100)
(1)
(env:1565)

# bar 23 32/27 diminshed stack

# D#  G#   A#   C#   Fx    A#
[3/4, 1/1, 9/8, 4/3, 15/8, 9/4]---
# G     A        C#   E       F#    A
[15/16, 256/243, 4/3, 128/81, 16/9, 512/243]
# Bb  C          G     E       A        C
[9/8, 8192/6561, 15/8, 128/81, 512/243, 16384/6561]

            This is the second option, using 19/16 as a m3 generator for D#-F#-A-C. This one prioritises
            minor third cyclic symmetry using a generator closer to 300c, but the interval between
            C-G is -8.18c flat from 3/2. In comparison, the 31edo fifth is 5.18c sharp from just.

# bar 23 19/16 diminished stack

# D#  G#   A#   C#   Fx    A#
[3/4, 1/1, 9/8, 4/3, 15/8, 9/4]---
# G     A          C#   E          F#     A
[15/16, 1083/1024, 4/3, 3249/2048, 57/32, 1083/512]
# Bb  wolf: C      G     E          A         C
[9/8, 20577/16384, 15/8, 3249/2048, 1083/512, 20577/8192]
         */

        // I prefer the second one as it has a warmer sound, the first one sounds 'angular'.
        // The wolf interval is very passable as it's not supposed to be a consonant sonority.
        // As long as it doesn't sound like it's comma shifting all over the place, the 19/16
        // stack is a good choice.

        let a_s = d_s * r(3, 2); // A#: 3/2 of chord root D#, also functions as Bb.
        let f_x = d_s * r(5, 4); // Fx: 5/4 of chord root D#, also functions as G.
        let f_s = d_s * r(19, 16); // F#: 19/16 of D#
        let a = f_s * r(19, 16); // stacked 19/16 from F#
        let c = a * r(19, 16); // stacked 19/16 from a
        let e = a * r(3, 4); // E: P4 below A.

        // Now there's the question of the tuning of E#. If we make it 5/3 of the fundamental root G#,
        // i.e. 5/4 of C#, it will be the 10/9 of the chord root D#, and it wolfs with A#, by a syntonic comma.
        // It doesn't have a stable sound.

        // For the note E#, make use of the 'tenorizans molle clausula' to "temper" the "E#" to E movement,
        // a kind of pseudo meantone temperament where we even out the difference between 9/8 and 10/9.
        // We can take the 17/16 semitone above E to get E#, which is in fact 55233/32768 of fundamental G#
        // which looks very scary, but this gives us a P5 tuning for A#-E# of 699.9c, very very 12-NEJI.

        let e_s = e * r(17, 16); // E#: 17/16 of E

        // Finally, the tuning of C# is originally 9/8 of chord root D#, but in bar 24, we have the
        // full C13b9 voicing with an A major triad on top. To bring out the 13b9 texture better,
        // we make C# = 5/4 of A, which pumps it down by 16245/16384 (14.7c)

        t.push(td(77.268, 4, r(5, 4), [
            P, P, P, e,
            e_s, f_s, f_x, P,
            a, a_s, P, c,
        ]));

        /*
        This tuning settles bars 23-25:

# bar 23
{r220hz}
{r5/4}
{r3/2}
(1)
# D#  G#   A#   C#         E#           Fx    A#
[3/4, 1/1, 9/8, 5415/4096, 55233/32768, 15/8, 9/4]---
# G     A          C#         E          F#     A
[15/16, 1083/1024, 5415/4096, 3249/2048, 57/32, 1083/512]
# Bb  C            G     E          A         C
[9/8, 20577/16384, 15/8, 3249/2048, 1083/512, 20577/8192]

# bar 24
# D#  G#   A#   C#         E#           Fx    A#
[3/4, 1/1, 9/8, 5415/4096, 55233/32768, 15/8, 9/4]--
[3/4, 1/1, 9/8, 5415/4096, 55233/32768, 15/8, 9/4]
# G     A          C#         E          F#     A
[15/16, 1083/1024, 5415/4096, 3249/2048, 57/32, 1083/512]
# Bb  C            G     E          A         C#
[9/8, 20577/16384, 15/8, 3249/2048, 1083/512, 5415/2048]

# bar 25
[3/4, 1/1, 9/8, 5415/4096, 55233/32768, 15/8, 9/4]--
[3/4, 1/1, 9/8, 5415/4096, 55233/32768, 15/8, 9/4]-
[3/4, 1/1, 9/8, 5415/4096, 15/8, 55233/32768, 9/4, 55233/16384]
         */

        // Notice that bar 25 no longer uses the diminished stack, letting the sonority of
        // F# reset in time for bar 26.

        // Bar 26: B#m11b5 (the 11 = E# = 55233/32768 for continuity)
        // In moving from D#9sus4(add10),

        // We can use a sweeter F# for the half dimished sound. Instead of constructing
        // B#m11b5 as upper notes of a G# (dominant) fundamental, we know that it is not
        // because the melody has a suspension (E#) that strongly suggests the D#m6 essence
        // over the G#7 essence.

        // We carry over the important A# anchor note's tuning from the previous,
        // which we recall is now tuned at 27/16 from the original 1/1 root of the beginning of the piece.

        // We take F# to be the 5-limit major third below A# (4/5)
        let f_s = a_s * r(4, 5);
        // the fundamental root should still technically be G#, (even though using D#m6) sonority,
        // so use 5/4 of G# for B# to keep consistent. This is a 9/5 m7th away from A#,
        // B#-F# forms a 36/25 tritone (interval between a 3-limit major 2nd and two 5-limit maj 3rds)
        // creates a very pure augmented sonority within the half dim itself, consisentent with how
        // Ravel explores relationships between Z/3Z and Z/4Z.
        let b_s = g_s * r(5, 4);
        let c_s = g_s * r(2, 3); // reset C#-G# P5 in case, even though we're not using it.

        // Premptive note: If we don't un-pump this F#, by bar 30, we will have pumped up
        // by a syntonnic comma, but since the next section is in G#, and G# has been our
        // harmonic fundamental that we've been building off of all this while

        t.push(td(86.424, 4, r(5, 4), [
            c_s, P, P, P,
            P, f_s, P, P,
            P, P, P, b_s,
        ]));

        // Bar 27: E#9

        // This chord may look out of place initially, until we see that bar 26 is has subdominant
        // function in a Sub-Dom-Tonic cadence, A#m11b5 is the 'ii', so E#9 is the V7.

        // The last two chords of bar 26 has no E#, so we can safely revert E# to 5-limit tunings:
        // E# is just a 2/3 fifth below B#, and since B# was tuned as 5/4 of G#, this means
        // E# = 5/4 of original C# root.
        let e_s = b_s * r(2, 3);
        assert!(e_s == r(5, 4), "Math not mathing");
        let g_x = e_s * r(5, 4); // Gx = 5-limit maj third of root E#
        let f_x = e_s * r(9, 8); // diatonic 2nd

        t.push(td(88.199, 4, r(5, 4), [
            P, P, P, P,
            e_s, P, f_x, P,
            g_x, P, P, P,
        ]));

        // Bar 27:4.5: E#7b9

        // Again looks weird on the score, but it's just E#7b9 (F# is enharmonic b9 of E#)

        // since there is no F# any time soon, we are free to tune the b9 however we want.
        // bars 28-29 are rich, so go for rich sounds.

        let f_s = e_s * r(17, 16); // 17th harmonic of E#
        t.push(td(92.576, 4, r(5, 4), [
            P, P, P, P,
            P, f_s, P, P,
            P, P, P, P,
        ]));

        // Bar 28: A#9#11(no3)
        // This chord reinforces the augmented symmetry theme of E+ = G#+ = B#+ as
        // a structure over A#, however, it is a shell chord and it's not possible to
        // identify the 'root' using notes in this bar alone.
        //
        // The obvious answer would be to assume that the previous E# is the V-dominant of this
        // A#'s Tonic, which would be nice. Looking forward though, m. 29 uses B as the dominant
        // of G# (m. 30), which highlights the use of augmented symmetry that B ~ D# ~ G
        // (hinting at the climax at m. 66) such that B ~ D# -> G# (V-I cadence by means of Z/3Z).

        // ~ means 'symmetrically equivalent to'
        // -> means 'resolves to'

        // By transitivity of the operators: Caug/A# ~ Caug/F# ~ F#7#11 -> B ~ D# -> G#
        //
        // This A#9#11 chord is in fact fundamentally rooted as F#, and A# is
        // the primodal-under-5 'false root'.

        // Recall that the JI structure 11:14:18 is a non-octave symmetry in 31edo that tempers the
        // mothwellsma (99/98), that is, 11:14, and 14:18 evenly splits 11:18 into two equal
        // parts. In JI, this comma is about 17.5c, so it is noticeable, but we can still use
        // this idea to color bar 28 with a superaugmented sound (stacking super thirds)

        // In order to preserve the tuning of G# (new key in m. 30) without pumping any commas
        // we let the middle of the mothwellsmic triad, the 14th harmonic, be G# itself, and let
        // E and B# be 11 and 18 respectively.

        // In doing this, we can justify having both F# and A# as possible
        // fundamental roots for this bar, and by pretending we "tempered" the mothwellsma
        // we can use Ravel's A# bass functionally as the fundamental JI root, while still
        // maintaining the tuning for G#.

        // This is a very bizzare sound, but it doesn't stray from the original effect of m. 28
        // in 12edo.

        let b_s = g_s * r(9, 7); // B# = 18th harmonic of A#
        let a_s = g_s * r(8, 7); // G# corresponds to 7th harmonic of A#, so A# = 8/7 w.r.t G#
        let e = g_s * r(11, 14); // E = 11th harmonic of A#

        t.push(td(93.242, 4, r(5, 4), [
            P, P, P, e,
            P, P, P, P,
            P, a_s, P, b_s,
        ]));

        // Bar 29: B9sus4, B9, B13b9

        // There's only one obvious option for this bar, notice that Ravel does not write
        // C# and G# in the same chord, which (coincidentally?) prevents needing to choose between
        // with wolf 5ths between C#-G#, or wolf 4ths between D#-G#.
        //
        // The D# is persistent, so we set G# as our new root 1/1 (which is still unaltered
        // from the beginning), set B as the 6/5 of G#, B-F#-C# is a P5 (3/2) chain,
        // D# is tuned as the 5/4 of B, which makes it also the 3/4 of G#


        let b = g_s * r(6, 5); // Tune B w.r.t anchor note G# as 6/5

        // chain of fifths: B-F#-C#
        let f_s = b * r(3, 4);
        let c_s = f_s * r(3, 4);

        // The C for the 13b9 chord can be set to a whole bunch of values,
        // as m. 30 has a clash between B and B#, so either ways
        // it's probably not important to maintain any particular pitch of C.

        let c = b * r(19, 18);
        // alternative options to try:
        // let c = g_s * r(5, 4);
        // let c = g_s * r(32, 25);

        let a = b * r(7, 8); // the 7th harmonic here gives a nice ring

        // even though there isn't an E in this bar, it could be added to the first chord
        // (not following the score as written) to give more septimal color by building it
        // off the septimal A.
        let e = a * r(3, 4);

        assert!(d_s == g_s * r(3, 4)); // just checking

        t.push(td(93.309, 4, r(5, 4), [
            c_s, P, P, e,
            P, f_s, P, P,
            a, P, b, c,
        ]));

        /*

# bar 26
# B#  F#    A#   E#           F#   A#   E#
[5/8, 9/10, 9/8, 55233/32768, 9/5, 9/4, 55233/16384]-
# B#  F#    A#   D#   A#   D#
[5/8, 9/10, 9/8, 3/2, 9/4, 3/1]-
# B#
[5/16, 9/20, 9/16, 9/8, 3/2, 9/5, 9/4]-

# bar 27
# E#   B#    Gx     Fx     D#   Fx
[5/24, 5/16, 25/48, 15/16, 3/2, 15/8]---
[5/24, 5/16, 25/48, 15/16, 3/2, 15/8]-
# E#   B#    Gx     F#    D#   F#   Gx
[5/24, 5/16, 25/48, 85/96, 3/2, 85/48, 25/12]-

(bpm:90)
# bar 28
# A#   G#   E      B#   E     B#
[4/14, 1/2, 11/14, 9/7, 11/7, 18/7]
[4/14, 1/2, 11/14, 1/1, 11/7, 16/7]-
[4/14, 1/2, 11/14, 11/7, 2/1]

(bpm:80)
# bar 29
# B    F#    A      C#     (E)    A      C#
[3/10, 9/20, 21/40, 27/40, 63/80, 21/20, 27/20]
#                          D#   F#
[3/10, 9/20, 21/40, 27/40, 3/4, 9/10]-
#                   C      D#   G#
[3/10, 9/20, 21/40, 19/30, 3/4, 1/1]
*/

        // -----------------------------------
        // PAGE 4

        // Bar 30: G# harmonic, with added m3 clash

        // Start on a clean slate, using whatever the last tuning of G# was

        let a_s = g_s * r(9, 8); // 9th harm
        let b_s = g_s * r(5, 4); // 10th
        let d_s = g_s * r(3, 4); // 12th
        let e = g_s * r(13, 16); // 13th
        let f_s = g_s * r(7, 8); // 7th

        // Update these just in case
        let c_s = g_s * r(2, 3); // 4/3 P4 of G#
        let e_s = g_s * r(5, 6); // 5/3 Maj6

        // Choose isodifference for m3-M3 clash (B-B#)
        // Since D#-E clash is 13/12, let B-B# be 13/12 as well.
        let b = b_s * r(12, 13);
        // Alternative options for b: 7/6, 6/5, 13/11, 39/66 of G#

        t.push(td(100.905, 4, r(5, 4), [
            c_s, P, d_s, e,
            e_s, f_s, P, g_s,
            P, a_s, P, b_s,
        ]));

        // Bar 33: D#m7b5 (F#m6) anchored by melody D#.

        // keep D# tuning consistent, then use (subminor)/6
        // tuning where F# = 1/1, A = 7/6, C# = 9/6, D# = 10/6 relative to F#, E# = 11/6.
        // at the same time, D# = 3/2 of previous G# tuning.

        let f_s = d_s * r(6, 5); // D# is 10/6 of F#, previously F# was 7/8 of G# root.
        let a = f_s * r(7, 6);
        let c_s = f_s * r(3, 4);
        let e_s = f_s * r(11, 12);

        t.push(td(109.792, 4, r(5, 4), [
            c_s, P, P, P,
            e_s, f_s, P, P,
            a, P, P, P,
        ]));

        // Bar 36: G# harmonic

        // need to revert F# to 7/8 of G#
        let f_s = g_s * r(7, 8);
        t.push(td(117.992, 4, r(5, 4), [
            P, P, P, P,
            P, f_s, P, P,
            P, P, P, P,
        ]));

        /*

# bar 30, 31
{r220hz}
{r5/4}
{r3/2}
(1)
(bpm:110)
(env:1749)
[1/4, 3/8, 1/2, 15/26, 5/8, 3/4, 13/16]-
[1/2, 3/4, 1/1, 5/4, 3/2, 13/8]-
[1/1, 5/4, 3/2, 13/8]-------

# bar 32
(env:1741)
[9/8, 1/1, 5/4, 3/2, 13/8]
[1/1, 1/1, 5/4, 3/2, 13/8]
[7/8, 1/1, 5/4, 3/2, 13/8]
[13/16, 1/1, 5/4, 3/2, 13/8]
[9/8, 1/1, 5/4, 3/2, 13/8]-
[1/1, 1/1, 5/4, 3/2, 13/8]
[3/4, 1/1, 5/4, 3/2, 13/8]

# bar 33
[3/4, 9/10, 21/20, 27/20, 3/2]--
[27/40, 9/10, 21/20, 27/20, 3/2]
[9/10, 9/10, 21/20, 27/20, 3/2]
[33/40, 9/10, 21/20, 27/20, 3/2]

# bar 34
[3/4, 9/10, 21/20, 27/20, 3/2]-
[27/40, 9/10, 21/20, 27/20, 3/2]
[3/4, 9/10, 21/20, 27/20, 3/2]
[9/10, 9/10, 21/20, 27/20, 3/2]
[33/40, 9/10, 21/20, 27/20, 3/2]

# bar 35
[27/40, 9/10, 21/20, 27/20, 3/2]--
[3/5, 9/10, 21/20, 27/20, 3/2]

# bar 36
[3/4, 1/1, 5/4, 3/2, 13/8]---

# bar 37
[9/8, 1/1, 5/4, 3/2, 13/8, 9/4]
[1/1, 1/1, 5/4, 3/2, 13/8, 2/1]
[7/8, 1/1, 5/4, 3/2, 13/8, 7/4]
[13/16, 1/1, 5/4, 3/2, 13/8]
[1/4, 3/8, 5/8, 3/4, 13/16, 9/8, 5/4, 3/2, 13/8, 9/4]-
[1/4, 3/8, 5/8, 3/4, 13/16, 1/1, 2/1]
[1/4, 3/8, 5/8, 3/4, 13/16, 3/4, 3/2]
         */

        // Bar 38: F#9(13)
        // Bar 39: A9(13)/F# (or F#7b9#9sus4)
        // Bar 40: F#9(13) to A9(13)/F#

        // For consistency, the last time the chord root moves to F# (m. 33), it was
        // tuned to 6/5 of D#, so effectively 9/10 of G#.

        let f_s = g_s * r(9, 10);

        // Initial thoughts: tuning E is not trivially simple as 7/4 of F#.
        // I want to consider the melodic function of the notes in the melody,
        // and how it resolves to the D# in m. 41.
        //
        // 38               39          40                     41
        // D# - - C# F# E | C# - B C# | D# C# F# E D# - C# B | A#
        //
        // the last 3 notes C#, B, A# has the function b7 b6 5 (where 1 = D#).
        //
        // Bring forward the tuning of B in m. 41 to m. 40, so B is 13/8 of A#
        // (to keep the 13/8 augmented 5th of the shimmering theme consistent)
        //
        // This implies that we let D# be the new fundamental 1/1, to prepare the
        // ears for the resolution in bar 41.
        //
        // Then, functionally, F#9(13) makes use of the (partial) augmented symmetry of
        // F#9 ~ A#9 -> D#, and the A9(13)/F# should be seen with respect to D#'s overtones
        //
        // The melody should have consistent tunings for notes that appear over both chords:
        // i.e., C# and D#. So let's consider those two first
        //
        // Ideally, C# should be 7/4 of the new D#, but it's not possible because it needs
        // to work well with F# as well. C# doesn't come until bar 42, though, so we can have
        // different tunings for C# for mm. 38-40, and m. 42.
        //
        // TODO: I'll first try to use C# = 3/2 of F# until beat 4 of m. 40, then
        // snap tune it exactly at that moment to be 7/4 of D#.
        //
        // For D#, it is almost forced to maintain as the 3/2 of G#, since the melody ties
        // D# over mm. 37-38, and it would be good to maintain the theme of the main motif
        // reappearing in perfect 3/2 transpositions (first C#, then G#, next D#).

        assert!(d_s == g_s * r(3, 4)); // D# maintains as is.

        let c_s = f_s * r(3, 4); // F#-C# forms P5, important interval

        /*

        The 12-centric theoretical chord root of A9(13) would be... A, but this chord
        should be more 'dominant' and discordant compared to the F#9(13), which is to
        act as a stable tonal centre (the motif is exploring the Z/4Z symmetries
        of going from F# to A to D# then C, almost like Central Park West (Coltrane))

        I want to make the A to D# Z/2Z (tritone sub) symmetry explicit.
        In 12edo theory, a tritone sub is defined by the 3 and b7 of a dominant chord swapping
        roles (since the tritone equally divides 12 semitones into 2 equal parts of 6 each).

        Hence, I want to look at the tuning for C# and G explicitly. Ravel writes G for
        mm. 38-40, but Fx for mm. 41, so G need not necessarily be 5/4 of D#.

        However, to minimize 'outness', I want to temper G such that it is compatible with
        the symmetries at hand.

        Notice that F#-C# is a forced 3/2, A# is part of the F#9(13) chord, but it's tuning
        need not be 5/4 of F# (mm. 38-40 is the 'tension' section, no need for excess concordance)

        Then, we can have the following symmetry: A#-C#-E-G, fixing C# as 3/2 of F#.

        It wasn't obvious to me playing and analyzing this piece in 12-centric mindset, but
        now it feels that mm 38-40 is a development/continuation of the harmonic structures in mm. 23-24.

        Previously (mm. 23-24), 19/16 (297.5c) was used for its closeness to 300c, and that 3 stacks of
        19/16 is 'close enough' to another necessary 5/3 relationship that was necessary to ensure
        that C-Fx double augmented 4th does not 'wolf' too badly.

        The same Z/4Z (min 3rds stack) stuff is going on, but this time there are
        different constraints:
        - D# has a fixed tuning (3/2 of G#)
        - so does F# (9/10 of G#)
        - so does C# (3/2 of F#, 27/20 of G#)
        - C# is the fixed anchor point in the m3 chain A#-C#-E-G
        - A#-D#-G forms the quintessential upper structure of the F#13b9(#9) sound in m. 40
        - G and Fx can't be too far off.
        - Maj 3rd between F# and A# should be passable, since mm. 38-40 uses F#9(13) with a
          clear intention of a major third function
        - Size of min 3rds A#-C#-E-G must be close to 300c to exhibit 'symmetry'.
        - Ideally, E should be as close to 7/4 of F# as possible to achieve that septimal 7th 'buzz'.
          This is not as important as the others above, but E should be a reasonable tuning that can
          let F#-E function as a dom 7th. (anything wider than 11/6 should not be allowed).

        Hence, we need a m3 chain 'generator' that yields passable tunings for the intervals
        A#-D# and D#-G.

        First, consider 19/16 as the generator (again) for A#-C#-E-G. Relative to G#, this will give:
        - A# = 108/95 (16/19 under C#)
        - C# = 27/20 (fixed)
        - E = 513/320 (19/16 above C#)
        - G = 9747/5120 (19/16 above E)
        - Fx = 15/8 (fixed)

        This gives (w.r.t. G#):
        - A#-D# = 3/2 * 95/108 = 95/72 = 479.9c (terribly flat fourth)
        - D#-G = 9747/5120 * 2/3 = 3249/2560 = 412.6c (very sharp Maj 3)
        - Fx-G = 9747/5120 * 8/15 = 3249/3200 = 26.3c (very sharp unison)
        - F#-A# = 108/95 * 10/9 = 24/19 = 404.4c (slightly sharp Maj 3)
        - m3 size = 19/16 = 297.5c (close to 300c)
        - F#-E = 513/320 * 10/9 = 57/32 = 999.4c (very sharp, passable 7/4 or dom7th)

        This looks quite bad. If we invert the faux 'symmetry' such that we expand the
        chain of 19/16s from the fixed C# instead:

        - A# = 108/95 (16/19 under C#)
        - C# = 27/20 (fixed)
        - E = 55296/34295 (16/19 under G)
        - G = 3456/1805 (16/19 under A#)
        - Fx = 15/8 (fixed)

        This gives:
        - A#-D# = 3/2 * 95/108 = 95/72 = 479.9c (same, badly flat fourth)
        - D#-G = 3456/1805 * 2/3 = 2304/1805 = 422.5c (even more sharp Maj 3)
        - Fx-G = 3456/1805 * 8/15 = 9216/9025 = 36.2c (worse unison)
        - F#-A# = 404.4c (same, slightly sharp Maj 3)
        - F#-E = 55296/34295 * 10/9 = 12288/6859 = 1009.4c (sharper, worse but passable dom7th)

        Corrections to be made:
        - Flatten A#: to sharpen the 4th A#-D# towards 4/3
        - Flatten G: to flatten M3 D#-G towards 5/4, and
        - Flatten E: to flatten unison G-Fx, to flatten 7th F#-E towards 7/4

        Since all intervals need to be flattened, but by varying amounts, I'm thinking:
        1. Stack all m3s above C#, reduce m3 size
        2. Stack all m3s under C#, increase m3 size
        3. Find which m3s should go under and which should go above C#, optimize error

        Option 1 (using mediant of 19/16 and 7/6 = 13/11 = 289.2c):
        - C# = 27/20 (fixed)
        - E = 351/220 => F#-E = 39/22 = 991.1c
        - G = 4563/2420 => D#-G = 1521/1210 = 396.0c; Fx-G = 3042/3025 = 9.7c
        - A# = 59319/26620 => A#-D# = 3/2 * 26620/59319 = 26620/19773 = 514.7c; F#-A# = 6591/5324 = 369.6c
        - A# below = 297/260 => A#-D# = 130/99 = 471.6c; F#-A# = 33/26 = 412.7c

        E.g.: Option 1A, A# above:

{r220hz}{r5/4}{r3/2}(1)(bpm:100)
[9/40, 27/80, 59319/106480, 351/440, 1/1, 59319/53240, 3/4, 3/2]
[9/40, 27/80, 59319/106480, 351/440, 1/1, 59319/53240, 27/40, 27/20]
[9/40, 27/80, 59319/106480, 351/440, 1/1, 59319/53240, 9/10, 9/5]
[9/40, 27/80, 59319/106480, 351/440, 1/1, 59319/53240,
351/220]

[9/40, 27/80, 117/220, 351/440, 4563/4840,
117/110, 3/4, 3/2]-
[9/40, 27/80, 117/220, 351/440, 4563/4840,
117/110, 27/40, 27/20]
[9/40, 27/80, 117/220, 351/440, 4563/4840,
117/110, 149/248, 149/124] # (the 149/124 value tempers 13/8 of D#, explanation later)

        E.g.: Option 1B, A# below

{r220hz}{r5/4}{r3/2}(1)(bpm:100)
[9/40, 27/80, 297/520, 351/440, 1/1, 297/260, 3/4, 3/2]
[9/40, 27/80, 297/520, 351/440, 1/1, 297/260, 27/40, 27/20]
[9/40, 27/80, 297/520, 351/440, 1/1, 297/260, 9/10, 9/5]
[9/40, 27/80, 297/520, 351/440, 1/1, 297/260,
351/220]

[9/40, 27/80, 117/220, 351/440, 4563/4840,
117/110, 3/4, 3/2]-
[9/40, 27/80, 117/220, 351/440, 4563/4840,
117/110, 27/40, 27/20]
[9/40, 27/80, 117/220, 351/440, 4563/4840,
117/110, 149/248, 149/124] # (the 149/124 value tempers 13/8 of D#, explanation later)

        The errors for E and G can still afford to be flatter still, but by stacking A# above G, it
        is now drastically too flat, and stack A# under G, it is too sharp.

        Option 2 (using mediant of 19/16 and 6/5 = 25/21 = 301.8c)
        - C# = 27/20
        - A# = 567/500 => A#-D# = 250/189 = 484.2c; F#-A# = 63/50 = 400.1c
        - G = 11907/6250 => D#-G = 3969/3125 = 413.9c; Fx-G = 15876/15625 = 27.5c
        - E = 250047/156250 => F#-E = 27783/15625 = 996.4c

        Option 2 looks a lot more sane (stacking everything under C#, increasing m3 size).

        25/21 is indistinguishably close to 300c, stopping the experiment now to preserve sanity.

        Personally, 25/21 as m3 has a non-complex, sour, turquoise, bitter sound.
        This is the second last time this theme appears.

        Option 1A has a nice texture, but the flat A# sticks out. Option 1B is out of the question.
        Option 2 is doesn't sound like it has the appropriate texture, but is the best one
        (in terms of pitch) so far.

        Let's try to reduce the error of F#-A# in option 1A (sharpening a bit) and see how it compares:

        Mediant of 19/16 and 13/11: 32/27 = 294.1c (3 limit angular):
        - C# = 27/20
        - E = 8/5 => F#-E = 16/9 = 996.1c
        - G = 256/135 => D#-G = 512/405 = 405.8c; Fx-G = 2048/2025 = 19.5c
        - A# = 4096/3645 => A#-D# = 10935/8192 = 500.0c; F#-A# = 8192/6561 = 384.3c

# F#   C#     A#         E         A#
[9/40, 27/80, 2048/3645, 4/5, 1/1, 4096/3645, 3/4, 3/2]
[9/40, 27/80, 2048/3645, 4/5, 1/1, 4096/3645, 27/40, 27/20]
[9/40, 27/80, 2048/3645, 4/5, 1/1, 4096/3645, 9/10, 9/5]
[9/40, 27/80, 2048/3645, 4/5, 1/1, 4096/3645,
8/5]

# F#   C#     A     E    G
[9/40, 27/80, 8/15, 4/5, 128/135,
16/15, 3/4, 3/2]-
[9/40, 27/80, 8/15, 4/5, 128/135,
16/15, 27/40, 27/20]
[9/40, 27/80, 8/15, 4/5, 128/135,
16/15, 149/248, 149/124]

        This is very concordant-sounding for the chord it is and very optimal.
        However, the 3-limitness of this is very apparent, creating a very plain white texture.
        This section should be very colorful.

        Let's sharpen the m3 a tad bit more, bringing F#-A# closer to just:
        (32*2 - 1)/(27*2 - 1) = 63/53 = 299.2c (too sharp)
        (32*3 - 1)/(27*3 - 1) = 19/16 (revert back to monke)
        (32*4 - 1)/(27*4 - 1) = 127/80 (127 is a mersenne prime, 'wrong' color)
        mediant of 19/16 and 32/27 = 51/43 = 295.3c (+1.2c per m3, interesting...)

        Using 51/43 as m3 gen:
        - C# = 27/20
        - E = 1377/860 => F#-E = 153/86 = 997.3c
        - G = 70227/36980 => D#-G = 23409/18490 = 408.3c; Fx-G = 46818/46225 = 22.0c
        - A# = 3581577/3180280 => A#-D# = 496.2c; F#-A# = 397953/318028 = 388.1c

        E.g.:

# F#   C#     A#               E               A#
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 3/4, 3/2]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 27/40, 27/20]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 9/10, 9/5]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280,
1377/860]

# F#   C#     A        E          G
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 3/4, 3/2]-
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 27/40, 27/20]
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 149/248, 149/124]

        I think this is enough deliberation for 3 bars...

        */

        // Stack 51/43 m3s up from C#:
        let e = c_s * r(51, 43);
        let g = e * r(51, 43);
        let a_s = g * r(51, 43);
        let a = e * r(4, 3); // A is 4/3 of E

        /*

        Now we have the issue of deciding what to tune B to (it occurs in mm. 39-40 over A9(13)/F#).

        The 'overtone scale' theme of m. 41 suggests 13/8 for B (w.r.t. the new fundamental D#),
        just as how E is 13/8 of G# and A is 13/8 of C# for the previous two occurences of the
        introductory theme.

        However, unlike the previous two instances, this theme is expanded with Z/2Z (and Z/6Z) symmetry,
        because in beats 3-4 of mm. 42-43, there's a 'tritone sub' utilizing the symmetry of the Fx-C#
        tritone and the C#-G tritone. This means the tuning of mm. 42-43 needs to be decided first
        and the tuning for B in mm. 39-40 should match that.

        For mm. 42-43 There are two options we can take:
        A. Ravel uses two enharmonically distinct spellings for the note, so tune Fx and G distinctly,
           leaving the D#-B interval as 13/8 (i.e. tune Fx as 5/4 of D#, but G as 7/9 of B)
        B. Try to 'temper' it out (specifically temper B)

         */

        /* Here is what we have so far, assuming we take option A:

{r220hz}{r5/4}{r3/2}(1)(bpm:100)
# bar 38
# F#   C#     A#               E          G#   A#
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 3/4, 3/2]--
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 27/40, 27/20]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 9/10, 9/5]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280,
1377/860]

# bar 39
# F#   C#     A        E          G
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 27/40, 27/20]-
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 39/64, 39/32] # B is here as well
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 27/40, 27/20]

# bar 40
# F#   C#     A#               E               A#
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 3/4, 3/2]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 27/40, 27/20]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 9/10, 9/5]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280,
1377/860]

            Using B = 13/8 of D#, doesn't capture the melody well:

# F#   C#     A        E          G
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 3/4, 3/2]-
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 27/40, 27/20]
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 39/64, 39/32]
# jarringly 'sharp' tuning for B here.


# bar 41
(env:1549)
{r220hz} {r5/4} {r9/8} # D# is the new fundamental
[1/4, 3/8, 5/8, 5/4, 3/2, 13/8]---

# bar 42
(env:1654)
[9/4, 5/2, 3/1, 13/4, 4/1]-
[7/4, 5/2, 3/1, 13/4, 4/1]--
[3/2, 5/2, 3/1, 13/4, 4/1]
[7/4, 49/20, 14/5, 13/4, 7/2, 21/5]
[2/1, 49/20, 14/5, 13/4, 7/2, 21/5]
        */

        /*

        I am already leaning towards option 2, because 13/8 for the melody note B in the phrase
        C# B A# at the end of m. 40 resolving into m. 41 is too sharp for fulfilling the melodic function,
        considering that C# is not tuned as 7/4 of D#, so we can't take advantage of C# B A# being part of the
        14th, 13th and 12th harmonics of D# respectively.

        To 'temper' these out, I want to fix the following conditions:
        - Fix: Fx = 5/4 of D#
        - Fix: C# = 7/4 of D# = 5/4 of A
        - Temper: B ~ 13/8 of D# ~ 9/8 of A

        w.r.t D#, B is either 13/8 or 63/40. Mediant: 19/12.

{r220hz}{r5/4}{r3/2}(1)(bpm:100)
[9/40, 27/80, 83349/156250, 250047/312500, 11907/12500,
83349/78125, 3/4, 3/2]-
[9/40, 27/80, 83349/156250, 250047/312500, 11907/12500,
83349/78125, 27/40, 27/20]
[9/40, 27/80, 83349/156250, 250047/312500, 11907/12500,
83349/78125, 19/32, 19/16]

# bar 41
(env:1549)
{r220hz} {r5/4} {r9/8} # D# is the new fundamental
[1/4, 3/8, 5/8, 5/4, 3/2, 19/12]---

# bar 42
(env:1654)
[9/4, 5/2, 3/1, 19/6, 4/1]-
[7/4, 5/2, 3/1, 19/6, 4/1]--
[3/2, 5/2, 3/1, 19/6, 4/1]
[7/4, 49/20, 14/5, 19/6, 7/2, 21/5]
[2/1, 49/20, 14/5, 19/6, 7/2, 21/5]

        However, this sounds rather close to the note A#, making the melodic movement
        from B to A# (-93.6c) unclear.

        Sharpening 19/12: (38-1)/(24-1) = 37/23 = 823c; A#-B = 121.1c (skewed too sharp)
        Flattening 37/23: (37*2 + 1)/(23*2 + 1) = 75/47 = 809c; A#-B = 107.1c

{r220hz}{r5/4}{r3/2}(1)(bpm:100)
# bar 40
[9/40, 27/80, 567/1000, 250047/312500, 1/1, 567/500, 3/4, 3/2]
[9/40, 27/80, 567/1000, 250047/312500, 1/1, 567/500, 27/40, 27/20]
[9/40, 27/80, 567/1000, 250047/312500, 1/1, 567/500, 9/10, 9/5]
[9/40, 27/80, 567/1000, 250047/312500, 1/1, 567/500,
250047/156250]

[9/40, 27/80, 83349/156250, 250047/312500, 11907/12500,
83349/78125, 3/4, 3/2]-
[9/40, 27/80, 83349/156250, 250047/312500, 11907/12500,
83349/78125, 27/40, 27/20]
[9/40, 27/80, 83349/156250, 250047/312500, 11907/12500,
83349/78125, 225/376, 225/188]

# bar 41
(env:1549)
{r220hz} {r5/4} {r9/8} # D# is the new fundamental
[1/4, 3/8, 5/8, 5/4, 3/2, 75/47]---

# bar 42
(env:1657)
[9/4, 5/2, 3/1, 150/47, 4/1]-
[7/4, 5/2, 3/1, 150/47, 4/1]--
[3/2, 5/2, 3/1, 150/47, 4/1]
[7/4, 49/20, 14/5, 150/47, 7/2, 21/5]
[2/1, 49/20, 14/5, 150/47, 7/2, 21/5]

        This is a grating sound, (plain 3x47-limit sharp knife feeling).

        Sharpening a tad bit more:
        (75*2-1)/(47*2-1) = 149/93 = 816.0c; A#-B = 114.1c

{r220hz}{r5/4}{r3/2}(1)(bpm:100)
# bar 40
[9/40, 27/80, 567/1000, 250047/312500, 1/1, 567/500, 3/4, 3/2]
[9/40, 27/80, 567/1000, 250047/312500, 1/1, 567/500, 27/40, 27/20]
[9/40, 27/80, 567/1000, 250047/312500, 1/1, 567/500, 9/10, 9/5]
[9/40, 27/80, 567/1000, 250047/312500, 1/1, 567/500,
250047/156250]

[9/40, 27/80, 83349/156250, 250047/312500, 11907/12500,
83349/78125, 3/4, 3/2]-
[9/40, 27/80, 83349/156250, 250047/312500, 11907/12500,
83349/78125, 27/40, 27/20]
[9/40, 27/80, 83349/156250, 250047/312500, 11907/12500,
83349/78125, 149/248, 149/124]

# bar 41
(env:1549)
{r220hz} {r5/4} {r9/8} # D# is the new fundamental
[1/4, 3/8, 5/8, 5/4, 3/2, 149/93]---

# bar 42
(env:1657)
[9/4, 5/2, 3/1, 298/93, 4/1]-
[7/4, 5/2, 3/1, 298/93, 4/1]--
[3/2, 5/2, 3/1, 298/93, 4/1]
[7/4, 49/20, 14/5, 298/93, 7/2, 21/5]
[2/1, 49/20, 14/5, 298/93, 7/2, 21/5]

        We have prime factors 149 (undulating off-white), and 31x3 (incandescent orange)

        I like this sound.

         */

        let b = d_s * r(149, 93); // 149/93 w.r.t. D#

        // Finally ready to tune m. 38
        t.push(td(124.045, 4, r(5, 4), [
            c_s, P, P, e,
            P, f_s, g, P,
            a, a_s, b, P,
        ]));

        /*

        Final tuning for mm. 38-40:

{r220hz}{r5/4}{r3/2}(1)(bpm:100)

# bar 38
# F#   C#     A#               E          G#   A#
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 3/4, 3/2]--
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 27/40, 27/20]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 9/10, 9/5]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280,
1377/860]

# bar 39
# F#   C#     A        E          G
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 27/40, 27/20]-
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 149/248, 149/124]
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 27/40, 27/20]

# bar 40
# F#   C#     A#               E              A#
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 3/4, 3/2]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 27/40, 27/20]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280, 9/10, 9/5]
[9/40, 27/80, 3581577/6360560, 1377/1720, 1/1, 3581577/3180280,
1377/860]

# F#   C#     A        E          G
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 3/4, 3/2]-
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 27/40, 27/20]
[9/40, 27/80, 459/860, 1377/1720, 70227/73960,
459/430, 149/248, 149/124]

         */

        // ------------------------------------------------------------
        // PAGE 5

        // Bar 41: D# harmonic, B is a 'tempered' 13th harmonic (149/93)
        //
        // New fundamental root: D#.

        let f_x = d_s * r(5, 4);
        let e_s = d_s * r(9, 8);
        let c_s = d_s * r(7, 8);
        let a_s = d_s * r(3, 2);

        // B still remains as the tempered 13th harmonic.
        assert!(b == d_s * r(149, 93));

        t.push(td(133.852, 4, r(5, 4), [
            c_s, P, P, P,
            e_s, P, f_x, P,
            P, a_s, P, P,
        ]));

        /*
# bar 41
(env:1549)
{r220hz} {r5/4} {r9/8} # D# is the new fundamental
[1/4, 3/8, 5/8, 5/4, 3/2, 149/93]---

# bar 42 & 43
(env:1657)
[9/4, 5/2, 3/1, 298/93, 4/1]-
[7/4, 5/2, 3/1, 298/93, 4/1]--
[3/2, 5/2, 3/1, 298/93, 4/1]
[7/4, 49/20, 14/5, 298/93, 7/2, 21/5]
[2/1, 49/20, 14/5, 298/93, 7/2, 21/5]

         */

        // Bar 42:4: A9#11

        /*
        Fix C# as 7/4 of D#.
        Let A be 4/5 of C#, G be 7/8 of A.
        B remains as the 'tempered 9/8' of A.
         */
        let a = c_s * r(8, 5);
        let g = a * r(7, 8);

        t.push(td(141.763, 4, r(5, 4), [
            P, P, P, P,
            P, P, g, P,
            a, P, P, P,
        ]));

        // Bar 43: reset to D# harmonic

        // Only difference is Fx instead of G.
        t.push(td(142.729, 4, r(5, 4), [
            P, P, P, P,
            P, P, f_x, P,
            P, P, P, P,
        ]));

        // Bar 43:4: A9#11

        t.push(td(145.547, 4, r(5, 4), [
            P, P, P, P,
            P, P, g, P,
            P, P, P, P,
        ]));

        // Bar 44: D# harmonic stuff, romantic flourishes on beat 2

        t.push(td(146.523, 4, r(5, 4), [
            P, P, P, P,
            P, P, f_x, P,
            P, P, P, P,
        ]));

        // On beat 2 (flourish), the original notes are A#, B, B#, C#, D#, Dx, A#, Fx, E#, C#, A#
        // etc...
        // Technically the note C# should function as a 7/4, but it would be interesting to hear the
        // chromatic A#, B, B#, C#, D#, Dx as part of the otonal stack 12:13:14:15:16:17,
        // and beat 3: E#, F#, Fx, G#, Gx, A# = 18:19:20:21:22:24

        let b_s = d_s * r(14, 8);
        let c_s = d_s * r(15, 16);
        let d_x = d_s * r(17, 16);
        let f_s = d_s * r(19, 16);
        let g_s = d_s * r(21, 16);
        let g_x = d_s * r(22, 16);

        // Only activate this tuning on beat 2, otherwise the carried over notes will change tuning weirdly.
        t.push(td(147.502, 4, r(5, 4), [
            c_s, P, P, d_x,
            P, f_s, P, g_s,
            g_x, P, P, b_s,
        ]));

        // Bar 44:2.5: reset C# to 7/4, otherwise the phrase (D#9) on beat 2.5 sounds weird
        // with a maj 7th.
        let c_s = d_s * r(7, 8);
        t.push(td(148.290, 4, r(5, 4), [
            c_s, P, P, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        /*

# bar 44 (play from halfway if synth polyphony exceeded)
{r220hz}{r5/4}{r9/8}(1)(bpm:100)
(env:1656)
[9/4, 5/2, 3/1, 298/93, 4/1]
[2/1, 5/2, 3/1, 298/93, 4/1]
(env:1657)(6)(bpm:100)
[3/1, 3/1] 13/4 14/4 15/4 16/4 17/4
(5)[5/2, 6/1] 5/1 9/2 7/2 6/2
(7)[9/4, 9/4] 19/8 20/8 21/8 22/8 3/1 7/2
(5) 9/2 8/2 7/2 6/2 5/2
[2/1, 2/1] 9/4 7/4 5/4 9/8
[3/2, 1/1] 7/8 6/8 5/8 9/16

         */

        // bar 45: C!7(13), tritone symmetry Gb9(13)

        // This is the first introduction of the theme for
        // "Mon pere bat l'eau coassante d'une brache d'aulne verte"
        // "My father beats the splashing water with a green alder branch"

        // We can continue the tempered 13/8 idea in the past few bars here, but since this is a
        // new theme, I want to consider the function of this theme and where it appears later, so
        // here's a big picture overview of the coming sections:

        // First appearance in C! (m. 45):
        // C!7(13)       F#9(13)
        // G Ab Bb - - C Eb Db

        // Second appearance in A! (m. 50):
        // A!7(13)     A7b5#9
        // E F G - - A C Bb

        // Interlude in Dm, D7#9. (maybe feature some under-19 neji for the frosty sound that /19 gives)
        // "sa chanson hurhuree, ..." (development of Ondine theme in different tonality)

        // Third appearance in C#! (m. 57), slightly different harmony
        // C#13b9 C#m13b5 G7(13)
        // G# A# B - - C# E D

        // Fourth appearance in Bb! (m. 60)
        // Bb9(13)       E7(13)
        // F G Ab - - Bb C# B

        /*
        The first two occurrences differ from the last two in the second note (m2 vs M2)

        My initial idea to that, since these chords all occur in the 'otonal' context,
        why not map the 6th of the chord root (both the former b6 and latter natural 6)
        to the 13th harmonic, 'tempering' out 6 and b6 in the melody?

        The third appearance have different harmonic structure though. Since the interlude only
        makes sense if we consider all the 12edo chroma basis commas (aug, dim, fifth symmetries)
        at once, the third appearance should still be in NEJI.
         */

        let e = e_s * r(15, 16); // Resolve Fa-Mi in 5-limit, use E# to anchor 'Fa'
        let c = e * r(8, 5); // Chord root C is now 243/256 of original starting note.
        assert!(c == r(243, 128));
        let g = c * r(3, 4);
        let a_b = c * r(13, 16); // 13th harmonic for b6.
        let b_b = c * r(7, 8); // 7th harmonic

        t.push(td(150.850, 4, r(5, 4), [
            P, P, P, e,
            P, P, g, a_b,
            P, b_b, P, c,
        ]));

        // Bar 45:4: Gb9(13)

        // Aiming for the colors of prime 13 here, very very dark blue colors.
        // This makes the intervals way wider than normal though, the melody now leaps
        // a neutral 3rd instead of a minor 3rd.
        //
        // Changing the function of these notes a lot, but it somehow sounds grander.

        let d_b = a_b * r(2, 3); // Db: 5th below the 13 harmonic
        let g_b = d_b * r(4, 3); // Gb: P4 from Db
        let b_b = g_b * r(5, 4); // Bb: 5 lim 3rd from Gb.

        t.push(td(153.880, 4, r(5, 4), [
            d_b, P, P, P,
            P, g_b, P, P,
            P, b_b, P, P,
        ]));

        /*
{r220hz}{r5/4}{r9/8}(1)(bpm:100)
(5)
[2/1, 2/1] 9/4 7/4 5/4 9/8
[3/2, 1/1] 7/8 6/8 5/8 9/16

# bar 45
{r220hz}{r5/4}{r243/256}(1)
[1/2, 3/4, 5/4, 3/2, 13/8]
[1/2, 13/16, 5/4, 3/2, 13/8]
[1/2, 7/8, 5/4, 3/2, 13/8]--
[1/2, 1/1, 5/4, 3/2, 13/8]
[26/36, 39/32, 5/4, 13/8, 65/36, 39/16]
[26/36, 13/12, 5/4, 13/8, 65/36, 13/6]
         */

        // ------------------------------------------------------------
        // PAGE 6

        // Bar 47: reset tuning to C!7

        // This part also alternates between Gb7 and C!, but the Gb7 is plain without the
        // 9th or 13th, so it's safe to build from the 7/4 and use the septimal color
        // instead of tridecimal (this phrase refers to the 'sisters', rather than the
        // 'father')

        // This settles the tuning till the 2nd flourish at m. 49:2

        let b_b = c * r(7, 8); // Bb: reset to 7th harm of C.
        let d = c * r(9, 16); // D: 9/8 of C (this wasn't set yet)
        let g_b = b_b * r(4, 5); // Gb-Bb forms 5-lim third (?)
        let d_b = g_b * r(3, 4); // Db-Gb forms 4/3 (?)

        t.push(td(158.49, 4, r(5, 4), [
            d_b, d, P, P,
            P, g_b, P, P,
            P, b_b, P, P,
        ]));

        // Bar 49:2: augmented flourish

        // The flourish starts with D aug and G aug triads
        // "de nenuphars et de glaieuls, ou se moquent du saule caduc et barbu qui peche a la ligne"
        // color of hysteria and mockery: use 11:14:18 triads.

        // fix D as 9/8 of C, build D-F#-Bb = 9:11:14
        // fix G as 3/2 of C, build G-B-D# = 9:11:14

        let f_s = d * r(11, 9); // D-F#-Bb forms 9:11:14 (D and Bb already in position)
        let b = g * r(11, 9);
        let d_s = b * r(14, 22); // D#: 14/11 of B

        t.push(td(167.437, 4, r(5, 4), [
            P, P, d_s, P,
            P, f_s, P, P,
            P, P, b, P,
        ]));

        // Bar 49:3:4/13: F# triad over Gm

        // Aiming for 11 color for F#.
        // Fix A# = Bb = 7/4 of C, but let F# be 11/8 of C and C# = 3/2 of F#.

        let f_s = c * r(11, 16);
        let c_s = f_s * r(3, 4);

        t.push(td(168.850, 4, r(5, 4), [
            c_s, P, P, P,
            P, f_s, P, P,
            P, P, P, P,
        ]));


        // Bar 50: A! and Eb9(13). Second iteration of 'father' theme.

        // A = 3/4 of D from last note of m. 49, so A = 6561/8192 of original starting C#.
        // From Eb, we moved 2 3-lim min 3rds from Eb to C to A. (3 lim m3rds preserve
        // some semblance of familiarity in that key centers are traditionally recognized as
        // built from fifths)

        // Build otonally from A.

        let a = d * r(3, 2); // 6561/8192 of original C#.
        assert!(a == r(6561, 4096));
        let c_s = a * r(5, 8);
        let e = a * r(3, 4);
        let f = a * r(13, 16);
        let g = a * r(7, 8);

        t.push(td(170.95, 4, r(5, 4), [
            c_s, P, P, e,
            f, P, g, P,
            a, P, P, P,
        ]));

        // On beat 4, since the root stays at A, instead of the wide 13-stuff,
        // fix C# = Db, let Db-Eb be 8/7 (so Eb is Euler's tritone 10/7 from A),
        // build overtones from Eb.

        let e_b = c_s * r(8, 7); // 10/7 from A
        let g = e_b * r(5, 4);
        let b_b = e_b * r(3, 2);
        let f = e_b * r(9, 8);
        // I may have played an extra Ab intentionally to add 11/8 color.
        let a_b = e_b * r(11, 8);
        let c = e_b * r(13, 8);

        t.push(td(174.01, 4, r(5, 4), [
            P, P, e_b, P,
            f, P, g, a_b,
            P, b_b, P, c,
        ]));

        // Bar 51: revert to A!13

        let c_s = a * r(5, 8);
        let e = a * r(3, 4);
        let f = a * r(13, 16);
        let g = a * r(7, 8);

        t.push(td(175.62, 4, r(5, 4), [
            c_s, P, P, e,
            f, P, g, P,
            a, P, P, P,
        ]));

        // Bar 52: 'interlude section' in A7, Dm6, Am7b5, Eb7#11
        // heavy use of all of 12 edo's commas all over the place in this section,
        // use under-(19*2) NEJI for 'frosty' color.

        /*
        12 NEJI /19
        D  0\12:  19/19  0c
        D# 1\12:  20/19  88.8c
        E  2\12:  43/38  214.0c
        F  3\12:  45/38  292.7c
        F# 4\12:  24/19  404.4c
        G  5\12:  51/38  509.3c
        G# 6\12:  27/19  608.4c
        A  7\12:  3/2    701.9c  (non NEJI P5)
        Bb 8\12:  60/38  790.7c
        B  9\12:  64/38  902.5c
        C  10\12: 34/19  1007.4c
        C# 11\12: 72/38  1106.4c
         */

        // root the NEJI in D (4/3 of A)
        let d = a * r(2, 3);
        let d_s = d * r(20, 19);
        let e = d * r(43, 38);
        let f = d * r(45, 38);
        let f_s = d * r(24, 19);
        let g = d * r(51, 38);
        let g_s = d * r(27, 19);
        let b_b = d * r(60, 38);
        let b = d * r(64, 38);
        let c = d * r(34, 19);
        let c_s = d * r(36, 38);

        t.push(td(179.42, 4, r(5, 4), [
            c_s, d, d_s, e,
            f, f_s, g, g_s,
            P, b_b, b, c,
        ]));

        // This NEJI works well till the end of m. 56 (before the appoggiatura in m. 57)

        // ------------------------------------------------------------
        // PAGE 7

        // Bar 57: third 'father waves' theme, this time more emphasis on the waves.
        // Keep same NEJI (the tritone sub use is more apparent in the third iteration of
        // this theme than the others), but melodically use 13/8 for the 6 (A#) to keep
        // 13/8 tempered b6 and nat 6 theme.

        // After trying out 13/8, it was too drastically flat, so instead,
        // use the mediant of 13/8 and 5/3 = 18/11

        // We use the NEJI's C# as the root (C# = 18/19 of D = 4/3 of A)
        let a_s = c_s * r(18, 11);

        // But the !13 sound only works well if the lower primes are tuned properly.
        // Even though the 13 is tempered to 18/11 here, I still want a sour harmonic sound:

        let b = c_s * r(7, 4); // B: 7th harm of C# (this is the important one)

        // not so important
        // let e_s = c_s * r(5, 4); // E#: 5th harm of C#
        // let g_s = c_s * r(3, 2); // G#: 3rd harm of C#
        // let e = b * r(2, 3); // make E-B a 3-limit P5
        // let g = e * r(7, 6); // septimal color for the Em triad.

        t.push(td(194.05, 4, r(5, 4), [
            P, P, P, P,
            P, P, P, P,
            P, a_s, b, P,
        ]));


        // ------------------------------------------------------------
        // PAGE 8

        // Bar 60: Bb! and E9(13)

        // The right hand part should evoke hysteria and mockery, left hand should be sour.

        // For Bb! (beats 1-3), build otonally from Bb.

        // Now, the tuning is anchored by the 13-limit A# as defined for m. 57.
        //
        // Bb = A# = 18/11 of C# = 18/19 of D = 4/3 of A = 6561/8192 of starting C#
        // Bb should be equal to 177147/107008 of the starting C#. If it's not then... gg.
        //
        // Is this too far of a stretch?

        assert!(a_s == r(177147, 107008)); // rabak

        let b = a_s * r(17, 16);
        let c = a_s * r(9, 8);
        let c_s = a_s * r(19, 32);
        let d = a_s * r(5, 8);
        let e_b = a_s * r(21, 32); // otonal 4th instead of 3-lim P4
        let e = a_s * r(11, 16);
        let f = a_s * r(3, 4);
        let g_b = a_s * r(4, 5); // mediant of 13/8 and 3/2, extremely clashy between 5, b6 and 6.
        let g = a_s * r(13, 16); // in order for melody theme's 6th to be 13/8
        let a_b = a_s * r(7, 8);
        let a = a_s * r(15, 16);

        t.push(td(206.90, 4, r(5, 4), [
            c_s, d, e_b, e,
            f, g_b, g, a_b,
            a, P, b, c,
        ]));

        // Bar 60:4: E9(13) temporal concordance, high-limit heavy comma shift
        //
        // Using the neutral third motif as in the 1st iteration of this theme.

        // Fix C# as 19/16 of Bb.
        // E is the new root (based on C# = 13/8 of E)
        let e = c_s * r(16, 13);

        // Build otonally from E.
        let d = e * r(7, 8);
        let g_s = e * r(5, 4);
        let b = e * r(3, 2);
        let f_s = e * r(9, 8);

        t.push(td(210.62, 4, r(5, 4), [
            P, d, P, P,
            P, f_s, P, g_s,
            P, P, b, P,
        ]));

        // Bar 61: Reset to Bb!19 = A# = 177147/107008 of starting C#.

        let e = a_s * r(11, 16);
        let d = a_s * r(5, 8);
        let g_s = a_s * r(7, 8);
        let b = a_s * r(17, 16);

        t.push(td(212.2, 4, r(5, 4), [
            P, d, P, e,
            P, g_b, P, g_s,
            P, P, b, P,
        ]));

        // Bar 62: 5-limit E#m7/G# (notes here are all very low, keep things simple)

        // E# Fx G# A# B# (E# min over G#-D# bass pedal)

        // Tune this w.r.t G# 5-limit major (G# scale in melody)

        // The first melody note is D#, simple 3-limit key relation with A#.
        let d_s = a_s * r(2, 3);
        assert!(d_s == r(59049, 53504));
        let g_s = d_s * r(4, 3); // G# is current key root
        assert!(g_s == r(19683, 13376)); // new key root.

        let c_s = g_s * r(2, 3);
        let e_s = g_s * r(5, 6);
        let f_x = g_s * r(15, 16);
        let b_s = g_s * r(5, 4);
        assert!(a_s == g_s * r(9, 8)); // A# is the anchor note.

        t.push(td(215.19, 4, r(5, 4), [
            c_s, P, d_s, P,
            e_s, P, f_x, g_s,
            P, a_s, P, b_s,
        ]));

        // Bar 63: F#m

        // Increase complexity, use 7-lim subminor.

        // Anchor tuning using G# as 9/8 of new root F#.
        // Build primodally under 6 over F# (so B is harmonic fundamental)
        let f_s = g_s * r(8, 9); // 3-lim key relation.
        assert!(f_s == r(2187, 1672)); // New root
        let a = f_s * r(7, 6); // 7-lim sub min
        let b = f_s * r(4, 3);
        let c_s = f_s * r(3, 4);
        let d_s = f_s * r(5, 6);
        let e_s = f_s * r(11, 12); // 11th harm of B

        t.push(td(218.75, 4, r(5, 4), [
            c_s, P, d_s, P,
            e_s, f_s, P, P,
            a, P, b, P,
        ]));

        // ------------------------------------------------------------
        // PAGE 9

        // Bar 64: E melodic min

        // Build intensity using 13/11 minor sonority (build under 11)

        // Anchor using B = 3/2 of E
        // New root (notice how the relation from starting fundamental is simplifying)
        let e = b * r(2, 3);
        assert!(e == r(243, 209));
        let f_s = e * r(12, 11); // lesser undecimal neutral second to build /11
        let g = e * r(13, 11);
        let a = e * r(4, 3); // use 3-lim for perfect ratios
        let c_s = e * r(37, 44); // 900.0c maj 6th
        let d_s = e * r(21, 22); // 1119.4c maj 7th

        t.push(td(221.5, 4, r(5, 4), [
            c_s, P, d_s, e,
            P, f_s, g, P,
            a, P, P, P,
        ]));

        // Bar 65: Grand C# harmonic (map nat 6 to 13/8)

        // Pump comma using very close to 12edo movement: anchor C#

        assert!(c_s == r(8991, 9196)); // -39.0c flatter than the start
                                       // post-climax should find a way to pitch drift upward.

        let d_s = c_s * r(9, 8);
        let e_s = c_s * r(5, 4);
        let f_s = c_s * r(21, 16); // there shouldn't be an F#, but in case it was accidentally played...
        let g_s = c_s * r(3, 2);
        let a_s = c_s * r(13, 8);
        let b = c_s * r(7, 4);

        t.push(td(224.3, 4, r(5, 4), [
            P, P, d_s, P,
            e_s, f_s, P, g_s,
            P, a_s, P, b,
        ]));

        // BAR 66: GIANT STEPS (this bar was 90% of the reason of why I wanted to do this whole thing.)

        /*
        Sa chanson murmuree, elle me supplia de recevoir son anneau a mon doigt, pour etre l'epoux d'une Ondine,
        et de visiter avec elle son palais, pour etre le roi des lacs.
         */

        // Chords: B-9 D9(13) | G-9 Bb9(13) | D#-9 - | F#13sus F#7b9

        /*
        Plan: the minor chords should have subminor color, so start with
        B-D-F# = 6:7:9, and we stack these 7:9 thirds, of which in 12edo, the octave has
        Z/3Z symmetry, but here we do not, so we decsend the commas.

        The dom9(13) chords should have 13 mapped to 13/8
        (since 13/8 represents the 'backdrop' of the ocean (in the shimmering intro etc...))

        So we have an interesting harmonic cycle:

        B-9         D7(13)        G-9          Bb7(13)      D#-9          F#13sus        F#7b9         B-9 (next cycle)
        B  1/1 --> 13/8 of D <>                                                                        2/3 of F#
        D  7/6 ==========anchor============--> 5/4 of Bb                                               7/6 of B ========...
        F# 3/2 --> 5/4 of D                                 7/6 of D# ==========anchor================================
        C# 9/8 <>                                                                                      9/8 of B
        C          7/8 of D <>                 9/8 of Bb <>
        E          9/8 of D <>                                            7/8 of F#
        G                         3/4 of D --> 13/8 of Bb                              17/16 of F#
        Bb                        7/6 of G ==========anchor===========--> 5/4 of F#
        A                         9/8 of G <>
        Ab                                     7/8 of Bb <>               9/8 of F#
        D#                                                  2/3 of Bb --> 13/8 of F#
        E#                                                  9/8 of D# <>

        In one cycle (from B- to the next B-) we have a comma drift of:
        (7/6 * 2/3)^3 * 2 = 686/729 = -105.25c.

        This harmonic cycle goes on for 2 bars, meaning that by the time we're done (bar 68), we would have
        shifted -210.5c on top of the -39.0c (plus a bit left over from the 9/8 relation of B-C#).
         */

        /*
        Attempt 2.
        The subminor thirds have shifted the comma wayyy too far, which completely messes up the melody.

        Instead of 7/6 thirds, let's try a high prime limit third that is close to exactly 300c, but
        preferably still slightly flatter, so that the descending feeling is still there.

        med(7/6, 6/5) = 13/11 = 289.2c
        med(13/11, 6/5) = 19/16 = 297.5c
        med(19/16, 6/5) = 25/21 = 301.8c
        med(25/21, 19/16) = 44/37 = 299.97c (too close to 300)
        med(44/37, 19/16) = 63/53 = 299.2c
        med(63/53, 19/16) = 82/69 = 298.8c
         */

        // adjust m3 size for the upward m3 bass movement and m3 interval of minor tonic chords.
        let b66_m3_size = r(82, 69);

        // adjust nat 6 size for the melody over the dom7(13) chords
        // (the ideal 13/8 is too astringent, losing the symmetry of the downward descending melody)
        //
        // To match 12edo commas exactly, the nat6_size should be a tempered P5 plus X,
        // where 2X is the interval spanned by going up a just 3/2 fifth and down one b66_m3_size.
        // If b66_m3_size is 82/69, then 2X = 207/164, and X ~ 46/41.
        //
        // med(184/164, 13/12) = 59/52.
        // 3/2 * 59/52 = 177/104 = 920.5c, quite a fair compromise
        let b66_nat6_size = r(177, 104);

        // B-9
        // Anchor C# as 9/8 of B, set B to new root:
        let b = c_s * r(16, 9);
        let d = b * b66_m3_size * r(1, 2);
        let f_s = b * r(3, 4);

        t.push(td(228.1, 4, r(5, 4), [
            P, d, P, P,
            P, f_s, P, P,
            P, P, b, P,
        ]));

        // D7(13) (anchor D)
        let b = d * b66_nat6_size;
        let f_s = d * r(5, 4);
        let c = d * r(7, 4);
        let e = d * r(9, 8);
        t.push(td(229.36, 4, r(5, 4), [
            P, P, P, e,
            P, f_s, P, P,
            P, P, b, c,
        ]));

        // G-9 (anchor D)
        let g = d * r(4, 3);
        let b_b = g * b66_m3_size;
        let a = g * r(9, 8);

        t.push(td(230.2, 4, r(5, 4), [
            P, P, P, P,
            P, P, g, P,
            a, b_b, P, P,
        ]));

        // Bb7(13) (anchor Bb)
        let a_b = b_b * r(7, 8);
        let d = b_b * r(5, 8);
        let f = b_b * r(3, 4);
        let g = b_b * b66_nat6_size * r(1, 2);

        t.push(td(230.95, 4, r(5, 4), [
            P, d, P, P,
            f, P, g, a_b,
            P, P, P, P
        ]));

        // Eb-9 (anchor Bb)
        let e_b = b_b * r(2, 3);
        let g_b = e_b * b66_m3_size;
        let f = e_b * r(9, 8);

        // in case of accidental wrong notes
        let a_b = e_b * r(4, 3);
        let d_b = g_b * r(3, 4);

        t.push(td(231.69, 4, r(5, 4), [
            d_b, P, e_b, P,
            f, g_b, P, a_b,
            P, P, P, P,
        ]));

        // F#13sus (anchor A# = Bb) and F#7b9
        let a_s = b_b;
        let f_s = a_s * r(4, 5);
        let e = f_s * r(7, 8);
        let g_s = f_s * r(9, 8);
        let d_s = f_s * b66_nat6_size * r(1, 2); // TODO: for melody's sake, should this be 13th harm or 27/16?
        let g = f_s * r(17, 16); // TODO: is this the correct color for the b9?

        t.push(td(233.05, 4, r(5, 4), [
            P, P, d_s, e,
            P, f_s, g, g_s,
            P, a_s, P, P,
        ]));

        // Bar 67: SECOND CYCLE

        // B-9 (anchor F#)
        let b = f_s * r(4, 3);
        let d = b * b66_m3_size * r(1, 2);
        let f_s = b * r(3, 4);

        t.push(td(234.34, 4, r(5, 4), [
            P, d, P, P,
            P, f_s, P, P,
            P, P, b, P,
        ]));

        // D7(13) (anchor D)
        let b = d * b66_nat6_size;
        let f_s = d * r(5, 4);
        let c = d * r(7, 4);
        let e = d * r(9, 8);
        t.push(td(235.05 , 4, r(5, 4), [
            P, P, P, e,
            P, f_s, P, P,
            P, P, b, c,
        ]));

        // G-9 (anchor D)
        let g = d * r(4, 3);
        let b_b = g * b66_m3_size;
        let a = g * r(9, 8);

        t.push(td(235.75 , 4, r(5, 4), [
            P, P, P, P,
            P, P, g, P,
            a, b_b, P, P,
        ]));

        // Bb7(13) (anchor Bb)
        let a_b = b_b * r(7, 8);
        let d = b_b * r(5, 8);
        let f = b_b * r(3, 4);
        let g = b_b * b66_nat6_size * r(1, 2);

        t.push(td(236.50, 4, r(5, 4), [
            P, d, P, P,
            f, P, g, a_b,
            P, P, P, P
        ]));

        // Eb-9 (anchor Bb)
        let e_b = b_b * r(2, 3);
        let g_b = e_b * b66_m3_size;
        let f = e_b * r(9, 8);

        // in case of accidental wrong notes
        let a_b = e_b * r(4, 3);
        let d_b = g_b * r(3, 4);

        t.push(td(237.31, 4, r(5, 4), [
            d_b, P, e_b, P,
            f, g_b, P, a_b,
            P, P, P, P,
        ]));

        // F#13sus (anchor A# = Bb) and F#7b9
        let a_s = b_b;
        let f_s = a_s * r(4, 5);
        let e = f_s * r(7, 8);
        let g_s = f_s * r(9, 8);
        let d_s = f_s * b66_nat6_size * r(1, 2); // TODO: for melody's sake, should this be 13th harm or 27/16?
        let g = f_s * r(17, 16); // TODO: is this the correct color for the b9?

        // we need to temper the A# closer toward 11/12 of B so that bar 68 is not jarring.
        // The original ratio between A# and B is 16/15, but m. 68 fixes 12/11 for A#-B.
        // med(16/15, 12/11) = 14/13 (still to jarring of a change)
        // med(16/15, 14/13) = 15/14
        let temp_a_s = b * r(14, 15);

        t.push(td(238.76, 4, r(5, 4), [
            P, P, d_s, e,
            P, f_s, g, g_s,
            P, temp_a_s, P, P,
        ]));

        // Bar 68: B-6/9

        // Build /6 subminor (anchor F#)
        let b = f_s * r(4, 3);
        let c_s = b * r(9, 16);
        let d = b * r(7, 12);
        let e = b * r(2, 3);
        let g_s = b * r(5, 6);
        let a_s = b * r(13, 14); // goal: A#-B = 12/11, but temper for now.

        t.push(td(240.29, 4, r(5, 4), [
            c_s, d, P, e,
            P, P, P, g_s,
            P, a_s, b, P,
        ]));

        // Bar 69: B-6/9 (untempered 11th harmonic mapping for nat 7 A#)
        let a_s = b * r(11, 12);

        t.push(td(242.31, 4, r(5, 4), [
            P, P, P, P,
            P, P, P, P,
            P, a_s, P, P,
        ]));

        // ------------------------------------------------------------
        // PAGE 10

        // Bar 72: Am, Cmaj gliss.
        // Map P4 to 11/8 for shimmer.
        // The note A should be the resolution for the previous chord, Bm6/9.
        // The Bm6/9 is identical to E7(13) over B, so the G#->A semitone movement
        // is the typical cantizans 7-1 resolution we can do in 5-limit.

        /*
        Et comme je lui repondais que j'aimais une mortelle (mm. 72-79)
         */

        let a = g_s * r(16, 15);
        let c = a * r(6, 5); // this is the root we are building off of.

        // println!("C: {c}");

        // FYI, after all the ridiculous comma pumps, we are at
        // C = 109631931154432/58625675076375 above the initial C#
        //   = (2^20 * 37 * 41^4) / (3^6 * 5^3 * 11^2 * 19 * 23^4)
        // This note is equal to -116.3c below starting C#, so we aren't far off.

        let d = c * r(9, 16);
        let e = c * r(5, 8);
        let f = c * r(11, 16);
        let g = c * r(3, 4);
        let b = c * r(15, 16);

        t.push(td(258.30, 4, r(5, 4), [
            P, d, P, e,
            f, P, g, P,
            a, P, b, c,
        ]));

        // Bar 74: F# maj pentatonic.

        // This part should sound very human, grounding, non-mystical and familiar.
        //
        // Use a 12-NEJI under /54 rooted at B = 54/54 for a relatively plain, familiar 12edo sound.
        // Use B as root since key is B (though the tonal center leans closer towards F#)
        //
        // also, reset C# to the starting pitch to 'reset' the hallucination

        let b = r(54, 61);        // 54/54  0.0c
        let c = b * r(57, 54);    // 57/54  93.6c
        let c_s = r(1, 1);        // 61/54  211.0c
        let d = b * r(64, 54);    // 64/54  294.1c
        let d_s = b * r(68, 54);  // 68/54  399.0c
        let e = b * r(72, 54);    // 72/54  498.0c
        let f = b * r(76, 54);    // 76/54  591.6c
        let f_s = b * r(81, 54);  // 81/54  701.9c
        let g = b * r(86, 54);    // 86/54  805.6c
        let g_s = b * r(91, 54);  // 91/54  903.4c
        let a = b * r(96, 54);    // 96/54  996.1c
        let a_s = b * r(102, 54); // 102/54 1101.0c

        // B and C have to be listed in the octave above C#
        let b = b * 2;
        let c = c * 2;

        t.push(td(271.7, 4, r(5, 4), [
            c_s, d, d_s, e,
            f, f_s, g, g_s,
            a, a_s, b, c,
        ]));

        // ------------------------------------------------------------
        // PAGE 11

        // Bar 79: D#!9

        /*
        ... boudeuse et depitee, elle pleura quelques larmes,
         */

        // Going back to the Ondine character, using otonal stuff again
        // build off D# from the NEJI. (D# = 68/61 from 1/1 C# = 188.1c)

        let f = d_s * r(9, 8);
        let g = d_s * r(5, 4);
        let a_s = d_s * r(3, 2);
        let c_s = d_s * r(7, 8);

        let g_s = d_s * r(4, 3); // pre-tune G# as 4/3 of D# so the detune effect is not so bad.
        let b = g_s * r(7, 6); // pretude B: septimal m3 also

        t.push(td(292.06, 4, r(5, 4), [
            c_s, P, P, P,
            f, P, g, g_s,
            P, a_s, b, P,
        ]));

        // Bar 80: G#m9(13)

        // reintroduce 13/8 and septimal min third
        // use G# = 4/3 of D# as new chord root.
        assert!(a_s == g_s * r(9, 8)); // A# maintains 9/8 of G#
        let e_s = g_s * r(13, 16); // E#: nat 6 becomes 13th harmonic.
        let f_s = g_s * r(7, 8); // F#: also septimal, P5 from B.

        // the detuning of F to E# is quite drastically noticeable...
        // pretune the unused notes G# and B in the previous tuning, then
        // hold off the tuning of E# until just before it happens.

        // Delay the tuning for B#, D and E to hold off messing up previously sustained notes.
        t.push(td(297.5, 4, r(5, 4), [
            P, P, P, P,
            e_s, f_s, P, P,
            P, P, P, P,
        ]));

        // Bar 80:4: G#7(b5,#5,#9)

        // The LH can form a G#!7 4:5:7 shell
        let b_s = g_s * r(5, 4); // B#: 5/4 simple maj 3

        // Idea: let G#-B-D-E be stack of septimal min. thirds (which in 31 edo, tempers out to aug 5th)
        let d = b * r(7, 12); // D: stack 7/6 from B
        let e = d * r(7, 6); // E: stack 7/6 from D

        t.push(td(300.8, 4, r(5, 4), [
            P, d, P, e,
            e_s, f_s, P, P,
            P, P, b, b_s,
        ]));

        // this settles the tuning until m. 83

        // Bar 83: Dm6

        // Is using the very flat double septimal minor D a good idea?
        // > FUTURE NOTE: no it is not.

        // Instead, use B as the anchor, and D is 3/5 of B.

        let d = b * r(3, 5);
        let f = d * r(7, 6); // same tuning as E previously
        let a = d * r(3, 2);
        let c_s = d * r(11, 12);

        // for the accented G#, use the same tuning as the in bar 80
        // G#-A = 21/20 = 84.5c
        // println!("G#-A interval: {}", a / g_s);

        t.push(td(314.4, 4, r(5, 4), [
            c_s, d, P, P,
            f, P, P, P,
            a, P, P, P,
        ]));

        // -----------------------------------------------------------
        // PAGE 12

        // Bar 88: Eb13b9, Db13b9 (rootless), Bb13b9 (rootless), G#13b9 (rootless)

        /*
        poussa un eclat de rire, ...
         */

        // Reset all comma pumps, the 'maniac laugh' need not be connected in pitch.
        // Aim for as many 7, 11, and 13-limit relations as possible (key characters of the mystical).
        // 7 - Ondine
        // 11 - Rust, rouge
        // 13 - Turbulence, waves, biting scenery, drying machinery.

        // C#, E#, G#, A must match the ending (and starting) theme of 8:10:12:13.

        // Eb is the current chord root.
        // Start by tuning the 'laugh' exactly the same as the more functional ending of the laugh
        // (end of m. 88, with the 4 times repeating G# F# C# D# melody that resolves to the
        // melody E# over C#(!13))

        let e_b = r(9, 8); // functionally 2 of the new root C# = 1/1 (back to starting tuning)
        let c_s = r(1, 1); // FIXED simple 3-lim relation with current chord root, to preserve key for later.
        let f_s = e_b * r(7, 6); // 7th harm of G# (dominant)
        let c = e_b * r(13, 8); // 13th harm of D#
        let e = c * r(5, 8); // C-E form 5-lim third
        let g = c * r(3, 4); // C-G form 3-lim P5 (Eb-G discordant)

        // For Db13b9 (rootless), target 7/6 for Cb-Ebb (B-D)
        // use 1/1 C# = Db as chord root, B is 7th harmonic of fundamental C#.
        let f = c_s * r(5, 4); // FIXED
        let b_b = f * r(4, 3); // F-Bb is a 3-limit P4
        let b = c_s * r(7, 4);
        let d = b * r(7, 12); // B-D = 7/6

        // For Bb13b9, target Ab = 3/2 of C#
        let a_b = c_s * r(3, 2); // FIXED

        // For G#13b9, target A = 13/8 of C#
        let a = c_s * r(13, 8); // FIXED

        t.push(td(346.1, 4, r(5, 4), [
            c_s, d, e_b, e,
            f, f_s, g, a_b,
            a, b_b, b, c,
        ]));

        // Bar 88, line 2, last 2 beats (written in cue size)

        // avoid 21/16 P4 between F# and C# for G# F# C# D# melody
        let f_s = c_s * r(4, 3);
        t.push(td(355.81, 4, r(5, 4), [
            P, P, P, P,
            P, f_s, P, P,
            P, P, P, P,
        ]));


        Arc::new(Mutex::new(Tuner::new(t)))
    };
}
