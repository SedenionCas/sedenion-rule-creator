# Sedenion Rule Creator

## Overview

The Sedenion Rule Creator is a powerful tool designed to facilitate the creation of mathematical rule sets utilized within the [Sedenion Engine](https://github.com/SedenionCas/sedenion-engine). This repository serves as the official source for the tool, providing developers with the means to craft custom rule sets to enhance the functionality of the Sedenion Engine.

## Usage

To use the Sedenion Rule Creator effectively, follow these simple steps:

1. Begin by defining your desired rules in the `rules.in` file. The syntax for expressing these rules is comprehensively described [here](https://github.com/SedenionCas/sedenion-rule-creator#syntax).

2. Once the rules have been specified in the `rules.in` file, proceed to build and run this repository using the command `cargo run`.

3. Upon successful execution, the newly generated rules will be made available in the `rules.rs` file, ready for integration into the Sedenion Engine.

## Syntax

The Sedenion Rule Creator supports a versatile and intuitive syntax for defining rules. Key aspects of the syntax include:

- Numbers, functions, and operands follow the conventions established in other Sedenion products, and they are matched precisely within the engine.

- Comments are denoted with double slashes at the start of the line (`//`) and last the entire file. Comments are ignored. There should not be comments between "from" and "to" states.

- `$L[n]`: Represents fixed constants, such as numerical values or specific characters. Usage: `$L1` or `$L42`.

- `$[n]`: Acts as a placeholder or wildcard, matching any subexpression in the corresponding position on the left-hand side (pattern) of the rule. Unlike `$L[n]`, this symbol is not limited to literal values and can encompass any expression. Usage `$1` or `$42`.

- When defining rules in the `rules.in` file, adhere to the following pattern:

```
<rule from>
<rule to>
<blank line>
<rule from>
<rule to>
<blank line>
...
```

Please note that all "from" states must possess greater complexity than the "to" state, although this is not enforced in any way.

