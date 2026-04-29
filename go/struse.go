// (c) 2026 : https://github.com/NMoroney + Gemini CLI
// MIT License
// https://github.com/NMoroney/Struse
// https://nmoroney.github.io/notes/2026/260423_struse/index.html
//

package main

import (
	"fmt"
	"math/rand"
	"strings"
	"time"
	"unicode"
)

var m1 = map[rune]string{
	'a': "&#x10300;", 'b': "&#x1041A;", 'c': "&#x1103;", 'd': "&#x1095;",
	'e': "&#x0F5B;", 'f': "&#x0CEF;", 'g': "&#x0D6D;", 'h': "&#x0A2E;",
	'i': "&#x1109;", 'j': "&#x10408;", 'k': "&#x051E;", 'l': "&#x10539;",
	'm': "&#x1328;", 'n': "&#x0E17;", 'o': "&#x07DB;", 'p': "&#x10502;",
	'q': "&#x09ED;", 'r': "&#x053B;", 's': "&#x0D17;", 't': "&#x10B5;",
	'u': "&#x053F;", 'v': "&#x13C9;", 'w': "&#x0E1F;", 'x': "&#x0514;",
	'y': "&#x080A;", 'z': "&#x10350;",
}

var m2 = map[rune]string{
	'a': "&#x1D70;", 'b': "&#x266D;", 'c': "&#x122D;", 'd': "&#x1D1CE;",
	'e': "&#x096C;", 'f': "&#x10346;", 'g': "&#x1B9D;", 'h': "&#x10AC;",
	'i': "&#x10382;", 'j': "&#x1D4A5;", 'k': "&#x20AD;", 'l': "&#x1D4DB;",
	'm': "&#x0BF1;", 'n': "&#x0E81;", 'o': "&#x0C20;", 'p': "&#xA6E0;",
	'q': "&#x1007;", 'r': "&#x10342;", 's': "&#x13A6;", 't': "&#x1D366;",
	'u': "&#x07CE;", 'v': "&#x2A54;", 'w': "&#x0E9C;", 'x': "&#x0CF1;",
	'y': "&#x10284;", 'z': "&#x2F04;",
}

var m3 = map[rune]string{
	'a': "&#x10001;", 'b': "&#x2CD2;", 'c': "&#x1573;", 'd': "&#x1D507;",
	'e': "&#x0B3D;", 'f': "&#x2A0D;", 'g': "&#x01E4;", 'h': "&#x12E0;",
	'i': "&#x1703;", 'j': "&#x3057;", 'k': "&#xA018;", 'l': "&#x2827;",
	'm': "&#x0D28;", 'n': "&#x12AC;", 'o': "&#x0DA7;", 'p': "&#xAA6D;",
	'q': "&#x10E2;", 'r': "&#x053B;", 's': "&#x1511;", 't': "&#x2020;",
	'u': "&#x00B5;", 'v': "&#xAA9C;", 'w': "&#x0429;", 'x': "&#x102C4;",
	'y': "&#x03AB;", 'z': "&#xA903;",
}

func init() {
	rand.Seed(time.Now().UnixNano())
}

// ToStruse performs Unicode inter-alphabet substitutions.
func ToStruse(s string) string {
	var builder strings.Builder
	for _, char := range s {
		cLower := unicode.ToLower(char)
		if _, exists := m1[cLower]; exists {
			rn := rand.Intn(3) + 1
			switch rn {
			case 1:
				builder.WriteString(m1[cLower])
			case 2:
				builder.WriteString(m2[cLower])
			case 3:
				builder.WriteString(m3[cLower])
			}
		} else {
			if char == ' ' {
				builder.WriteString("&ensp; ")
			} else {
				builder.WriteRune(cLower)
			}
		}
	}
	return builder.String()
}

func main() {
	input := "rhymes with juice"
	fmt.Println(ToStruse(input))
}
