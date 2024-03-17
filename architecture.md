
## Design point
`pid` is kind of confusing in linux, especially when it comes to debug. On Windows, when you attach to process, all threads are controlled(but child process might not, depending on `DEBUG_ONLY_THIS_PROCESS` flag)

But on linux, ptrace's parameter is thread id in fact.

Anyway, it does not make much sense to only track some thread for debugger(maybe useful for user):
- Theads share the memory, so when software breakpoint is set, non-tracked thread might also hit the breakpoint. Then what would happen? It should be stooped, but, there is no one to handle the singal.
- When a thread is stoped, all other threads are stoped, no matter whether it's traced or not.
