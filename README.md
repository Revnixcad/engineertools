# engineertools.nl 🛠️

_Started as my personal project out of passion for programming, technology, and engineering._

**engineertools.nl** is a fast, WebAssembly-powered website built with [Rust](https://www.rust-lang.org/), [Leptos](https://leptos.dev/), and [Fermyon Spin](https://www.fermyon.com/). It provides engineers with a growing collection of calculators and tools to support their daily work—available directly from the browser.

This project serves as both an open-source playground for modern WASM-based microservices and a practical utility hub for technical professionals.

---

## 🚀 Live Site

🔗 https://www.engineertools.nl

---

## 🧱 Tech Stack

- **Rust** – safe, fast systems language
- **Leptos** – reactive frontend framework for Rust and WebAssembly
- **Spin** – cloud-native WASM app runtime by Fermyon
- **Wasm32** – compiled to WebAssembly for efficient execution

---

## 🔧 Local Development

### Prerequisites

Install Rust with the required target:

```bash
rustup target add wasm32-wasip1
```

Install the Spin CLI:

```bash
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash
sudo mv ./spin /usr/local/bin/spin
```

---

### Run the project locally

```bash
# Build the project
spin build

# Serve locally
spin up
```

Open your browser at http://127.0.0.1:3000 to explore the app.

---

## 📦 Deployment

Deployment is handled via GitHub Actions, which:

- Builds the project using `cargo leptos build`
- Targets `wasm32-unknown-unknown`
- Deploys to Fermyon Cloud using `spin deploy`

> Ensure your local `wasm-bindgen` CLI version matches the one in your project.

---

## 🧮 Features (WIP)

- 🧰 Engineering calculators for mechanics, hydraulics, and more
- 🔎 Material property lookups
- 🔄 Unit converters
- 📱 Mobile-first, lightweight UI
- 🧠 Fully client-side with WASM performance

---

## 🤝 Contributing

Contributions are welcome! Got an idea for a new tool? Found a bug? Submit a pull request or open an issue.

---

## 📣 Acknowledgments

Powered by:

- Fermyon Spin
- Leptos Framework
- Rust Wasm Tooling

Big thanks to the open-source community for tools and support.
