
## ᔑႵ𐍂ߎᎦ६ : ꩭΫႵႬఠท

The Gemini CLI port of struse.js to Python.

```py
m1 = {
    "a": "&#x10300;",  # OLD ITALIC LETTER A 
    "b": "&#x1041A;",  # DESERET CAPITAL LETTER VEE 
    "c": "&#x1103;",   # HANGUL CHOSEONG TIKEUT
    "d": "&#x1095;",   # MYANMAR SHAN DIGIT FIVE
    "e": "&#x0F5B;",   # TIBETAN LETTER DZA
    "f": "&#x0CEF;",   # KANNADA DIGIT NINE
    "g": "&#x0D6D;",   # MALAYALAM DIGIT SEVEN
    "h": "&#x0A2E;",   # GURMUKHI LETTER MA
    "i": "&#x1109;",   # HANGUL CHOSEONG SIOS
    "j": "&#x10408;",  # DESERET CAPITAL LETTER SHORT A 
    "k": "&#x051E;",   # CYRILLIC CAPITAL LETTER ALEUT KA 
    "l": "&#x10539;",  # CAUCASIAN ALBANIAN LETTER CHA 
    "m": "&#x1328;",   # ETHIOPIC SYLLABLE CHA
    "n": "&#x0E17;",   # THAI CHARACTER THO THAHAN 
    "o": "&#x07DB;",   # NKO LETTER SA
    "p": "&#x10502;",  # ELBASAN LETTER CE
    "q": "&#x09ED;",   # BENGALI DIGIT SEVEN
    "r": "&#x053B;",   # ARMENIAN CAPITAL LETTER INI 
    "s": "&#x0D17;",   # MALAYALAM LETTER GA 
    "t": "&#x10B5;",   # GEORGIAN CAPITAL LETTER KHAR
    "u": "&#x053F;",   # ARMENIAN CAPITAL LETTER KEN
    "v": "&#x13C9;",   # CHEROKEE LETTER QUO
    "w": "&#x0E1F;",   # THAI CHARACTER FO FAN 
    "x": "&#x0514;",   # CYRILLIC CAPITAL LETTER LHA
    "y": "&#x080A;",   # SAMARITAN LETTER KAAF
    "z": "&#x10350;"   # OLD PERMIC LETTER AN
}
```

The Gemini CLI prompt :

```
The input file struse.js is code JavaScript for Unicode inter-alphabet substitutions.
It uses three dictionaries and randomization to substitute ASCI lower case characters a to z with similar looking Unicode characters.
Read the contents of struse.js and create a similar version using standard Python.
Save this prompt to the file struse-js_to_py.txt.
Save the resulting python code to the file struse.py and stop.
```

Took 10 minutes wall time vs 7.3s tool time for 30K tokens :

```
  Interaction Summary               
  Tool Calls:                 3 ( ✓ 3 x 0 )  
  Success Rate:               100.0%     
  User Agreement:             100.0% (3 reviewed) 
  Code Changes:               +117 -0   
    
  Performance   
  Wall Time:                  9m 57s    
  Agent Active:               9m 38s  
    » API Time:               9m 31s (98.7%) 
    » Tool Time:              7.3s (1.3%) 
      
  Model                   Reqs   Input Tokens   Cache Reads  Output Tokens 
  ──────────────────────────────────────────────────────────────────────── 
  gemini-2.5-flash-lite      1            853             0             66 
  gemini-3-flash-preview     3         30,469         9,032          2,117
```

