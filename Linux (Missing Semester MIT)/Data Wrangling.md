# **Lecture 4: Data Wrangling** 

# **Lecture 4: Data Wrangling**

### **Pipes & Filtering:**
- `|` : Sends the output of the left command as input to the right command  
  e.g. `journalctl | grep sshd`

- `grep` : Filters lines matching a pattern  
  e.g. `grep -i intel`

- `ssh` : Runs commands on a remote machine and streams output locally  
  e.g. `ssh myserver 'journalctl | grep sshd | grep "Disconnected from"' | less`

### **Viewing & Saving Output:**
- `less` : View large output page by page  
  e.g. `less ssh.log`

- `>` : Redirect output to a file (overwrite)  
  e.g. `journalctl | grep sshd > ssh.log`

- `>>` : Append output to a file  
  e.g. `echo "new line" >> ssh.log`

### **sed (Stream Editor):**
- `sed 's/OLD/NEW/'` : Replace text using patterns  (s stands for substution)
  e.g. 
  ```bash
  ssh myserver journalctl
  | grep sshd
  | grep "Disconnected from"
  | sed 's/.*Disconnected from //''`

- `sed -E` : Enable extended regular expressions  
  e.g. `sed -E 's/.*user (.*) port.*/\1/'`

- `( )` : Capture groups (extract parts of a line)  
  e.g. `sed -E 's/.*user (.*) port.*/\1/'`

### **Sorting & Counting:**
- `sort` : Sort input  
  e.g. `sort`, `sort -n`, `sort -r`

- `uniq -c` : Collapse duplicate adjacent lines and show counts  
  e.g. `sort | uniq -c`

- `tail -n10` : Show last 10 lines  
  e.g. `sort | uniq -c | sort -nk1,1 | tail -n10`

### **awk (Text Processing):**
- `awk '{print $2}'` : Print the second column (whitespace-separated)  
  e.g. `awk '{print $2}'`

- `awk 'condition { action }'` : Filter lines based on conditions  
  e.g. `awk '$1 == 1 && $2 ~ /^c.*e$/ { print $2 }'`

### **Joining Lines**
- `paste` : Combine multiple lines into one  
  e.g. `paste -sd,`

### **Line Counting**
- `wc -l` : Count number of lines  
  e.g. `wc -l`

### **Math in Shell**
- `bc` : Command-line calculator that reads from stdin  
  e.g. `paste -sd+ | bc -l`

### **Plotting**
- `gnuplot` : Plot data directly from the command line  
  e.g. `gnuplot -p -e 'plot "-" using 1:xtic(2) with boxes'`

### **xargs (Build Arguments from Input)**
- `xargs` : Convert stdin into command arguments  
  e.g.
  ```bash
  rustup toolchain list \
   | grep nightly \
   | sed 's/-x86.*//' \
   | xargs rustup toolchain uninstall


### **Binary Data Wrangling**
- Pipes can also process binary data 
  e.g.
  ```bash
  ffmpeg ... \
  | convert ... \
  | gzip \
  | ssh remote 'gzip -d | tee copy.jpg | feh -'


Reference: [https://missing.csail.mit.edu/2020/course-shell/](https://missing.csail.mit.edu/2020/course-shell/)