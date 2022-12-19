+++
title = "Architecture"

[extra]
order = 1
+++

- rustc: Compiler for node graph generics and custom nodes
- rust-gpu: Compiler backend to generate compute shaders from Rust source code
- wgpu: Portable graphics API for running compute shaders on desktop and web
- Vello: GPU-accelerated vector graphics renderer
- COSMIC Text: Text shaping and typesetting
- Wasmer or Wasmtime: Portable, sandboxed runtime for custom nodes
- Tokio: parallelized job execution in the node graph pipeline
- Tauri: lightweight desktop web UI shell while the backend runs natively
- Xilem: High-performance native UI framework, to replace Tauri when ready
