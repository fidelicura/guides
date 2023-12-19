# Info
>Reference: [Crust of Rust: Smart Pointers and Mutability](https://www.youtube.com/watch?v=8O0Nt9qY_vo) by [Jon Gjengset](https://github.com/jonhoo/)

There is simple "instruction": all smart pointers are provided as libraries with own set of tests and implementation details.

I really try to give clear and simple explanations of each item.

>All examples are made **in** and **for** Rust.

# Overview

Information about every data structure is represented in it's own folder's `INFO.md` as:

```markdown
---
title: Smart Pointer Name
author: Kirill <fidelicura> Bezuglyi
---

Main Information
---

# Overview
Brief explanation of **the principle** of this smart pointer;

# Advantages
Common advantages **by design** of this smart pointer;

# Disadvantages
Common disadvantages **by design** of this smart pointer;

# Usecases
Highly **preferred** use cases of this smart pointer;

<!-- end_slide -->

Questions & Answers
---

# Q: "?"
  ### A: "."
```

>All `INFO.md` are fully compatible with [presenterm](https://github.com/mfontanini/presenterm), so try to use it!

# Topics

### Vector
This library demonstrates some basic techniques around `Vector<T>`.
Also, it provides simple and basic implementation of `Vector<T>` by hand.

### Stack
This library provides simple and basic implementation of `Stack<T>` by hand.
Also, this library demonstrates some basic techniques around `Stack<T>`.
>Do not confuse `Stack<T>` data structure with the same concept in terms of memory.

# TODO

- [ ] Y `Y<T>`;
- [ ] X `X<T>`:
  - [ ] X1 `X1<T>`;
  - [ ] X2 `X2<T>`;
  - [ ] X3 `X3<T>`;
