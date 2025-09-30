# 🦀 My Rust Learning Roadmap

This roadmap is my personal plan to learn Rust step by step. Each stage focuses on specific Rust concepts, and each project helps me practice them in a hands-on way.

---

## **Stage 1 – Core Concepts**

*Goal: Get comfortable with Rust syntax, variables, ownership basics, Option, enums, and `match`*

### **Projects**

1. **Guessing Game**

   * Practice: variables, mutability, control flow, input/output
   * Add-ons: count attempts, give hints (“too high/too low”)

2. **Contact Book CLI**

   * Practice: structs, Vecs, searching with `Option<Contact>`
   * Learn: `Some` / `None`, ownership when storing/retrieving contacts

3. **Todo List CLI**

   * Practice: enums (`Command { Add, List, Complete }`)
   * Learn: using `match` for user commands, ownership for adding/removing tasks

---

## **Stage 2 – Structs, Ownership, Borrowing**

*Goal: Model data safely and understand borrowing*

### **Projects**

1. **Bank Account System**

   * `struct Account { owner: String, balance: u32 }`
   * Functions for deposit/withdraw
   * Learn: borrowing vs mutable borrowing (`&` vs `&mut`)

2. **Library Management System**

   * Track `Book { title, author, available }`
   * Borrowing/returning updates availability
   * Learn: references vs ownership

---

## **Stage 3 – Enums, Pattern Matching, Error Handling**

*Goal: Use Rust’s type system to handle choices and errors safely*

### **Projects**

1. **ATM Simulation**

   * Enum for actions (`Withdraw`, `Deposit`, `CheckBalance`)
   * Use `match` to execute actions
   * Handle errors with `Result<T, E>`

2. **Simple Calculator**

   * Parse input like `"5 + 3"`
   * Enum for operators (`Add`, `Subtract`, etc.)
   * Use `match` to execute calculations

---

## **Stage 4 – Collections, Iterators, File I/O**

*Goal: Learn how to handle data like in real-world apps*

### **Projects**

1. **Word Frequency Counter**

   * Read a text file
   * Count word frequencies with `HashMap<String, u32>`
   * Use iterators (`.iter()`, `.map()`, `.filter()`)

2. **Todo App with Persistence**

   * Save tasks to JSON or CSV
   * Load tasks on startup
   * Practice file I/O (`std::fs`) and serialization (`serde`)

---

## **Stage 5 – Advanced Concepts**

*Goal: Practice traits, generics, modules, and API interactions*

### **Projects**

1. **Mini Chat App (local)**

   * Multiple users in a struct
   * Messages stored in a vector
   * Practice traits (`Display`) for pretty printing

2. **Weather CLI (API fetch)**

   * Fetch JSON from an API (`reqwest`, `serde_json`)
   * Parse into structs
   * Practice error handling with `Result`

3. **Command-Line Game (Tic Tac Toe)**

   * Board = 2D array
   * Players = `enum Player { X, O }`
   * Practice `match`, borrowing, and state updates

---

## **Stage 6 – Portfolio-Level Project**

*Goal: Combine everything I’ve learned into a polished project*

### **Personal Finance Tracker**

* Track expenses + income
* Use `struct Transaction` and `enum Category`
* Save/load from file (JSON/CSV)
* Use `Result` for error handling
* CLI commands with `match`
* Optional: build a web dashboard using Rocket or Actix

---

## **🔥 My Goals After This Roadmap**

By completing these stages, I will:

* Master **structs, enums, Option, Result, match, ownership/borrowing**
* Learn **collections, iterators, traits, file I/O, modules, error handling**
* Build a **portfolio of Rust projects** demonstrating my skills