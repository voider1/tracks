# Tracks

## What is the goal of Tracks?

*Tracks* has to goal to become a library that enables experimenting with
binary analysis. The library should provide:
* Translators to various intermediate languages (currently focusing on REIL).
* Implementations of known algorithms to lift binary code to higher abstractions
(e.g., Basic blocks, CFGs, callgraphs)
* A generic dataflow implementation that enables quick prototyping of dataflow analysis.
* ...

## Why Tracks?
*Tracks* allows me, and potentially others, to learn and experiment with binary analysis techniques.
By providing a solid foundation it could become easier to learn, test, and compare new advances
in the area of binary reverse-engineering.

Learning is also the reason for reinventing the wheel as suppose to using existing binary
reverse-engineering frameworks.
How *intermediate languages*, such as [BIL](https://users.ece.cmu.edu/~aavgerin/papers/bap-cav-11.pdf),
[REIL](http://www.zynamics.com/downloads/csw09.pdf), and [ESIL](https://radare.gitbooks.io/radare2book/content/esil.html),
compare to each other should be one of the questions that can be investigate with *Tracks*. 

## What is Tracks not?

Tracks is not intended as a binary reverse-engineering tool like like [IDA Pro](https://www.hex-rays.com/products/ida/index.shtml),
[Binary Ninja](https://binary.ninja/), or [Radare2](https://radare.org/r/).
Becoming a successful binary reverse engineering tool requires
many features that distracts from the goals of *Tracks*. This doesn't exclude the use of *Tracks*
,if it is mature enough, as part of a binary reverse-engineering tool.

