# **Lecture 1: The Shell**

### **Elements in Shell:**
- `missing:~$` - missing is the machine name; ~$ is your current directory (i.e. home directory)

### **Commands in Shell:**

- `date` - prints date info
```bash
missing:~$ date
```
- `echo hello world` - prints “Hello World”
- `echo $PATH` - shows paths through which machine search
- `which echo` - shows path for ‘echo’
- `pwd` - shows current working directory
- `cd /sys` - change working directory of sys
- `cd class/` - change working directory to 'class' 
- `cd ..` - change back to home directory
- `cd .` - goes to current directory
- `ls` - list all files in current directory
- `cd ~` - goes to /home/illumine i.e. home directory
- `cd -` - goes back to previous directory
- `mv` - Moves file to new location or rename it when present and new file name is entered
```bash
missing:~$ mv file.txt tory.txt
```
- `cp` - creates entirely new path for a file while not changing the original file
- `rm` - remove a file
- `rmdir` - remove an empty directory
- `rm -r` - recursively remove(-r) a directory and all its contents (use with caution!)
- `mkdir` - create a new directory
    use mkdir "My photos" to create a directory with space
- `man` - shows manual of any command(like man ls)
- `Ctrl+L` - Clear terminal
- `< file` - save content of what is written in “file”
```bash
missing:~$ cat < hello.txt # shows content of hello.txt
```
- `> file` - save content to file
```bash
missing:~$ cat < hello.txt > hello2.txt
# This reads input from hello.txt and writes it to hello2.txt (effectively copying the file, nothing shows after running this line)
```
- `cat` - show content of file
- `>>` - append contents to a file
```bash
missing:~$ cat < hello.txt >> hello2.txt
# This reads input from hello.txt and writes it to hello2.txt (effectively copying the file, nothing shows after running this line)
missing:~$ cat hello2.txt # it will show the content of hello.txt for twice
```
- `|`  - pipe command(sends output of LHS as input to RHS)
```bash
missing:~$ cat < hello.txt | wc -l # shows number of lines in hello.txt
```
```bash
missing:~$ curl --head --silent https://missing.csail.mit.edu | grep "Last-Modified" # shows last modified time of the website
```
- `tail -n1` - shows last line (-n1)
- `sudo` - ‘do’ as ‘superuser’
- `sudo su` - gives root access to user([amy]$ will become [root]# , you need to use `exit`  to leave the root mode)
- `find` - find files/directories in current directory
```bash
missing:~$ find . -name "*.txt" # find all files with extension .txt in current directory (including directories)
missing:~$ find . -type f -name "*.txt" # find all files with extension .txt in current directory (excluding directories)
```
- `xdg-open` - opens file using the default application for that file type (like double-clicking in GUI)
- `nano` - opens a simple terminal-based text editor (easy for beginners)
- `vim` - opens a powerful terminal-based text editor (steep learning curve but very efficient)
- `less` - view file content page by page (allows scrolling up/down, unlike cat)
- `code` - opens file in VS Code (if installed)



Reference: [https://missing.csail.mit.edu/2020/course-shell/](https://missing.csail.mit.edu/2020/course-shell/)