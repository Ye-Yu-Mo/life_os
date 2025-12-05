---
title: "GPUI 完整入门教程：从安装到打包你的第一个桌面应用 - 全栈开发"
source: "https://www.lvtao.net/dev/gpui-complete-tutorial-from-install-to-package.html"
author:
  - "[[memory]]"
published: 2025-11-01
created: 2025-12-05
description: "GPUI 是一个由 Figma 工程师团队开源的、用 Rust 编写的跨平台 GUI 框架。它的核心思想是利用 GPU 的强大并行计算能力来渲染界面，从而实现极致的流畅度和性能。官方的 Zed ..."
tags:
  - "clippings"
---
GPUI 是一个由 Figma 工程师团队开源的、用 编写的跨平台 GUI 框架。它的核心思想是利用 GPU 的强大并行计算能力来渲染界面，从而实现极致的流畅度和性能。官方的 [Zed 编辑器源码](https://www.lvtao.net/url.html?t=aHR0cHM6Ly9naXRodWIuY29tL3plZC1pbmR1c3RyaWVzL3plZA==) 是学习 GPUI 最佳实践的宝库

**核心特点：**

- **高性能** ：直接使用 GPU 进行渲染，即使是复杂的动画和大量的 UI 元素也能保持高帧率。
- **Rust 语言** ：享受 Rust 带来的内存安全、高性能和现代化的工具链。
- **声明式 UI** ：类似于 React 或 Vue，你只需要描述 UI 在不同状态下应该是什么样子，GPUI 会负责处理底层的更新和渲染。
- **跨平台** ：支持 macOS、 和 Windows。

#### 二、 环境准备与安装

在开始之前，请确保你的开发环境已经准备就绪。

**1\. 安装 Rust**

GPUI 是基于 Rust 的，所以首先需要安装 Rust。推荐使用 `rustup` 来管理 Rust 版本。

```bash
# 访问 https://rustup.rs/ 并按照指示安装，或者直接运行以下命令
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后，重启终端，并验证：

```bash
rustc --version
cargo --version
```

**2\. 安装 GPUI CLI 工具**

GPUI 提供了一个方便的命令行工具 `cargo-gpui` ，用于创建和管理项目。

```bash
cargo install cargo-gpui
```

安装完成后，验证：

```bash
cargo gpui --version
```

**3\. 系统依赖**

根据你的操作系统，可能需要安装一些额外的系统库。

- : 通常不需要额外安装，Xcode Command Line Tools 已包含所需组件。
- **Linux (Ubuntu/Debian)**: 需要安装 GTK 3 及其开发库。
	```bash
	sudo apt-get update
	sudo apt-get install libgtk-3-dev libglib2.0-dev libpango1.0-dev libcairo2-dev libatk1.0-dev libgdk-pixbuf2.0-dev libx11-dev libxext-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev libxfixes-dev libxcomposite-dev libxdamage-dev libxss-dev libxtst-dev
	```
- : 需要安装 [Microsoft Visual Studio C++ Build Tools](https://www.lvtao.net/url.html?t=aHR0cHM6Ly92aXN1YWxzdHVkaW8ubWljcm9zb2Z0LmNvbS92aXN1YWwtY3BwLWJ1aWxkLXRvb2xzLw==) ，并确保在安装时勾选 "Windows 10/11 SDK" 和 "C++ CMake tools"。

#### 三、 创建第一个 GPUI 应用 ("Hello, World!")

让我们从最简单的 "Hello, World!" 应用开始，感受一下 GPUI 的基本结构。

**1\. 创建项目**

```bash
# 创建一个新的 Rust 二进制项目
cargo new --bin my_gpui_app

# 进入项目目录
cd my_gpui_app
```

**2\. 配置 `Cargo.toml`**

打开项目根目录下的 `Cargo.toml` 文件，添加 GPUI 和日志相关的依赖。

```toml
[package]
name = "my_gpui_app"
version = "0.1.0"
edition = "2021"

[dependencies]
# GPUI 核心库
gpui = { git = "https://github.com/zed-industries/zed", branch = "main" }
# 日志库，用于调试
log = "0.4"
# 简单的日志实现器，将日志输出到控制台
env_logger = "0.10"
```

> **注意**: 目前 GPUI 还没有发布到 crates.io，所以需要直接从其 GitHub 仓库（Zed 编辑器的仓库）引入。 `branch = "main"` 确保你使用的是最新的主分支代码。

**3\. 编写 "Hello, World!" 代码**

现在，用下面的代码替换 `src/main.rs` 的全部内容。代码中包含了详细的中文注解，解释了每一部分的作用。

```rust
// src/main.rs

use gpui::*;
use std::sync::Arc;

// 定义一个全局唯一的应用 ID，用于系统识别应用
static APP_ID: &str = "d8b8e2b1-0c9b-4b7e-8b8a-0c9b4b7e8b8a";

// 应用程序的入口函数
fn main() {
    // 初始化日志系统，这样我们就可以在控制台看到 log::info! 等宏的输出
    env_logger::init();

    // 创建一个新的 GPUI 应用实例
    // App::new() 是应用的构造函数
    App::new().run(|cx: &mut AppContext| {
        // 在应用启动时执行的闭包
        // cx 是一个应用上下文，包含了应用的全局状态和方法

        // 1. 注册我们的视图组件
        // GPUI 需要知道我们的自定义视图类型，以便在内部管理它
        cx.on_app_quit(|_event, _cx| {
            // 当应用退出时执行的回调
            println!("App is quitting!");
        });

        // 2. 打开一个新窗口
        // cx.open_window() 用于创建并显示一个新的窗口
        // WindowOptions::default() 使用默认窗口配置
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    // 设置窗口初始位置和大小
                    origin: Point::new(100.0, 100.0),
                    size: Size::new(800.0, 600.0),
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("我的第一个 GPUI 应用".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(Point::new(12.0, 12.0)),
                }),
                window_background: WindowBackgroundAppearance::Transparent,
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                display_id: None,
                window_min_size: Some(Size::new(400.0, 300.0)),
                window_max_size: None,
                fullscreen: false,
                maximized: false,
                ..WindowOptions::default()
            },
            |cx| {
                // 在窗口打开后，构建并显示视图
                // cx.new_view() 创建一个新的视图实例
                // HelloView::new() 是我们自定义视图的构造函数
                let view = cx.new_view(|cx| HelloView::new(cx));
                
                // 将视图添加到窗口中
                cx.show_view(view)
            },
        );
    });
}

