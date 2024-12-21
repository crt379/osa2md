# osa2md

A simple tool to convert OSA(openapi) files to Markdown.

一个将 OSA(openapi) 文件转换为 Markdown 的简单工具。以自定义的脚本语言来实现 Markdown 生成，本工具对脚本语言进行解析，并生成 Markdown 文件。

## 语言格式

- `$` 表示命令, 以 `$` 开始 `;` 结束, `$` 后面跟着要执行的函数, 例如: `$get();`, ()后面可以添加备注例如: **$get(paths)\`get paths\`;** 备注中不能包含 `:` 和 `;`.

- `:` 表示代码块开始, `$();` 结束代码块, 例如：`$for(paths, path, pathobj): ... $();`, `$()` ()里可以添加备注.

- `{}` 表示条件, {} 中的为条件，例如 `$for(paths, path{/Alphabetical_list_of_products}, pathobj);`, `/Alphabetical_list_of_products` 为符合条件的路径，`pathobj` 为后续使用的对象名称.

- `!` 表示非, 配合 {} 使用.

### 内置函数：

- go(path, name): 进入某个 object, path 为路径，name 为后续这个使用这个object的名称, 例如：`$go(paths, paths);`.

- get(name): 获取参数, name 为参数名称, 例如：`$get(paths);`.

- tryget(name): 和get函数一样, 没有对应值时不抛错;

- for(source, name, ...)：循环某个 object, source 为对象名称，name 为后续使用这个对象的没一项的名称, 例如：`$for(paths, path);`, for 需要 `:` 跟着后续操作或命令.


## Usage

```
osa2md -i openapi3.md -d Northwind-V3.openapi3.json
```