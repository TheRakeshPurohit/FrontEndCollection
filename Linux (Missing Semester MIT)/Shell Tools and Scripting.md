# **Lecture 2: Shell Tools and Scripting**

### **Variables in Bash:**
```bash
foo=bar
echo "$foo"   # prints bar
echo '$foo'   # prints $foo (no variable substitution)
```
### **Special Variables:**
- `$0` - the name of the script
- `$1` - the first argument to the script
- `$2` - the second argument to the script
- `$@` - all arguments to the script
- `$#` - the number of arguments to the script
- `$$` - the process ID of the script
- `$?` - the exit status of the last command
- `$!` - the process ID of the last background command
- `!!` - the last command executed
- `$_` - last argument of previous command
```bash
mcd () {
    mkdir -p "$1" # the argument to the function mcd()
    cd "$1"
}
```

### **Special Variables:**
- `$0` - the name of the script
- `$1` - the first argument to the script
- `$2` - the second argument to the script
- `$@` - all arguments to the script
- `$#` - the number of arguments to the script
- `$$` - the process ID of the script
- `$?` - the exit status of the last command
- `$!` - the process ID of the last background command
- `!!` - the last command executed
- `$_` - last argument of previous command
```bash
mcd () {
    mkdir -p "$1" # the argument to the function mcd()
    cd "$1"
}
```

### **Exit Codes & Command Chaining:**
- 0 — success
- non-zero — failure

Operators:
- `$?` - the exit status of the last command
- `&&` - run the second command if the first command succeeds
- `||` - run the second command if the first command fails

```bash
false || echo "Oops, fail"
true && echo "Success"
```

### **Command Substitution:**
- $(command) — capture output of command
```bash
echo "I am in $(pwd)"
```

### **Process Substitution:**
- <(command) — pass command output as file
```bash
diff <(ls) <(ls /)
```

### **Conditionals:**
- [[ condition ]] — preferred comparison syntax
```bash
if [[ $? -ne 0 ]]; then
    echo "Error occurred"
fi
```

### **Shebang:**
- Tells kernel which interpreter to use
```bash
#!/usr/bin/env bash
#!/usr/bin/env python
```

### **Shell Globbing (Filename Expansion):**
- * — match any number of characters
- ? — match one character

```bash
rm foo*      # removes foo, foo1, foo2, etc.
```

### **Brace Expansion {}**

```bash
cp file.{txt,md}
touch {foo,bar}/{a..h}
mv *{.py,.sh} folder
```

### **Finding Command Usage**
```bash
command -h / command --help ## short help
man command ## full manual
tldr command ## common examples
```

### **Finding Files**
- find — recursive file search
```bash
find . -name "*.txt"
find . -type d -name src
find . -mtime -1
find . -size +500k -size -10M
```
- fd — faster, user-friendly alternative to find
```bash
fd PATTERN
```
- locate — search indexed file database
```bash
locate filename
```


### **Finding Code**
- grep — search text patterns
```bash
grep "pattern" file
grep -R "pattern" .
grep -C 5 "pattern" file
```
- rg (ripgrep) — faster grep alternative
```bash
rg -t py "import requests"
rg foo -A 5
rg --stats pattern
```

### **Finding Shell Commands (History)**
- history 
```bash
history #show command history
```
- history | grep pattern  
```bash
history | grep pattern #search history for pattern
```
- Ctrl + R 
```bash
Ctrl + R #search history for pattern
```

### **Directory Navigation Tools**
- z / j  #jump to frequently used directories

- tree  #show directory structure

- ranger / nnn   #terminal file managers

Reference: [https://missing.csail.mit.edu/2020/course-shell/](https://missing.csail.mit.edu/2020/course-shell/)