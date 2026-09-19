# 更新日志

所有正式版本都发布在 [GitHub Releases](https://github.com/xiongzikun0106/Catify/releases/latest)，可执行文件也从那里下载。

---

## v0.1.0 —— 让每一次回车都变成喵

首个公开版本。功能就一个：**按下回车，自动多打一个「喵」。**

### 下载

| 文件 | 平台 | 说明 |
| --- | --- | --- |
| [`catify.exe`](https://github.com/xiongzikun0106/Catify/releases/latest/download/catify.exe) | Windows 10 / 11 x64 | 31 KB，免安装、免解压、不需要管理员权限 |

- 文件大小：**31,744 字节**（31 KB）
- SHA-256：`FE1DE97B3DF58737AD64A3649859085DDDA04EBD437D1A4FB394EB1F64A7F95E`

想确认下到的就是这份，可以自己核一下哈希：

```powershell
Get-FileHash .\catify.exe -Algorithm SHA256
```

### 怎么用

1. 双击 `catify.exe`
2. 什么都不会发生 —— **这就是成功了**。没有窗口、没有托盘图标、没有任何提示，只有一只看不见的猫
3. 随便打开一个能打字的地方，按回车试试

想确认它真的在跑：任务管理器里会有一个 `catify.exe` 进程。

### 怎么关掉

程序没有退出菜单（它觉得猫不该被关起来），只能杀掉：

```powershell
taskkill /IM catify.exe /F
```

### 这一版做了什么

- 全局低级键盘钩子（`WH_KEYBOARD_LL`）监听回车，用 `SendInput` + `KEYEVENTF_UNICODE` 把「喵」注入输入流
- `LLKHF_INJECTED` 判断：注入的按键会被钩子再次看到，这行保证它不会自己触发自己（否则会喵出满屏）
- `Global\catify-mutex` 全局互斥体：手贱双击两次也不会养出第二只猫
- `#![windows_subsystem = "windows"]`：GUI 子系统，运行时不弹控制台黑框
- release 配置：`opt-level = "z"` + `lto` + `codegen-units = 1` + `panic = "abort"` + `strip`，成品压到 31 KB

### 已知问题 / 注意事项

- **需要 VC++ 运行库**：这个 exe 是动态链接 MSVC 运行时的（依赖 `VCRUNTIME140.dll` 和 UCRT）。绝大多数 Windows 10/11 都有，但**干净系统**上可能提示缺少 DLL。遇到了装一个 [Microsoft Visual C++ 2015-2022 Redistributable (x64)](https://aka.ms/vs/17/release/vc_redist.x64.exe) 即可；或者自己静态编译出一个不依赖任何运行库的版本：

  ```powershell
  $env:RUSTFLAGS = "-C target-feature=+crt-static"
  cargo build --release
  ```

- **全局键盘钩子 + 模拟按键**，这两个特征叠在一起和键盘记录器长得很像，杀毒软件、游戏反作弊可能会报警或拦截。介意就先别装，打完排位再装。
- 在独占全屏程序、UAC 提权窗口、以及安全桌面（Ctrl+Alt+Del）里，钩子收不到按键，喵不出来。
- 只处理 `VK_RETURN`，完全不看修饰键 —— 所以 Shift+回车、Ctrl+回车、小键盘回车**统统都会喵**。
- 只在 Windows 上能跑。Linux / macOS 用户请自觉手动喵。

### 许可证

[MIT](https://github.com/xiongzikun0106/Catify/blob/main/LICENSE)。Copyright (c) 2026 [@CharlesLiu9441](https://github.com/CharlesLiu9441)（原作者）、[@xiongzikun0106](https://github.com/xiongzikun0106)（代为开源）。
