package go_test

import "strings"

func sortSentence(s string) string {

	words := strings.Split(s, " ")
	wordies := make([]string , len(words))
	for _,word := range words { 

		for i,ch := range word { 
			if ch >= '0' && ch <= '9' { 
				val := int(ch-'0')
				wordies[val-1] = word[:i]
				break 
			}
		}
	}
	return strings.Join(wordies, " ")
}
