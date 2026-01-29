# **Lecture 3: Editors (Vim)** 

### **Modes in Vim:** 

- normal mode 
    - type 'vim filename' to open a file in Vim 
    - ****you must get into the normal mode first to enter other modes**** 
    - :q quit (close current window) 
    - :w save (“write”) 
    - :wq save and quit 
    - :e {name of file} open file for editing 
    - :ls show open buffers 
    - :help {topic} open help 
    - :help :w opens help for the :w command 
    - :sp {name of file} open file in a new split window 
- insert mode: use buffer to store text 
    - i - insert before cursor 
    - press esc to exit insert mode 
- replace mode: use buffer to replace text 
    - r - replace single character x 
    - R - replace multiple characters x 
- visual mode 
    - v - visual character mode 
    - V - visual line mode 
    - ctrl-v - visual block mode 
- command line mode 
    - : - enter command mode 
    - q! - quit without saving 
    - !command - execute shell command 

### **Vim’s Movement:** 
- when you get into vim normal mode, you can move the cursor around the file using the hjkl keys 
    h - left 
    j - down 
    k - up 
    l - right 
- Words: w (next word), b (beginning of word - move forward), e (end of word - move backward) 
- Lines: 0 (beginning of line), ^ (first non-blank character), $ (end of line) 
- Screen: H (top of screen), M (middle of screen), L (bottom of screen) 
- Scroll: Ctrl-u (move up one screen faster than k), Ctrl-d (move down one screen faster than j) 
- File: gg (move to the beginning of file), G (move to the end of file) 
- Line numbers: :{number}<CR> or {number}G (line {number}) 
- Misc: % (corresponding item) 
- Find: f{character}, t{character}, F{character}, T{character} find/to forward(low caption f,t)/backward(capital F,T) first appear {character} on the current line 
- , / ; for navigating matches 
- Search: /{regex}, n / N for navigating matches 

### **Vim’s Editing:** 
- i enter Insert mode but for manipulating/deleting text, want to use something more than backspace 
- o / O insert line below / above 
- d{motion} delete {motion} e.g. dw is delete word, de is delete the end of the word, d0 is delete to beginning of line, dd is delete the line 
- c{motion} change {motion} e.g. cw is change word, ce is change the end of the word, cc is change the current line context like d{motion} followed by i 
- x delete character (equal to dl) 
- s substitute character (equal to cl) 
- u to undo 
- <C-r> to redo 
- y to copy (copy the current line) yw means copy a word 
- p to paste (paste the copied line below the current line)
 - ~ flips the case of a character (from i to I, a to A) 
 
 ### **Counts:** 
 - 3w move 3 words forward 
 - 5j move 5 lines down 
 - 7dw delete 7 words 
 
 ### **Modifiers:** 
 - a stands for around 
    - da' delete a single-quoted string, including the surrounding single quotes (it will delete all the content within the quotes "") 
 - i stands for inside
    - ci( change the contents inside the current pair of parentheses 
    - ci[ change the contents inside the current pair of square brackets (it will delete all the content within the square bracket"[]") 
    
### **Advanced Vim: (not cover in the lecture but in the notes)** ### ***Search and replace*** 
- :s (substitute) command (documentation) 
-- %s/foo/bar/g: replace foo with bar globally in file 
-- %s/\[.*\](\(.*\))/\1/g: replace named Markdown links with plain URLs 

### ***Macros*** 
- q{character} to start recording a macro in register {character} 
- q to stop recording 
- @{character} replays the macro 
- Macro execution stops on error 
- {number}@{character} executes a macro {number} times 
- Macros can be recursive 
    first clear the macro with q{character}q 
    record the macro, with @{character} to invoke the macro
    
recursively (will be a no-op until recording is complete) 
- Example: convert xml to json (file) 
    Array of objects with keys “name” / “email” 
    Use a Python program? 
    Use sed / regexes 
    - g/people/d 
    - %s/<person>/{/g
    - %s/<name>\(.*\)<\/name>/"name": "\1",/g 

    Vim commands / macros 
    - Gdd, ggdd delete first and last lines 
    - Macro to format a single element (register e) 
        - Go to line with <name> 
        - qe^r"f>s": "<ESC>f<C"<ESC>q 
    - Macro to format a person 
        - Go to line with <person> 
        - qpS{<ESC>j@eA,<ESC>j@ejS},<ESC>q 
    - Macro to format a person and go to the next person 
        - Go to line with <person> 
        - qq@pjq 
    - Execute macro until end of file 
        - 999@q 
    - Manually remove last , and add [ and ] delimiters

Reference: [https://missing.csail.mit.edu/2020/course-shell/](https://missing.csail.mit.edu/2020/course-shell/)