# 🦀 My Rust Learning Roadmap ✅

This roadmap tracks my Rust learning journey. Each project can be **checked off** when completed.

---

## **Stage 1 – Core Concepts**

*Goal: Syntax, variables, ownership, Option, enums, match*

* [x] **Guessing Game** – Variables, mutability, control flow, I/O, count attempts, give hints
* [ ] **Contact Book CLI** – Store names + numbers in a `Vec`, search returns `Option<Contact>`, practice `Some`/`None`
* [ ] **Todo List CLI** – Enum `Command { Add, List, Complete }`, match user commands, ownership with tasks

---

## **Stage 2 – Structs, Ownership, Borrowing**

*Goal: Data modeling and safe memory management*

* [ ] **Bank Account System** – `struct Account { owner: String, balance: u32 }`, deposit/withdraw, borrowing vs mutable borrowing
* [ ] **Library Management System** – Track `Book { title, author, available }`, borrow/return books, references vs ownership

---

## **Stage 3 – Enums, Pattern Matching, Error Handling**

*Goal: Rust’s type system and safe error handling*

* [ ] **ATM Simulation** – Enum for actions, match to execute, handle errors with `Result<T, E>`
* [ ] **Simple Calculator** – Parse input like `"5 + 3"`, enum for operators, match to execute

---

## **Stage 4 – Collections, Iterators, File I/O**

*Goal: Real-world Rust applications*

* [ ] **Word Frequency Counter** – Read text file, count word frequencies with `HashMap<String, u32>`, iterators
* [ ] **Todo App with Persistence** – Save/load tasks (JSON/CSV), file I/O, `serde` serialization

---

## **Stage 5 – Advanced Concepts**

*Goal: Traits, generics, modules, API interactions*

* [ ] **Mini Chat App (local)** – Multiple users in struct, messages vector, traits for printing
* [ ] **Weather CLI (API fetch)** – Fetch JSON via `reqwest`, parse into structs, error handling with `Result`
* [ ] **Command-Line Game (Tic Tac Toe)** – 2D array board, players enum, match, borrowing, state updates

---

## **Stage 6 – Portfolio-Level Project**

*Goal: Combine everything learned*

* [ ] **Personal Finance Tracker** – Track expenses/income, structs + enums, save/load (JSON/CSV), CLI commands, error handling, optional web dashboard

---

## **🔥 Goals After Completing Roadmap**

* Master **structs, enums, Option, Result, match, ownership/borrowing**
* Learn **collections, iterators, traits, file I/O, modules, error handling**
* Build a **portfolio of Rust projects** demonstrating my skills

---