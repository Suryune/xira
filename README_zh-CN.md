# Xira

[English](README.md) | 中文
Xira 是一个实验性的 Rust UI 框架。  
核心理念是统一、确定性和性能。  

核心模型详见 [docs/core-model_zh-CN.md](docs/core-model_zh-CN.md)。

## 统一

所有 Widget 都只是 Widget，不持有数据。  
所有数据由框架统一管理，生命周期由框架统一调度。  
Widget 不需要管生命周期，也不需要区分数据层和视图层。  
异步任务作为独立组件存在，生命周期由开发者管理。  


因为数据统一管理，框架可以为同质数据自动实现 SoA 布局，  
缓存友好，批量处理高效。  

## 确定性

依托编译期的代码分析和 ID 系统，Xira 实现 UI 的确定性：  

- 每次变化都有 ID 和原因
- 只更新真正需要更新的 Widget，不做广播
- 调试信息可精确到调用者和具体函数

## 性能

数据集中 + SoA + 点对点更新，三者的组合让 Xira 在  
状态变化时只触碰真正相关的数据。  

## 组件

目标是提供一套符合 Material Design 3 的组件。  

## 为什么需要 Xira

现有 Rust UI 框架在数据所有权、生命周期管理、依赖追踪上各有取舍。  
Xira 尝试用"中央 store + 静态 ID + 编译期依赖分析"统一这三个问题。  

## 当前状态

Xira 是一个学生项目，没有截止日期。

它的设计理念要求从底层开始构建，工程量不小。我无法承诺它会在  
什么时候变成一个可用的框架，甚至无法承诺它一定会变成。  

但我希望 Xira 的理念本身能启发更多项目。  

## 致谢

Xira 的底层基础设施依赖 Vello、Taffy、Parley、winit、wgpu。  

底层对接方式参照了 Masonry 的实现。  
MD3 组件的设计参照了 Flutter 的 Material 库。  

## License


本项目采用 Apache-2.0 OR MIT 双许可。你可以根据自身需求选择其中任意一种：  
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) 或 <http://www.apache.org/licenses/LICENSE-2.0>)  
- MIT license ([LICENSE-MIT](LICENSE-MIT) 或 <http://opensource.org/licenses/MIT>)  

---

```text
 __  ___          
 \ \/ (_)_ _ __ _ 
  >  <| | '_/ _` |
 /_/\_\_|_| \__,_|
```           
