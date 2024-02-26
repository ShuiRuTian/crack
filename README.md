# crack
rust debugger, to crack the shell of crap

## Motivation
Why this should exist when we already have GDB/LLDB?

An interesting fact is usually a tool of some language is written by the language itself. This might make it much more simpiler to implement some functions.

And GDB/LLDB might not pritorize some tasks only for rust, but we could.

Motivation or opportunity:
http://nbaksalyar.github.io/2020/05/19/rust-debug.html
https://blog.yoshuawuyts.com/rust-should-own-its-debugger-experience/

Good features:
https://werat.dev/blog/what-a-good-debugger-can-do/

## Resources
(Paper)The State of Debugging in 2022:
https://dl.acm.org/doi/10.1145/3563768.3570525

go implments its own debugger for better understanding of language features

Delve: https://github.com/go-delve/delve/tree/master

rust communicity has one library, although it's not in active developing, it's still a very good example to learn.

headcrab: https://github.com/headcrab-rs/headcrab/tree/master/Documentation

Writing a Linux Debugger:
https://blog.tartanllama.xyz/writing-a-linux-debugger-setup/

Write a Windows Debugger:
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-1/

Windows Debug resource:
https://learn.microsoft.com/en-us/windows/win32/debug/basic-debugging

EBPF:
https://ebpf.io/what-is-ebpf/

> Note
> Delve use this to provide better perforamnce for trace

https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-1/

x64/x32 debugger for windows: https://x64dbg.com/, the core is https://github.com/x64dbg/TitanEngine

live++ https://liveplusplus.tech/index.html, the tech details: https://liveplusplus.tech/downloads/THQ_Nordic_Dev_Summit_2023_Live++_Behind_the_Scenes.pptx

How replay.io
https://medium.com/replay-io/recording-and-replaying-d6102afee273

This lists very helpful resources for time-traveling debug
https://github.com/airportyh/time-traveling-debugger
