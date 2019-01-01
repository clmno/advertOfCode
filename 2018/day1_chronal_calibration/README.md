Navigation
Let's just write something so we can test vim. Now that was a sentence. How'bout many sentences like that?

This will be a new paragraph. It should be able to jump between para's using { and }.

I'm not sure what a section is. They say we can jump to previous section using [[(previous) and ]] (next). And using [] to jump to the end of the previous section.

Naviagtion keys
Remember, h to move cursor to the left. j to move it down, k to move up and l to move it to the right. 0 to go to the beginning of a sentence, $ to go to the end of a sentence. w to move forward one word, W to move forward one word (delimiter will be space here) and b to move backwards one work, B..
G to move to the end of the file and gg to go to the beginning of the file.

Inseting text
a 	- Insert text after the cursor
A 	- Insert text at the end of the line
i 	- Insert text before the cursor
o 	- Begin a new line below the cursor
O 	- Begin a new line above the cursor

Special Inserts
:r[filename] - Insert the file below the cursor
:r![command] - Execute [command] and insert it's o/p below the cursor

Delete text
x 	- Delete character at cursor
dw 	- delete word
de	- delete word, not the ending whitespace
d0 	- delete to the beginning of a line
d$	- delete to the end of a line
d)	- delete the end of a sentence
dgg	- delete to the beginning of a file
dG	- you get the point
dd	- delete line
3dd	- delete three line

Simple replace text
r{text}	- Replace the chars under the cursor with {text}
R	- Replace chars instead of inserting them

Copy/Paste text 
yy	- copy current line into storage buffer
["x]yy	- Copy the current lines into register x
p	- paste storage buffer after current line
P	- paste storage buffer before current line
["x]p	- paste from register x after current line
["x]P	- paste from register x before current line

Undo/Redo
u	- undo the last operation
Ctrl+r	- redo the last undo

Search and Replace
/search_text	- search document for search_text going foward
?search_text	- "" going baclwards
n		- move to the next instance of the result from the search
N		- "" previous instance
:%s/original/
replacement	- Search for first occurance of string "original" and replace with "replacement"
:%s/original/
replacement/g	- Above stuff but for all
:%s/original/
replacement/gc	- Above (for all) but ask for confirmation before moving forward

Select Text
v	- Enter visual mode per charater
V 	- Enter visual mode per line
Esc	- Exit visual mode

Modify selected text
~	Switch case
d	delete a word
c	change
y	yank
>	shift right
<	shift left
!	filter through an external command


