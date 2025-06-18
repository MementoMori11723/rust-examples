# 🦀 Rust Examples

A collection of simple Rust projects to demonstrate how to build:

- ✅ A basic **Hello World** CLI
- 🎮 A **Sample Game** using `raylib`
- 🌐 A basic **Web Server** that serves HTML pages

---

## 📁 Directory Structure

```bash
rust-examples
├── hello-world       # Minimal Hello World CLI app
├── sample-game       # A simple interactive game
└── web-server        # Basic web server with static HTML support

````

---

## 🚀 Getting Started

### ✅ 1. Hello World

A simple CLI program to verify your Rust setup.

**Run it:**

```bash
cd hello-world
make run
````

---

### 🎮 2. Sample Game

A basic 2D game using [raylib](https://crates.io/crates/raylib) and Rust bindings.

**Dependencies:**

Install `raylib` system dependencies:

```bash
# Debian/Ubuntu
sudo apt install libraylib-dev
```

**Run it:**

```bash
cd sample-game
make run
```

---

### 🌐 3. Web Server

A minimal static file server written in Rust that serves HTML pages from the `pages/` folder.

**Run it:**

```bash
cd web-server
make run
```

**Visit:**

Open [http://localhost:11500](http://localhost:11500) in your browser.

---

## 🛠 Requirements

* Rust (via [rustup.rs](https://rustup.rs/))
* Make (for using the provided `Makefile`)
* Optional system libraries (e.g. `libraylib-dev`)
