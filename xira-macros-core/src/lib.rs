//! Xira 过程宏的核心逻辑.
//!
//! 包含解析, 生成和分析三个阶段的实现.
//! 与 `xira-macros` 分离, 以便单元测试和 IDE 支持.

mod analyze;
mod expend;
mod parse;
mod tests;
