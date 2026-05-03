<p align="center">
  <img src="https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=black" />
  <img src="https://img.shields.io/badge/SvelteKit-FF3E00?style=for-the-badge&logo=svelte&logoColor=white" />
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" />
</p>

<h1 align="center">🛡️ Rodroid Il2CppDumper — Desktop & Mobile GUI</h1>

<p align="center">
  <b>A modern, cross-platform GUI for the Rodroid Il2CppDumper engine.</b><br/>
  Built with <a href="https://tauri.app/">Tauri 2</a>, <a href="https://kit.svelte.dev/">SvelteKit</a>, and <a href="https://www.typescriptlang.org/">TypeScript</a> — runs natively on Windows, macOS, Linux, Android, and iOS.
</p>

<p align="center">
  <a href="https://t.me/+WmudnO0-xoNhMDQ8">📢 Telegram Channel</a> &nbsp;·&nbsp;
  <a href="https://t.me/+QylrYL1GNsJiYjc0">💬 Telegram Group</a> &nbsp;·&nbsp;
  <b>Dev:</b> <a href="https://t.me/rodroidmods"><code>@rodroidmods</code></a>
</p>

---

## ✨ What's New in V5

- **Total UI overhaul** — redesigned card-based layout, refined typography, theme tokens, smooth transitions
- **Live streaming logs** — real-time progress for binary loading, format detection, registration search, and output generation
- **Per-step status** — CodeRegistration / MetadataRegistration addresses are now displayed in every search path (Mach-O `__mod_init_func`, helper section search, NSO, WASM)
- **Full configuration panel** — every CLI flag exposed as a toggle, including all 7 new disassembly engine features
- **CODM toggle** — one switch to enable Call of Duty Mobile's custom v23 metadata layout (Android ELF 32/64 + iOS Mach-O 32/64)
- **Mobile-ready** — works on Android and iOS with native file pickers and platform-specific file copying
- **Auto Unity version detection** — shows Unity build label as soon as you pick the binary

---

## 🖼️ Screens

```
┌─────────────────────────────────────┐
│  🛡️  Rodroid Il2CppDumper           │
│  ���────────────────────────────────  │
│  📂 Binary    [libil2cpp.so]   ✓    │
│       ELF64 · Unity 2022.3.62f2     │
│  📦 Metadata  [global-metadata.dat]✓│
│                                     │
│  ⚙️  Dump Options                   │
│                                     │
│  ▶  Start Dump                      │
└─────────────────────────────────────┘
```

The configuration dialog exposes:

- **Output**: methods, fields, properties, attributes, offsets, typedef indices, assembly names, split-per-type
- **Generation**: structs, DummyDLLs, tokens, generics dump
- **C++ Headers**: scaffold, name mangling, IDA metadata, Unity headers, topological sort, GCC/MSVC layout
- **Disassembly**: target (dump.cs / DiffableCS / both), hex bytes, field names, annotations, CFG, max instructions
- **Advanced**: force IL2CPP version, force dump, no-redirected-pointer, **CODM**

---

## 🚀 Features

All capabilities of the [il2cpp_dumper](../il2cpp_dumper/README.md) CLI are available through the GUI:

### Core
- IL2CPP **v16 – v39** (Unity 5.3 → Unity 6) including v104/v106 undocumented formats
- Auto XOR metadata decryption (1-byte, 4-byte, 8-byte, rolling, position-dependent)
- ELF / PE / Mach-O / Fat Mach-O / NSO / WASM
- Variable-width indices for v39 / Unity 6
- Auto-numbered output directories (Dump0/, Dump1/...)

### Disassembly Engine (V5 highlights)
- **Backward slicing** — resolves `BLR Xn` virtual calls into `// virtual call: TypeName.MethodName`
- **Init-check folding** — collapses `il2cpp_codegen_initialize_method` / `Il2CppCodeGenWriteBarrier` / TBZ-on-bit-0 prologues into a single `// [init check]` annotation
- **String literal indirect resolution** — annotates `il2cpp_string_new_wrapper` calls with the actual literal content
- **Generic instantiation tracking** — annotates calls with the concrete specialization (`// → List<int>.Add(this, item)`)
- **Switch table reconstruction** — detects ARM64 jump tables and emits `switch (var)` blocks in the CFG
- **Boxing / unboxing detection** — annotates `il2cpp_codegen_box` / `il2cpp_unbox` with the resolved type
- **Static field access annotation** — resolves the `ADRP+ADD → static_fields → field_offset` chain
- Plus everything from V4: forward constant propagation, register-pair memory access, semantic variable tracking, ARM64/ARM32/x86/x64 multi-arch decoders

