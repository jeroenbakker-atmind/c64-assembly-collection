## Code style

- [ ] Add glosary
  - `char` A character in a charset describes the character in 64 bits/8 bytes.
  - `encoded_char` A compressed character Uses less bits and will result in low resolution characters.
  - `charset` a set of 256 characters each being indexed by a single byte.
  - `screenchar`: A byte in screen memory references a char inside the charset

## Code improvements

- [ ] Introduce a image sequence.

## Encoding improvements

- [ ] Split static screen chars, that is screen chars that don't alter (they can share a char in the charset), from dynamic screen chars. Store the static screen chars at the beginning of the char set. Leave the rest for the dynamic.
- [ ] When number of dynamic screen chars fit fully in charset number them and only alter the charset.
  - [ ] Screen RLE decoding can skip (for the background) add a number of static chars (includes an array of static chars) or add a number of dynamic chars (screen char is last used dynamic screen char + 1)
- [ ] When number of dynamic screen chars don't fit, they they need to be changed in time.
- [ ] LHZ encoding/decoding?
- [ ] Autodetect repeating frames and automatically add a loop? Perhaps not this should be done when building the demo.

## Areas of investigation

- [ ] Both charmap and screenchars uses similar encoding techniques. Perhaps join them into update memory commands. The main difference is that screen chars are 8 bit and charmap (encoded) are 16 bit.