// 定义我们的 "Hello, World!" 视图结构体
// 这个结构体将持有视图的状态（虽然这个例子没有状态）
struct HelloView {
    // 在这里可以添加视图的状态，例如一个计数器
}

// 为 HelloView 实现方法
impl HelloView {
    // 视图的构造函数
    fn new(cx: &mut ViewContext<Self>) -> Self {
        // 当视图被创建时，这个函数会被调用
        // cx 是视图上下文，提供了视图级别的方法
        log::info!("HelloView created!");
        Self {}
    }

    // 渲染方法：这是 GPUI 的核心
    // 它描述了视图在任何时候应该是什么样子
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // gpui::div() 类似于 HTML 中的 <div>，是一个容器元素
        div()
            // .flex() 使其成为一个 Flexbox 容器
            .flex()
            // .items_center() 和 .justify_center() 实现垂直和水平居中
            .items_center()
            .justify_center()
            // .size_full() 使其占满父容器的全部空间
            .size_full()
            // .bg() 设置背景色，rgb() 是一个颜色辅助函数
            .bg(rgb(0x2e2e2e))
            // .child() 添加一个子元素
            .child(
                // gpui::label() 用于显示文本
                label("Hello, GPUI!")
                    // .text_size() 设置字体大小
                    .text_size(32.0)
                    // .color() 设置文字颜色
                    .color(rgb(0xffffff))
            )
    }
}

// 为 HelloView 实现 Render trait，这是 GPUI 渲染系统要求的
impl Render for HelloView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.render(cx)
    }
}
```

**4\. 运行应用**

在终端中运行以下命令：

```bash
cargo run
```

如果一切顺利，你将会看到一个深灰色背景的窗口，中间显示着白色的 "Hello, GPUI!" 文字。

#### 四、 增加交互：一个简单的计数器

现在，让我们在 "Hello, World!" 的基础上增加一个按钮和计数器，学习 GPUI 的状态管理和事件处理。

修改 `src/main.rs` 文件如下：

```rust
// src/main.rs

use gpui::*;
use std::sync::Arc;

static APP_ID: &str = "d8b8e2b1-0c9b-4b7e-8b8a-0c9b4b7e8b8a";

fn main() {
    env_logger::init();
    App::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point::new(100.0, 100.0),
                    size: Size::new(400.0, 200.0),
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("GPUI 计数器".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(Point::new(12.0, 12.0)),
                }),
                window_background: WindowBackgroundAppearance::Transparent,
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                display_id: None,
                window_min_size: None,
                window_max_size: None,
                fullscreen: false,
                maximized: false,
                ..WindowOptions::default()
            },
            |cx| {
                let view = cx.new_view(|cx| CounterView::new(cx));
                cx.show_view(view)
            },
        );
    });
}

