# ᔑႵ𐍂ߎᎦ६

Rhymes with juice. 

Short for [abstruse](https://en.wiktionary.org/wiki/abstruse).

Unicode inter-alphabet substitutions. Try the [online version](https://nmoroney.github.io/notes/2026/260423_struse/index.html).

![struse message a to z](/images/2604-struse_message-a_to_z-01-gain_mapped.jpg)

Really pops with a [gain map](https://github.com/NMoroney/Awesome-Gain-Maps/tree/main).

---

**Overview**

The code basically consists of a series of dictionaries.

The dictionary key is a lower case a-to-z character and the dictionary value is a Unicode character subsitution.

An example dictionary is shown below :

```js
m1 = {
  "a": "&#x10300;",  // OLD ITALIC LETTER A 
  "b": "&#x1041A;",  // DESERET CAPITAL LETTER VEE 
  "c": "&#x1103;",   // HANGUL CHOSEONG TIKEUT
  "d": "&#x1095;",   // MYANMAR SHAN DIGIT FIVE
  "e": "&#x0F5B;",   // TIBETAN LETTER DZA
  "f": "&#x0CEF;",   // KANNADA DIGIT NINE
  "g": "&#x0D6D;",   // MALAYALAM DIGIT SEVEN
  "h": "&#x0A2E;",   // GURMUKHI LETTER MA
  "i": "&#x1109;",   // HANGUL CHOSEONG SIOS
  "j": "&#x10408;",  // DESERET CAPITAL LETTER SHORT A 
  "k": "&#x051E;",   // CYRILLIC CAPITAL LETTER ALEUT KA 
  "l": "&#x10539;",  // CAUCASIAN ALBANIAN LETTER CHA 
  "m": "&#x1328;",   // ETHIOPIC SYLLABLE CHA
  "n": "&#x0E17;",   // THAI CHARACTER THO THAHAN 
  "o": "&#x07DB;",   // NKO LETTER SA
  "p": "&#x10502;",  // ELBASAN LETTER CE
  "q": "&#x09ED;",   // BENGALI DIGIT SEVEN
  "r": "&#x053B;",   // ARMENIAN CAPITAL LETTER INI 
  "s": "&#x0D17;",   // MALAYALAM LETTER GA 
  "t": "&#x10B5;",   // GEORGIAN CAPITAL LETTER KHAR
  "u": "&#x053F;",   // ARMENIAN CAPITAL LETTER KEN
  "v": "&#x13C9;",   // CHEROKEE LETTER QUO
  "w": "&#x0E1F;",   // THAI CHARACTER FO FAN 
  "x": "&#x0514;",   // CYRILLIC CAPITAL LETTER LHA
  "y": "&#x080A;",   // SAMARITAN LETTER KAAF
  "z": "&#x10350;"   // OLD PERMIC LETTER AN
}
```

These dictionaries were created by spending quality time poring over Unicode tables.

Some randomization is added on a per character basis :

```js
function to_struse(s1) {
  s2 = ""
  for (const char of s1) {
      c1 = char.toLowerCase();
      if (c1 in m1) {
        const rn = Math.floor(Math.random() * 3) + 1;
        if (rn == 1) {
          s2 += m1[c1];
        } else if (rn == 2) {
          s2 += m2[c1];
        } else {
          s2 += m3[c1];
        }
      } else {
        if (char == " ") {
          s2 += "&ensp; "
        } else {
          s2 += c1;
        }
      }
  }
  return s2
}
```

That's it.

---

**Notes**

* don't mix [right-to-left](https://en.wikipedia.org/wiki/Category:Right-to-left_writing_systems) and [left-to-right](https://developer.mozilla.org/en-US/docs/Glossary/LTR) in the same word
* the [JavaScript](/javascript/) implementation desribes the hub-and-spoke usage of Gemini CI
* see [struse examples](struse_examples.md) for more details about the HTML entity substitution process



