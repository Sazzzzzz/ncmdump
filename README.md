# 🎵 NCMDump

> 一款 Rust 编写的网易云音乐 `.ncm` 解密工具

[![Rust](https://img.shields.io/badge/made%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)]()

---

## ✨ 简介

[NCMDump](https://github.com/Sazzzzzz/ncmdump) 能把网易云音乐的加密文件 `.ncm` 还原成普通音频文件（`.mp3` / `.flac` 等），支持**批量处理**。

**主要特性**：

+ **使用简单**，下载后只需要放在网易云下载目录下别管，双击拖拽即可。
+ **纯 Rust** 实现，安全高效
+ **零依赖**，单文件可执行，下载即用

---

## 🚀 使用方法

### 🖥️ Windows / macOS

在[Release](https://github.com/Sazzzzzz/ncmdump/releases/tag/latest)页面下载对应平台的可执行文件，放在网易云音乐的下载目录。

+ 双击运行：解压当前目录的所有 `.ncm` 文件
+ 拖拽文件（Windows）：将一个或多个 `.ncm` 文件拖拽到`.exe` 文件上，即可解密。

### ⌨️ 命令行

```bash
# 解密当前目录所有 .ncm 文件
ncmdump

# 指定一个或多个文件
ncmdump song1.ncm song2.ncm
```

> [!NOTE]
>
> 命令行使用时，`ncmdump` 先扫描当前shell的 cwd，如果未发现任何 `.ncm` 文件，再扫描可执行文件所在目录。
