# 🦀 Rust CLI Banking System

> Pet project for learning the Rust programming language\
> Built from scratch to understand core Rust concepts through real-world
> logic.

------------------------------------------------------------------------

## 📌 About This Project

This is a pet project created for educational purposes to deeply learn
the Rust programming language.

The goal of this project is not just to write code --- but to
understand:

-   Ownership & Borrowing
-   Structs & Methods
-   Modules
-   File I/O
-   JSON serialization
-   Error handling
-   CLI interaction
-   Project structure in Rust

This project simulates a simple banking system running in the terminal.

------------------------------------------------------------------------

## 🏦 Features

-   Account creation\
-   Login system\
-   Password handling\
-   Account information display\
-   Money manipulation\
-   JSON-based storage\
-   Modular architecture

------------------------------------------------------------------------

## 🛠 Project Structure

src/ │ ├── main.rs \# Entry point\
├── accountinfo.rs \# Account information logic\
├── money_manipulations.rs \# Deposit / Withdraw / Transfers\
├── saveload.rs \# JSON save & load logic\
├── password.rs \# Password validation logic

------------------------------------------------------------------------

## ⚙️ How To Run

Make sure you have Rust installed:

    rustc --version
    cargo --version

Run the project:

    cargo run

Build release version:

    cargo build --release

------------------------------------------------------------------------

## 📦 Technologies Used

-   Rust\
-   serde\
-   serde_json\
-   File I/O\
-   Command Line Interface

------------------------------------------------------------------------

## 🧠 Learning Focus

This project is designed to practice:

-   Rust ownership model\
-   Borrow checker behavior\
-   Working with Option and Result\
-   Struct implementation blocks\
-   Modular Rust design\
-   Handling user input\
-   Persistent storage using JSON

------------------------------------------------------------------------

## 🔮 Future Improvements

-   Password hashing\
-   Multi-currency support using HashMap\<String, f64\>\
-   Docker containerization\
-   REST API using Axum\
-   PostgreSQL database integration\
-   Authentication system upgrade

------------------------------------------------------------------------

## 👨‍💻 Author

Pavel (scen1c)

------------------------------------------------------------------------

## 📚 Why This Project?

This is a foundational backend-style project built while learning Rust.

CLI → Modular Architecture → Backend → Production-level Systems