// 将 HelloView 重命名为 CounterView
struct CounterView {
    // 为视图添加一个状态：计数值
    count: i32,
}

impl CounterView {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        // 初始化时，计数值为 0
        Self { count: 0 }
    }

    // 定义一个增加计数的方法
    fn increment(&mut self, cx: &mut ViewContext<Self>) {
        self.count += 1;
        // **关键步骤**：通知 GPUI 状态已改变，需要重新渲染视图
        // cx.notify() 会触发 render 方法被再次调用
        cx.notify();
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col() // 改为垂直方向排列
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x2e2e2e))
            .gap_4() // 添加元素间距
            .child(
                // 显示当前计数值
                label(format!("Count: {}", self.count))
                    .text_size(24.0)
                    .color(rgb(0xffffff))
            )
            .child(
                // 创建一个按钮
                Button::new("increment", "Click me!")
                    // 设置按钮的样式
                    .style(ButtonStyle::Filled)
                    .size(ButtonSize::Compact)
                    // 为按钮添加点击事件
                    .on_click(cx.listener(|_view, _event, cx| {
                        // 当按钮被点击时，调用 increment 方法
                        // 注意：这里的 listener 是一个闭包，它会捕获视图的引用
                        // 我们需要通过 \`view\` 来调用视图的方法
                        // 更简单的写法是直接在闭包里操作状态
                        // 但为了演示，我们调用方法
                        // _view.increment(cx);
                        // 在 on_click 闭包中，第一个参数是视图的 &mut Self
                        // 所以可以直接修改
                        _view.increment(cx);
                    }))
            )
    }
}

