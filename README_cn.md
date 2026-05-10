# hide_info

`hide_info` 是一个 Rust 项目，用于将数据隐藏到图片中。

## 在线体验

| 功能          | 状态 | 在线试用                                                                                       |
| ----------- | -- |--------------------------------------------------------------------------------------------|
| hide_as_img | ✅  | [https://www.guofei.site/os/hide_as_img.html](https://www.guofei.site/os/hide_as_img.html) |
| mirage_tank | ✅  | [https://www.guofei.site/os/hide_as_img.html](https://www.guofei.site/os/hide_as_img.html) |

## 功能特性

* `hide_as_img`：将任意二进制数据/文件隐藏到 PNG 图片中。
* `mirage_tank`：将一张 RGB 图片和一张灰度图合成为 RGBA 图片，实现轻量级“幻影坦克”效果。

## Rust 库使用方法

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
hide_info = "0.1"
```

然后在 Rust 中调用：

```rust
use hide_info::hide_as_img::HideAsImg;

let data = b"secret binary data";

let hide_as_img = HideAsImg::new();

let png_bytes = hide_as_img.encode(data)?;
let decoded = hide_as_img.decode(&png_bytes)?;

assert_eq!(decoded, data);
```

仓库中的 `./tests` 目录还包含更多示例代码。

## CLI 使用方法

安装命令行工具：

```sh
cargo install hide_info
```

### hide_as_img 编码/解码

编码：

```sh
hide_info hide_as_img encode \
  --input file.zip \
  --output png.png
```

解码：

```sh
hide_info hide_as_img decode \
  --input png.png \
  --output file.zip
```

### mirage_tank

```sh
hide_info mirage_tank \
  --input1 png1.png \
  --input2 png2.png \
  --output output.png \
  --a 0.5
```

其中：

* `--a` 为可选参数，默认值为 `0.5`

## 说明

* `hide_as_img` 不提供加密功能，也不包含纠错机制。
* 如果需要保密性，请在隐藏前自行对数据进行加密或加密码保护。
* 如果需要抗损坏能力，请自行增加校验、冗余或纠错机制。当前算法本身不具备抗破坏能力。

## 测试

仓库中包含集成测试：

```sh
cargo test
```