### CODM (Call of Duty Mobile)
- Custom v23 metadata layout with two-slot `type_definitions_count` fingerprint anchor
- Android packed relocations (`DT_ANDROID_RELA` / `DT_ANDROID_REL`, APS2 + SLEB128) — 32-bit and 64-bit ELF
- iOS chained fixups (`LC_DYLD_CHAINED_FIXUPS`) and legacy rebase opcodes (`LC_DYLD_INFO_ONLY`) — 32-bit and 64-bit Mach-O
- Pointer formats: `DYLD_CHAINED_PTR_64`, `_64_OFFSET`, ARM64E
- Toggle from the **Advanced** section of the config dialog — additive code path, leaves standard Unity games untouched

---

## 📦 Installation

### Prebuilt Binaries
Grab the latest installer for your platform from the [Releases](https://github.com/rodroidmods/il2cpp-dumper-rs/releases) page.

| Platform | Artifact |
|----------|----------|
| Windows | `Rodroid-Il2CppDumper-setup.exe` / `.msi` |
| macOS | `Rodroid-Il2CppDumper.dmg` |
| Linux | `.AppImage` / `.deb` / `.rpm` |
| Android | `.apk` |
| iOS | `.ipa` (sideload via AltStore / TrollStore) |

### From Source

Requirements:
- [Node.js](https://nodejs.org/) ≥ 18 + [pnpm](https://pnpm.io/)
- [Rust](https://rustup.rs/) stable toolchain
- Tauri prerequisites for your OS — see the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

```bash
git clone https://github.com/rodroidmods/il2cpp-dumper-rs.git
cd il2cpp-dumper-rs/rodroid-il2cppdumper
pnpm install
```

Run in dev mode:
```bash
pnpm tauri dev
```

Build a release bundle:
```bash
pnpm tauri build
```

Mobile targets:
```bash
pnpm tauri android init
pnpm tauri android dev      # or: pnpm tauri android build

pnpm tauri ios init
pnpm tauri ios dev          # or: pnpm tauri ios build
```

---

## 🔧 Usage

1. Launch the app
2. **Pick a binary** — `libil2cpp.so`, `GameAssembly.dll`, `UnityFramework`, `.nso`, or `.wasm`
3. **Pick a metadata file** — usually `global-metadata.dat`
4. *(Optional)* Open **Dump Options** and tweak generation, disassembly, or enable **CODM**
5. Hit **Start Dump** — watch the live log and find the output in an auto-numbered `Dump0/`, `Dump1/`, … folder

---

## 🏗️ Architecture

```
rodroid-il2cppdumper/
├── src/                              # SvelteKit frontend
│   ├── lib/
│   │   ├── components/
│   │   │   ├── IdleScreen.svelte     # binary/metadata picker
│   │   │   ├── ConfigDialog.svelte   # full options panel
│   │   │   ├── DumpingScreen.svelte  # live log + progress
│   │   │   └── DoneScreen.svelte     # output summary
│   │   ├── stores.ts                 # app state, config, i18n
│   │   └── types.ts                  # DumperConfig, BinaryInfo
��   └── routes/+page.svelte           # screen router
├── src-tauri/                        # Rust backend
│   ├── src/
│   │   ├── lib.rs                    # Tauri commands: detect_binary, run_dump
│   │   └── main.rs
│   ├── tauri.conf.json
│   └── Cargo.toml                    # depends on ../il2cpp_dumper
└── package.json
```

The backend wraps the [`il2cpp_dumper`](../il2cpp_dumper) Rust crate as a library and streams progress events back to the frontend via Tauri's event system.

---

## 📜 License

MIT

---

## 🙏 Credits

- [Perfare/Il2CppDumper](https://github.com/Perfare/Il2CppDumper) — Original C# implementation
- [SamboyCoding/Cpp2IL](https://github.com/SamboyCoding/Cpp2IL) — Advanced IL2CPP analysis tool
- [tauri-apps](https://tauri.app/) — Cross-platform native shell
- [SvelteKit](https://kit.svelte.dev/) — Frontend framework
- [console-rs](https://github.com/console-rs) — Terminal styling ecosystem reused for the CLI

---

## 📬 Community

| | Link |
|---|---|
| 📢 **Telegram Channel** | [Join Channel](https://t.me/+WmudnO0-xoNhMDQ8) |
| 💬 **Telegram Group** | [Join Group](https://t.me/+QylrYL1GNsJiYjc0) |
| 👤 **Developer** | [`@rodroidmods`](https://t.me/rodroidmods) |

---

> **⚠️ Disclaimer**: This tool is for educational and research purposes only. Respect game developers' rights and terms of service.
