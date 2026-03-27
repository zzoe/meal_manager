# Meal Manager 打包指南

基于 Makepad 官方打包指南：https://makepad.rs/guide/appendix/packaging-guide

---

## 桌面端打包

### 1. 安装工具

```bash
# 安装 cargo-packager
cargo install cargo-packager --locked

# 安装 robius-packaging-commands (用于自动收集 Makepad 资源)
cargo install --version 0.2.0 --locked --git https://github.com/project-robius/robius-packaging-commands.git robius-packaging-commands
```

### 2. 配置 Cargo.toml

项目已在 `Cargo.toml` 中配置了 `[package.metadata.packager]`，包括：
- 应用信息（名称、标识符、描述）
- 资源文件列表（Makepad 内置资源）
- 平台特定配置

> 注意：`before-packaging-command` 已从 `Cargo.toml` 移除，改为手动执行脚本。
> 因为 `Cargo.toml` 不支持按平台条件判断二进制文件后缀（Windows 为 `.exe`，其他平台无后缀）。

### 3. 各平台打包命令

> **重要**：运行 `cargo packager` 前必须先执行资源收集脚本。

#### Linux (Debian/Ubuntu)

```bash
# 安装系统依赖
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libx11-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxi-dev \
    libxss-dev \
    libwayland-dev \
    libxkbcommon-dev \
    libpulse-dev \
    libasound2-dev \
    libgl1-mesa-dev \
    libudev-dev

# 收集资源
bash scripts/before-packaging.sh

# 打包 (生成 .deb)
cargo packager --release
```

#### Windows

在 Git Bash / MSYS2 中执行：

```bash
# 收集资源
bash scripts/before-packaging.sh

# 打包 (生成 .exe 安装程序)
cargo packager --release --formats nsis
```

#### macOS

```bash
# 收集资源
bash scripts/before-packaging.sh

# 打包 (生成 .dmg)
cargo packager --release
```

### 4. 输出目录

打包产物在 `./dist` 目录：
- `dist/meal_manager_*.deb` - Linux DEB 包
- `dist/meal_manager_*.exe` - Windows NSIS 安装包
- `dist/meal_manager_*.dmg` - macOS DMG

### 5. 资源文件

`robius-packaging-commands` 会自动收集以下 Makepad 资源到 `./dist/resources/`：

| 资源 | 说明 |
|------|------|
| `makepad_widgets` | Makepad 组件库资源（字体、图标） |
| `makepad_fonts_chinese_bold` | 中文字体（粗体） |
| `makepad_fonts_chinese_bold_2` | 中文字体（粗体 2） |
| `makepad_fonts_chinese_regular` | 中文字体（常规） |
| `makepad_fonts_chinese_regular_2` | 中文字体（常规 2） |
| `makepad_fonts_emoji` | Emoji 字体 |

### 6. 运行打包的应用

安装后直接运行即可，程序会自动找到资源文件。

---

## WASM 打包

### 1. 安装工具

```bash
# 安装 cargo-makepad
cargo install --git https://github.com/makepad/makepad --tag 1.0.0 cargo-makepad

# 安装 WASM 工具链
cargo makepad wasm install-toolchain
```

### 2. 构建 WASM

```bash
# 构建并运行（开发）
cargo makepad wasm run -p meal_manager --release

# 或仅构建
cargo makepad wasm --bindgen build -p meal_manager --release
```

### 3. 输出目录

生成的文件位于：
```
target/makepad-wasm-app/release/meal_manager/
├── index.html
├── meal_manager.wasm
├── bindgen.js
├── bindgen_bg.wasm
├── makepad_platform/
├── makepad_widgets/
├── makepad_fonts_emoji/
└── meal_manager/
    └── resources/
```

### 4. 打包发布

```bash
cd target/makepad-wasm-app/release/meal_manager/
tar -czf meal_manager-wasm.tar.gz .
```

### 5. 本地测试

```bash
cd target/makepad-wasm-app/release/meal_manager/
python3 -m http.server 8080
# 访问 http://localhost:8080
```

---

## CI/CD

GitHub Actions 会自动执行以下流程：

### 工作流

1. **native-build** (并行)
   - Ubuntu → DEB
   - Windows → NSIS
   - macOS → DMG

2. **wasm-build**
   - 构建 WASM
   - 打包为 tar.gz

3. **release** (仅 tag 推送时)
   - 下载所有 artifact
   - 创建 GitHub Release
   - 上传所有平台的包

### 触发条件

- Push 到任意分支
- Push 到标签 (`v*`)
- Pull Request

### 产物

- **Artifact**: 临时存储，供 release 使用
- **Release**: 永久存储，用户下载

---

## 故障排查

### 资源文件缺失

```
Error: Could not load resource .../makepad_widgets/resources/fa-solid-900.ttf
```

**原因**: `robius-packaging-commands` 未正确复制资源

**解决**:
1. 检查 `before-packaging-command` 是否正确配置
2. 手动运行 `robius-packaging-commands before-packaging --force-makepad --binary-name meal_manager --path-to-binary ./target/release/meal_manager`
3. 检查 `./dist/resources/` 是否包含所有资源

### 运行时找不到资源

**原因**: `MAKEPAD_PACKAGE_DIR` 环境变量未设置

**解决**: 确保 `main.rs` 中有以下代码：

```rust
if std::env::var("MAKEPAD_PACKAGE_DIR").is_err() {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            unsafe {
                std::env::set_var("MAKEPAD_PACKAGE_DIR", exe_dir);
            }
        }
    }
}
```

### WASM 加载失败

**原因**: 本地测试时 MIME 类型不正确

**解决**: 使用 `cargo makepad wasm run` 或支持 WASM 的 HTTP 服务器

---

## 参考链接

- [Makepad 官方打包指南](https://makepad.rs/guide/appendix/packaging-guide)
- [cargo-packager](https://github.com/crabnebula-dev/cargo-packager)
- [robius-packaging-commands](https://github.com/project-robius/robius-packaging-commands)
- [cargo-makepad](https://github.com/makepad/makepad)
