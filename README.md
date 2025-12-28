# Zed

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

---

## 🪟✨ Multi-Window Editing — Fork with Multi-Monitor Superpowers!

> **📢 Note:** This is a **fork** of the original [Zed Editor](https://github.com/zed-industries/zed). We're actively working on multi-window support and plan to submit these changes upstream once they're polished. Fingers crossed they accept! 🤞
>
> **🖥️ Tested on:** Windows machines so far. Need help testing in more platforms!

Say goodbye to the single-window life! 🎉 This fork brings **true multi-window editing** to Zed, letting you spread your code across multiple monitors like a pro.

### 🚀 What's New?

| Feature | Description |
|---------|-------------|
| 🪟 **Secondary Editor Windows** | Open as many editor-only windows as you need — perfect for multi-monitor setups! |
| 🎯 **Smart File Routing** | Click a file in the project tree and it opens in whichever window you're working in |
| ⚡ **Real-Time Sync** | Edit a file in one window, see changes *instantly* in all other windows showing the same file |
| 🔴 **Dirty Indicators Everywhere** | Unsaved file? The indicator shows up in ALL windows — no more confusion! |
| 🐛 **Run & Debug Anywhere** | Hit that debug button from any window — it just works! |
| 🖱️ **Drag Tabs to New Windows** | Ideally we would want to drag any tab to a different monitor to spawn a new window, but will try to implement if these changes are accepted by Zed team |
| 📊 **Status Bar in Every Window** | Cursor position and Outline Panel toggle available everywhere |
| 💾 **Session Restore** | Close Zed, reopen it — all your windows and tabs come back exactly as you left them |

---

### 📸 See It In Action!

#### 🆕 Create a New Editor Window from Project Panel
Right-click any file in the project panel to open it in a new window!

![New Editor Window From Project Panel](specs/01%20New%20Editor%20Window%20From%20Project%20Panel.png)

#### 🪟 Your New Editor Window
A clean, focused editor-only window ready for your code!

![After Opening New Editor Window](specs/02%20After%20Opening%20New%20Editor%20Window.png)

#### 🐛 Run & Debug from Any Window
No need to switch to the main window — run and debug right where you are!

![Can Run and Debug From Secondary Window As Well](specs/03%20Can%20Run%20and%20Debug%20From%20Secondary%20Window%20As%20Well.png)

#### ⚡ Real-Time Sync Across Windows
Same file in multiple windows? They stay perfectly in sync with every keystroke!

![Same file in multiple windows are always in sync with each keystroke](specs/04%20Same%20file%20in%20multiple%20windows%20are%20always%20in%20sync%20with%20each%20keystroke.png)

#### 🖱️ Right-Click Tab to Open in New Window
Even your tab headers support opening files in new windows!

![Files can be opened in new editor window even by right clicking on tab header](specs/05%20Files%20can%20be%20opened%20in%20new%20editor%20window%20even%20by%20right%20clicking%20on%20tab%20header.png)

#### 🪟🪟🪟 Open As Many Windows As You Need
Multi-monitor heaven — spread your code across all your screens!

![Can Open Multiple Secondary Windows As Needed](specs/06%20Can%20Open%20Multiple%20Secondary%20Windows%20As%20Needed.png)

#### 🎯 Debugging with Inline Values Everywhere
Hit a breakpoint and see inline values in ALL windows showing that file!

![Same File in Multiple Windows Hit Breakpoint And Show Inline Values](specs/07%20Same%20File%20in%20Multiple%20Windows%20Hit%20Breakpoint%20And%20Show%20Inline%20Values.png)

---
### The original readme content follows below

Welcome to Zed, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

---

### Installation

On macOS, Linux, and Windows you can [download Zed directly](https://zed.dev/download) or install Zed via your local package manager ([macOS](https://zed.dev/docs/installation#macos)/[Linux](https://zed.dev/docs/linux#installing-via-a-package-manager)/[Windows](https://zed.dev/docs/windows#package-managers)).

Other platforms are not yet available:

- Web ([tracking issue](https://github.com/zed-industries/zed/issues/5396))

### Developing Zed

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Zed.

Also... we're hiring! Check out our [jobs](https://zed.dev/jobs) page for open roles.

### Licensing

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).
