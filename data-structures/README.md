# Info
>Reference: [Illustrated Data Structures](https://youtube.com/playlist?list=PLkZYeFmDuaN2-KUIv-mvbjfKszIGJ4FaY) by https://roadmap.sh/

There is simple "instruction": all data structures are provided as libraries with own set of tests and implementation details.
Implementations of the structures are demonstrated only in the context of data storage and nothing more.

I really try to give clear and simple explanations of each data structure.

>All examples are made **in** and **for** Rust.

# Overview

Information about every data structure is represented in it's own folder's `INFO.md` as:

```markdown
---
title: Data Structure Name
author: Kirill <fidelicura> Bezuglyi
---

Main Information
---

# Overview
Brief explanation of **the principle** of this data structure;

# Advantages
Common advantages **by design** of this data structure;

# Disadvantages
Common disadvantages **by design** of this data structure;

# Usecases
Highly **preferred** use cases of this data structure;

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

### Linked List
This library demonstrates some basic techniques around `LinkedList<T>`.
Also, it provides simple and basic implementation of `LinkedList<T>` by hand.

### Stack
This library provides simple and basic implementation of `Stack<T>` by hand.
Also, this library demonstrates some basic techniques around `Stack<T>`.
>Do not confuse `Stack<T>` data structure with the same concept in terms of memory.

### Double Ended Queue
This library demonstrates some basic techniques around `Dequeue<T>`.
Also, it provides simple and basic implementation of `Dequeue<T>` by hand.
>Do not confuse `Dequeue<T>` with `Queue<T>`

### Hash Set
This library demonstrates some basic techniques around `HashSet<T>`.
Also, it provides simple and basic implementation of `HashSet<T>` by hand.

### Hash Map
This library demonstrates some basic techniques around `HashMap<T>`.
Also, it provides simple and basic implementation of `HashMap<T>` by hand.

### Bit Array
This library demonstrates some basic techniques around `BitArray<T>`.
Also, it provides simple and basic implementation of `BitArray<T>` by hand.

### Binary Trees
This library provides simple and basic implementation of `BT<T>`, `FBT<T>` and `BST<T>` by hand.
Also, this library demonstrates some basic techniques around `BT<T>`, `FBT<T>` and `BST<T>`.

### Directed Acyclic Graph
This library provides simple and basic implementation of `DAG<T>` by hand.
Also, this library demonstrates some basic techniques around `DAG<T>`.

### Heap
This library provides simple and basic implementation of `Heap<T>` by hand.
Also, this library demonstrates some basic techniques around `Heap<T>`.
>Do not confuse `Heap<T>` data structure with the same concept in terms of memory.

# TODO

- [x] Move to [presenterm](https://github.com/mfontanini/presenterm);
  - [x] Change documentation from [slides](https://github.com/maaslalani/slides/) to it;
  - [x] Add custom tags to `INFO.md`;

- [ ] Vector `Vector<T>`;
- [ ] Linked List `LinkedList<T>`;
- [ ] Stack `Stack<T>`;
- [ ] Dequeue `Dequeue<T>`;
- [ ] Hash Set `HashSet<T>`;
- [ ] Hash Map `HashMap<T>`;
- [ ] Binary Trees:
  - [ ] Binary Tree `BT<T>`;
  - [ ] Full Binary Tree `FBT<T>`;
  - [ ] Binary Search Tree `BST<T>`;
- [ ] Directed Acyclic Graph `DAG<T>`;
- [ ] Heap `Heap<T>`;
