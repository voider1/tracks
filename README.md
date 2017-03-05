# Tracks

## What is the goal of Tracks?

Tracks has to goal to become a library that enables experimenting with
binary analysis. The library should provide:
* Translators to various intermediate languages (currently focusing on REIL).
* Implementations of known algorithms to lift binary code to higher abstractions
(e.g., Basic blocks, CFGs, callgraphs)
* A generic dataflow implementation that enables quick prototyping of dataflow analysis.
* ...

## Why Tracks?
Tracks allows me, and potentially others, to learn and experiment with binary analysis techniques.
By providing a solid foundation it could become easier to learn, test, and compare new advances
in the area of binary reverse-engineering.

Learning is also the reason for reinventing the wheel as suppose to using existing binary
reverse-engineering frameworks.

## What is tracks not?

Tracks is not intended as a binary reverse-engineering tool like like IDA Pro,
Binary Ninja, or Radare2. Becoming a successful binary reverse engineering tool requires
many features that distracts from the goals of Tracks. This doesn't exclude the use of Tracks
,if it is mature enough, as part of a binary reverse-engineering tool.

