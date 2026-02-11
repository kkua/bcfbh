# BCFBH - Booklet Creator For Binding by Hand

一个用于将大型PDF文件拆分为多个小册子（booklet）的Rust工具。

## 功能

- 将大型PDF文件按指定纸张数量拆分为多个小册子
- 自动计算每册的最佳页数分配
- 支持智能页数对齐（自动对齐到4的倍数）
- 保留原始PDF的页面内容

## 暂未实现的功能
- 自动页码
- 按小册子模式重新排版成PDF文件，适配中间装订、两边装订。由于打印机支持小册子且可选择装订位置，故可能不会实现该功能
- 添加中缝装订线
- 调用打印机打印，自动设置打印参数

## 安装

确保已安装Rust环境，然后克隆并构建项目：

```bash
git clone https://codeberg.org/Kaay/bcfbh
cd bcfbh
cargo build --release
```

## 使用方法

编辑 `src/main.rs` 中的配置参数：

```rust
let booklet_config = booklet::BindingRule{
    input_path: PathBuf::from("input.pdf"),      // 输入PDF文件路径
    output_dir: PathBuf::from("out"),              // 输出目录
    sheets_per_booklet: 10,                        // 每个小册子的A4纸张数量（默认10张，即40页）
};
booklet::split_pdf(&booklet_config);
```

然后运行：

```bash
cargo run
```

## 配置参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `input_path` | `PathBuf` | 输入PDF文件的完整路径 |
| `output_dir` | `PathBuf` | 输出目录路径 |
| `sheets_per_booklet` | `usize` | 每个小册子包含的A4纸张数量，每张纸可打印4页（双面打印，每面2页） |

## 输出文件

程序将生成多个PDF文件，命名格式为 `booklet_XX.pdf`，其中 `XX` 为两位数序号（如 `booklet_01.pdf`, `booklet_02.pdf` 等）。

## 算法说明

拆分算法会智能处理以下情况：

1. **页数对齐**：自动将总页数对齐到4的倍数（因为每张A4纸可打印4页）
2. **均匀分配**：当剩余页数较少时，会将页数均匀分配到各册
3. **增量分配**：当剩余页数适中时，前几册会多分配1张纸

## 依赖

- [oxidize-pdf](https://crates.io/crates/oxidize-pdf) (1.6.13) - PDF处理库

## 项目结构

```
bdfb/
├── Cargo.toml          # 项目配置
├── src/
│   ├── main.rs         # 程序入口
│   ├── booklet.rs      # 小册子拆分逻辑
│   └── pdf_edit.rs     # PDF编辑工具函数
└── README.md           # 本文件
```

## 许可证

MIT License
