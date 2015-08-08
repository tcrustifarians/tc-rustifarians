# Let's Build a Compiler [in Rust]!

This project is a translation from Turbo Pascal of the series
["Let's Build a Compiler"][lbc], by Jack Crenshaw, written starting in
1988. It emits x86_64 assembly.

[lbc]: http://compilers.iecc.com/crenshaw/

The emitted code can be assembled and linked with GCC. It calculates an
integer expression (so far) and puts the result in its exit code:

    $ cargo run -q > prgm.s
    (1+3-2)*8-10
    $ gcc -o prgm prgm.s 
    $ ./prgm; echo $?
    15
