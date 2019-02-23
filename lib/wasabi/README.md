### DESCRIPTION
Wasabi is basically a project that wraps the Clang/musl toolchain. And it is Linux-based.

### PORTABILITY ISSUE
It's somewhat easy to reimplement the following syscalls for BSD-like operating systems, but some form of emulation is needed for Windows support.

There are several POSIX-compatible environment for Windows like [Cygwin](https://cygwin.com/) and [Midipix](https://midipix.org/), but they have restrictive licenses. Cygwin has an LGPLv3 license while Midipix has a GPLv3 license. While the inclusion of a GPLv3 project in an Apache project is considered [incompatible](https://www.apache.org/licenses/GPL-compatibility.html), an Apache project can still dynamically link against a LGPL project. This also means one can't distribute the binary with the project. I'm not sure that is a great approach.


Windows 10 has a subsytem for Linux, but that is behind a developer flag and there are several users that are going to be on older Windows versions.

To cut the long story short, this problem remains unsolved.

### SYSCALLS AND SECURITY


Id  | Name                     | Signature                         | Capabilites
----|:-------------------------|:----------------------------------|:---------------------
_   |                          |                                   |