impl Render for CounterView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.render(cx)
    }
}
```

再次运行 `cargo run` ，现在你将看到一个包含计数和按钮的窗口。每次点击按钮，计数值都会增加，并且界面会自动更新。

**核心概念解释：**

- **状态**: `struct CounterView` 中的 `count: i32` 就是视图的内部状态。
- **事件处理**: `.on_click(...)` 为按钮绑定了点击事件。当事件发生时，提供的闭包会被执行。
- **状态更新与重渲染**: 在 `increment` 方法中， `self.count += 1` 修改了状态。紧接着的 `cx.notify()` 是告诉 GPUI：“这个视图的状态变了，请重新调用 `render` 方法来更新界面”。这是 GPUI 响应式编程的核心。

#### 五、 打包成可执行产品

当你开发完成后，需要将应用打包成可分发的格式。

**1\. 构建优化版本**

首先，使用 `--release` 标志进行编译，这会启用所有优化，使应用运行得更快，体积更小。

```bash
cargo build --release
```

编译完成后，可执行文件位于：

- **macOS/Linux**: `target/release/my_gpui_app`
- **Windows**: `target/release/my_gpui_app.exe`

直接运行这个文件是可以的，但它缺少了图标、依赖库等信息，不方便分发。

**2\. 使用 `cargo-gpui` 打包**

`cargo-gpui` 工具提供了打包命令，可以自动处理很多繁琐的细节。

**macOS (打包成.app)**

```bash
cargo gpui macos-bundle --release
```

执行后，你会在 `target/release/bundle/macos` 目录下找到一个 `my_gpui_app.app` 文件。这是一个标准的 macOS 应用包，你可以直接双击运行，或者分发给其他用户。

**Linux (打包成 AppImage)**

对于 Linux，AppImage 是一个很好的分发方式，因为它包含了所有依赖，可以在大多数 Linux 发行版上直接运行。

首先，安装 `cargo-appimage` 工具：

```bash
cargo install cargo-appimage
```

然后，运行打包命令：

```bash
cargo appimage --release
```

打包完成后，你会在 `target/appimage` 目录下找到一个 `.AppImage` 文件。给它添加可执行权限后即可运行：

```bash
chmod +x target/appimage/my_gpui_app-0.1.0-x86_64.AppImage
./target/appimage/my_gpui_app-0.1.0-x86_64.AppImage
```

**Windows (打包成安装包)**

Windows 的打包相对复杂一些，通常需要创建一个 MSI 安装包。 `cargo-gpui` 目前可能没有直接的 MSI 打包命令，但你可以使用其他工具如 `wix` 或 `cargo-wix` 来创建安装包。

一个简单的方法是：

1. 找到 `target/release/my_gpui_app.exe` 。
2. 确保目标机器上安装了 [Microsoft Visual C++ Redistributable](https://www.lvtao.net/url.html?t=aHR0cHM6Ly9sZWFybi5taWNyb3NvZnQuY29tL2VuLXVzL2NwcC93aW5kb3dzL2xhdGVzdC1zdXBwb3J0ZWQtdmMtcmVkaXN0) （与你的 Rust 工具链版本匹配）。
3. 将 `.exe` 文件和必要的资源文件（如果有的话）打包成一个 `.zip` 文件进行分发，并提醒用户安装 VC++ Redistributable。

**打包方法总结**

| 操作系统 | 打包命令 | 输出产物 | 备注 |
| --- | --- | --- | --- |
| **macOS** | `cargo gpui macos-bundle --release` | `.app` 包 | 标准应用格式，可直接分发 |
| **Linux** | `cargo appimage --release` | `.AppImage` 文件 | 自包含，跨发行版运行 |
| **Windows** | (暂无一键命令) | `.exe` 文件 | 需手动处理依赖和安装包制作 |

> 版权声明：本文为原创文章，版权归 [全栈开发技术博客](https://www.lvtao.net/) 所有。
> 
> 本文链接： [https://www.lvtao.net/dev/gpui-complete-tutorial-from-install-to-package.html](https://www.lvtao.net/dev/gpui-complete-tutorial-from-install-to-package.html)
> 
> 转载时须注明出处及本声明

- 上一篇: [PHPMailer完全指南：从基础发件、附件发送等高级配置技巧](https://www.lvtao.net/dev/phpmailer-complete-guide-from-basic-to-advanced.html "PHPMailer完全指南：从基础发件、附件发送等高级配置技巧")
- 下一篇: [ThinkPHP模型关联终极指南：一文搞定hasOne、hasMany与belongsTo](https://www.lvtao.net/dev/thinkphp-model-relationships-guide.html "ThinkPHP模型关联终极指南：一文搞定hasOne、hasMany与belongsTo")

[![JSON格式化](https://tool.lvtao.net/static/index/json.svg)](https://tool.lvtao.net/json)

[JSON格式化](https://tool.lvtao.net/json)

[

JSON Formatter

](https://tool.lvtao.net/json)

[![图片Base64编码](https://tool.lvtao.net/static/index/img2base.svg)](https://tool.lvtao.net/image2base64)

[图片Base64编码](https://tool.lvtao.net/image2base64)

[

Image to Base64

](https://tool.lvtao.net/image2base64)[WebP转PNG/JPG](https://tool.lvtao.net/webp2png)

[

WebP to PNG/JPG

](https://tool.lvtao.net/webp2png)

[![安卓报毒禁止安装应用](https://tool.lvtao.net/static/index/android.svg)](https://tool.lvtao.net/android)

[安卓报毒禁止安装应用](https://tool.lvtao.net/android)

[

Android App Blocker

](https://tool.lvtao.net/android)

[![](https://www.lvtao.net/wp-content/uploads/affiliate/cc2.gif)](https://www.lvtao.net/url.html?t=aHR0cHM6Ly9hcHAuY2xvdWRjb25lLmNvbS8/cmVmPTk1MTk=)

### 相关文章

在开始之前，请确保您已准备好以下内容：macOS操作系统Docker Desktop for Mac（已安装并运行）人大金仓数据库Docker镜像文件（KingbaseES\_V009R001C0...

Surge 作为 macOS 和 iOS等跨平台上功能强大的网络调试工具，其灵活的配置方式让用户能够精细控制设备网络行为。本教程将详细介绍 Surge 的配置方法，从基础协议到高级功能。配置文件...

今天上线一个小业务，因为代码要做授权保护，于是用了ioncube加密了几个核心文件，在本地开发环境（macOS）中，使用 ionCube Encoder v12 加密的 PHP 文件运行一切正常...

deepseek-ocr.rs是一个用Rust语言实现的DeepSeek-OCR推理栈，它提供了快速的命令行界面(CLI)和OpenAI兼容的HTTP服务器。这个项目将模型加载、视觉输入预处理、...

使用这篇文章教程之前，我推荐大家先看下这篇 无需Python环境！在Windows和Mac上直接运行DeepSeek-OCR视觉模型的完整指南 ， 因为不需要安装环境，下载个安装包直接就能跑起来...

Maddy Mail Server 是一个功能强大、一体化且配置相对简单的邮件服务器。它集成了 SMTP (发送/接收)、IMAP (接收) 等所有必要的组件，非常适合个人或小型组织搭建自己的邮...

基础篇：Docker 安装与单机配置1. 使用 Docker 启动 MinIO首先，拉取最新的 MinIO 镜像并运行：docker run -d \\ --name minio \\ -p...

在开发内网服务或测试 HTTPS 接口时，我们经常需要自签发 CA 及域名证书，之前有介绍一个工具 macOS下使用mkcert创建本地HTTPS证书完全指南，如果你不想安装工具的话，就可以使用...