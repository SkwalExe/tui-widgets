# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/alignment.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 480
Hide
Type@0 "cargo run -p tui-big-text --example alignment --quiet"
Enter
Sleep 2s
Show
Screenshot "target/alignment.png"
Sleep 1s
Hide
Escape