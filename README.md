# 基于Orange文件搜索的自动操作

## Orange文件搜索器

[英文](Orange_README.md)
[中文](Orange_README_cn.md)

## Excel读写

- [tafia/calamine: A pure Rust Excel/OpenDocument SpeadSheets file reader: rust on metal sheets](https://github.com/tafia/calamine)
- [xlsxwriter - Rust](https://docs.rs/xlsxwriter/latest/xlsxwriter/)

## 关于python sidecar更新

1. 激活python环境，执行下列指令

```shell
pyinstaller --onefile excel_operator.py
```

2. 将dist/excel_operator放入sidecar文件夹，需要额外复制一个执行文件：

```shell
excel_operator-x86_64-apple-darwin
```

3. 如果没有结果，说明中间报错了，这块错误提示还需要考虑和rust/js交互