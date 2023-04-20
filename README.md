# 基于Orange文件搜索的自动操作

## 项目主体结构说明

1. 整体使用Tauri框架，基于Orange文件搜索器的文件搜索功能，实现了一个简单的文件搜索器

```shell
tree -L 1                                                                                                                                        ─╯
.
├── LICENSE
├── Orange_README.md
├── Orange_README_cn.md
├── README.md
├── app-icon.png
├── build
├── doc
├── materials
├── node_modules
├── package.json
├── public
├── screenshot
├── src // 主要前端代码
├── src-tauri // tauri相关代码（rust+python）
└── yarn.lock

8 directories, 7 files
```

2. src-tauri: rust+python代码

```shell
tree -L 2 src-tauri                                                                                                                              ─╯
src-tauri
├── Cargo.lock
├── Cargo.toml
├── build.rs
├── cargo_run.log
├── examples
│   ├── include_absolute_python_from_rust.rs
│   ├── include_relative_python_from_rust.rs
│   ├── python_demo
│   ├── try_xlsxreader.rs
│   ├── try_xlsxwriter.rs
│   └── using_python_from_rust.rs
├── excel_operators // python代码, 处理excel文件
│   ├── READEME.md
│   ├── basic
│   ├── build
│   ├── dist
│   ├── excel_operator.py
│   ├── excel_operator.spec
│   ├── project_venv
│   ├── projects
│   └── requirements.txt
├── icons
├── rustfmt.toml
├── sidecars // 存放python编译后的可执行文件
│   ├── excel_operator
│   ├── excel_operator-x86_64-apple-darwin
│   ├── excel_operator-x86_64-pc-windows-msvc.exe
│   └── excel_operator.exe
├── src // rust代码
│   ├── excel_operator.rs
│   ├── file_doc.rs
│   ├── file_view.rs
│   ├── fs_watcher.rs
│   ├── idx_store.rs
│   ├── indexing.rs
│   ├── kv_store.rs
│   ├── main.rs
│   ├── ui.rs
│   ├── user_setting.rs
│   ├── usn_journal_watcher.rs
│   ├── utils.rs
│   ├── walk_exec.rs
│   ├── walk_metrics.rs
│   └── watch_exec.rs
├── target
│   ├── CACHEDIR.TAG
│   ├── debug
│   ├── release
│   └── simple1.xlsx
├── tauri.conf.json
└── test.xlsx

14 directories, 53 files

```

## Orange文件搜索器

[英文](Orange_README.md)
[中文](Orange_README_cn.md)

## Excel读写

- [tafia/calamine: A pure Rust Excel/OpenDocument SpeadSheets file reader: rust on metal sheets](https://github.com/tafia/calamine)
- [xlsxwriter - Rust](https://docs.rs/xlsxwriter/latest/xlsxwriter/)

## 关于python sidecar更新

> 不同平台的文件需要在对应平台执行pyinstaller编译

1. 激活python环境，执行下列指令

```shell
pyinstaller --onefile excel_operator.py
```

2. 将dist/excel_operator放入sidecar文件夹，需要额外复制一个执行文件：

```shell
cp excel_operator/dist/excel_operator sidecar/excel_operator-x86_64-pc-windows-msvc.exe
```

> tauri主要根据后缀来判断在不同平台执行哪个文件

```shell
excel_operator-x86_64-apple-darwin
```

3. 如果没有结果，说明中间报错了，这块错误提示还需要考虑和rust/js交互