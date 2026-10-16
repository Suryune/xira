# Xira

[中文](README_zh-CN.md) | English  
Xira is an experimental Rust UI framework.  
Its core ideas are unity, determinism, and performance.  

See the [Core Model](docs/core-model.md) for details.

## Unity

All Widgets are just Widgets; they do not hold data.  
All data is managed uniformly by the framework, and lifecycle is scheduled uniformly by the framework.  
Widgets do not need to manage lifecycle, nor do they need to distinguish between the data layer and the view layer.  
Async tasks exist as independent components, and their lifecycle is managed by the developer.  

Because data is managed uniformly, the framework can automatically implement SoA layout for homogeneous data, which is cache-friendly and efficient for batch processing.  

## Determinism

Relying on compile-time code analysis and an ID system, Xira achieves UI determinism:  

- Every change has an ID and a reason
- Only Widgets that truly need updating are updated; no broadcasting
- Debug information can be precise down to the caller and the specific function

## Performance

Centralized data + SoA + point-to-point updates.  
The combination allows Xira to touch only the truly relevant data when state changes.  

## Components

The goal is to provide a set of components that conform to Material Design 3.  

## Why Xira

Existing Rust UI frameworks make different trade-offs in data ownership, lifecycle management, and dependency tracking.  
Xira attempts to unify these three problems with a "central store + static IDs + compile-time dependency analysis" approach.  

## Current Status

Xira is a student project with no deadline.  

Its design philosophy requires building from the ground up, which is a considerable amount of work.  
I cannot promise when it will become a usable framework, or even that it will become one.  

But I hope the ideas behind Xira can inspire more projects.  

## Acknowledgements

Xira's low-level infrastructure depends on Vello, Taffy, Parley, winit, and wgpu.  

The low-level integration approach references Masonry's implementation.  
The design of MD3 components references Flutter's Material library.  

## License

This project is dual-licensed under Apache-2.0 OR MIT. You may choose either one according to your needs:  
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

---

```text
 __  ___          
 \ \/ (_)_ _ __ _ 
  >  <| | '_/ _` |
 /_/\_\_|_| \__,_|
```
