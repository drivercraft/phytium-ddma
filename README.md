# Phytium DDMA 驱动

Phytium DDMA（Direct Memory Access）控制器的 Rust 驱动程序，提供高性能的内存到外设和外设到内存的数据传输功能。

## 特性

- 支持 8 个 DMA 通道
- 支持内存到外设和外设到内存的传输
- 支持中断和轮询模式
- 提供安全的 Rust API 封装
- 支持超时配置
- 兼容 Phytium 芯片的 DDMA 控制器

## 开发和测试

### 安装依赖

安装 `ostool`：

```bash
cargo install ostool
```

### 运行测试

在模拟器中测试：

```bash
cargo test --test test -- tests --show-output
```

在带有 U-Boot 的开发板上测试：

```bash
cargo test --test test -- tests --show-output --uboot 
```

调试测试（仅编译不运行）：

```bash
cargo test --test test -- --show-output --no-run
```

### 项目结构

```text
src/
├── lib.rs     # 主要的 DDMA 控制器实现
├── chan.rs    # DMA 通道实现
└── reg.rs     # 寄存器定义和操作
examples/
└── dma_examples.rs  # 使用示例
tests/
└── test.rs    # 集成测试
```

## 硬件要求

- Phytium 系列芯片
- 支持 DDMA 控制器的硬件平台
- AArch64 架构

## 许可证

本项目采用开源许可证，具体请查看 LICENSE 文件。

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目。

## 注意事项

- 这是一个 `no_std` 库，适用于嵌入式环境
- 需要正确配置内存映射和中断处理
- 在使用前请确保硬件平台支持相应的外设
