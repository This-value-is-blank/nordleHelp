A command line [Nordle](https://www.nordle.us) solver.
======================================================

Well, currently its just a Wordle solver

How to run
----------

``` 
$ cargo run -r [path to wordslist.txt]
```

What is wordslist.txt?
----------------------

Its a list of "all" the words, separated by newlines (lf or crlf)

I have put my words list in Documents/wordsList.txt, so I run  
```
$ cargo run -r ~/Documents/wordsList.txt
```


Usage
-----

First of all, the prompt  
```
(370106) >> 
```
The number is the current number of words possible. As you run commands, the number of possible words will go down.

Lots of commands have a input "[asd]". This is what Nordle (or wordle) has given you back from your guess.

| Letter | What it means | Colour (usually) |
|--------|---------------|------------------|
| a      | The letter is correct, in the right spot | Green |
| s      | The letter is correct, in the wrong spot | Yellow / Orange |
| d      | The letter is wrong | Grey |

---

### Commands:
- ```help [command]```
	- Its the help command
- ```length [number]```
	- Removes all words which don't have the specified length
- ```aw [word] [asd]```  
	Add Word
	- Removes all the words which don't match
	- This requires all the words to have the same length as the inputted [word]
- ```al [letter(s)] [sd]```  
	Add Letter
	- Removed all words that don't match the letters given
	- Because al dos'nt have positional information, you cannot specify a. Only use	s (in the word), and d (not in the word).
	- This dose not require all words to be the same length
- ```reset [?length]```
	- Reloads the words list, and optionally also runs length if a length is given
- ```rsl [length]```  
	ReSet Length
	- Runs reset, then length
- ```list```
	- Print the possible words to the screen
- ```write```
	- Writes the possible words to ./possibleWords.txt, as a newline (\n) separated file
- ```exit```
	- Exits. Bye!
	

Examples
--------

```length 5``` Start by restricting the length to (usually 5, for wordle like games)

``` aw super ddddd ```
removes all  words that contain the letters "s", "u", "p", "e", or "r". 

``` aw waves addsd ```
has "w" in the correct (first) spot, "e" correct but not in the fourth spot, and "a", "v", and "s" are not in the word. This will remove all words that don't match this criteria.

For adding words with duplicated letters (Eg Taste)  
``` aw tas_e sdddd ```  
Replace the not interesting occurrence (In this case, the second one) with a character that dos'nt exist in the words list (eg _ ), and in the asd input, put d in the duplicated letters place.   
This is only an issue (and therefore only necessary) when the duplicated letters asd different. 

| Example | Use word | Use asd |
|---------|-----|-----|
| sxxdx | tas_e | sxxdx |
| sxxsx | taste | sxxsx |
| axxsx | taste | axxsx |
| dxxax | _aste | dxxax |
| dxxdx | taste | dxxdx |
|       | tas_e | dxxdx |
|       | _aste | dxxdx |

(x in this table is a padding character)


Run ``` list ``` when the number to the left of the prompt is small.

And once you got the word correct (or lost 🙁 ), run ``` rsl 5 ``` to reset, and only include 5 letter words
