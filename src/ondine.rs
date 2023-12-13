//! Contains tuning data for Ondine (Gaspard de la Nuit, Ravel).
//!
//! Page number & score references are with respect to the EDITION PETERS publication.

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
        t.push(td(5.0, 4, r(5, 4), [ // written as C# root
            P, c_x, P, P,
            P, P, P, g_s,
            P, P, P, P, // (B# remains as 9/8 of A#)
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

        t.push(td(8.0, 4, r(5, 4), [
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

        t.push(td(11.0, 4, r(5, 4), [
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

        t.push(td(14.0, 4, r(5, 4), [
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

        t.push(td(16.0, 4, r(5, 4), [
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

        t.push(td(17.0, 4, r(5, 4), [
            P, P, d_s, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        // Bar 18: F#9(13)/G# (Same as bar 16)
        let d_s = r(9, 8); // back to normal
        t.push(td(17.0, 4, r(5, 4), [
            P, P, d_s, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        // Bar 19: A#7#11/E (same 31 limit tuning as bar 17)
        let d_s = c_s * r(31, 28);
        t.push(td(17.0, 4, r(5, 4), [
            P, P, d_s, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        // -----------------------------------
        // PAGE 3

        // Bar 20: F#9(13)/C# (as C#m6add11)
        let d_s = r(9, 8); // back to normal
        t.push(td(17.0, 4, r(5, 4), [
            P, P, d_s, P,
            P, P, P, P,
            P, P, P, P,
        ]));

        // Bar 21: A#m11b5 (slightly different sonority here)
        // No more D# here, and the function of D# on beat 3 of this bar
        // is different, we can use the 13 limit D# to bring out the full
        // primodal-3 stack: [5, 6, 7, 9, 11, 13]/3
        let d_s = f_s * r(13, 16);
        t.push(td(17.0, 4, r(5, 4), [
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

        t.push(td(22.0, 4, r(5, 4), [
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
        t.push(td(22.9, 4, r(5, 4), [
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

        t.push(td(23.0, 4, r(5, 4), [
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
[3/4, 1/1, 9/8, 5415/4096, 15/8, 55233/16384, 9/4, 55233/8192]
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

        t.push(td(26.0, 4, r(5, 4), [
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

        t.push(td(27.0, 4, r(5, 4), [
            P, P, P, P,
            e_s, P, f_x, P,
            g_x, P, P, P,
        ]));

        // Bar 27:4.5: E#7b9

        // Again looks weird on the score, but it's just E#7b9 (F# is enharmonic b9 of E#)

        // since there is no F# any time soon, we are free to tune the b9 however we want.
        // bars 28-29 are rich, so go for rich sounds.

        let f_s = e_s * r(17, 16); // 17th harmonic of E#
        t.push(td(27.45, 4, r(5, 4), [
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

        t.push(td(28.0, 4, r(5, 4), [
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

        t.push(td(29.0, 4, r(5, 4), [
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
# B    F#    A      C#     A      C#
[3/10, 9/20, 21/40, 27/40, 21/20, 27/20]
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

        t.push(td(30.0, 4, r(5, 4), [
            c_s, P, d_s, e,
            e_s, f_s, P, g_s,
            a_s, P, P, b_s,
        ]));


        Arc::new(Mutex::new(Tuner::new(t)))
    };
}
