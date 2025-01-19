# osa2md

A simple tool to convert OSA(openapi) files to Markdown.

将 OSA(openapi) 文件转换为 Markdown 的工具。使用定义的语言将 OSA 数据生成 Markdown 文件。

## 语言格式

- `$` 表示命令, 以 `$` 开始 `;` 结束, `$` 后面跟着要执行的函数, 例如: `$get();`, ()后面可以添加备注例如: **$get(paths)\`get paths\`;** 备注中不能包含 `:` 和 `;`。

- `:` 表示代码块开始, `$();` 结束代码块, 例如：`$for(paths, path, pathobj): ... $();`, `$()` ()里可以添加备注。

- `{}` 表示条件, {} 中的为条件，例如 `$for(paths, path{/Alphabetical_list_of_products}, pathobj);`, `/Alphabetical_list_of_products` 为符合条件的路径，`pathobj` 为后续使用的对象名称。

- `!` 表示非, 配合 {} 使用。

- 参数可以使用 `""` 包裹来应对参数存在特殊字符的情况。 

### 内置函数：

下列为内置函数，有些函数是特殊需求特殊处理的，不适用于普通情况。

函数特殊参数标注：
1. 函数名前 `+` 表示参数可选 
2. `{}` 参照语言格式, 
3. `...` 表示多个参数

---

- go(source, var): 进入某个 object; source 为路径, var 为后续这个使用这个object的名称。示例:
```
$go(paths, paths);
```

- get(source, default): 获取参数; name 为参数名称, default 为默认值 找不到对应值时使用的值。示例:
```
$get(methodobj.summary);
```

- for(source, key, +val, +retval...)：循环某个 object/arrays; source 为对象名称, key 为后续使用key值的名称, val 则是val值的名称, retval 为返回的变量名。示例:
```
|参数名|类型|必填|说明|
|-----|----|----|----|
$for(methodobj.parameters, param{in:query})`param`:
|$get(param.name);|$osa3type(param.schema);|$get(param.required, false);|$get(param.description, none);|
$(`param`);
```

- break(): 跳出循环。示例:
```
$for(paths, path, pathobj):
$break();
$();
```

- continue(): 跳过当前循环。示例:
```
$for(paths, path, pathobj):
$continue();
$();
```

- exist(source, var): 判断某个var是否存在于source中, 存在则执行子命令。示例:
```
$exist(already, name)`exist`:
$continue();
$()`exist`;
```

- recurs(name, source, var)：递归, name 为递归的函数名称, source 为对象名称, var 为后续使用的对象名称也就是source改名为var, 来支持递归函数中的source的变动。示例:
```
$recurs(recurs, objarr, arr)`recurs`:

$for(arr, name, objref)`forobj`:
$exist(already, name)`exist`:
$continue();
$()`exist`;
<details>
<summary>$get(name);</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
$for(objref.properties, field, fieldobj, sobjarr)`field`:
|$get(field);|$osa3type(fieldobj, sobjarr);|$get(fieldobj.required, false);|$get(fieldobj.description, none);|
$()`field`;
$push(already, name);
$recurs(recurs, sobjarr, arr);

</details>

$()`forobj`;
$()`recurs`;
```

- debug(name): 输出调试信息, name 为要输出的调试信息。示例:
```
$debug(test);
```

- global(name, type): 全局变量, name 为变量名称, type 为变量类型。示例:
```
$global(already, arr);
```

- push(source, var): 将 var 添加到 source 中。示例:
```
$push(already, name);
```

- drop(name): 删除全局变量。示例:
```
$drop(already);
```

- osa3type(source, +var): 获取参数的类型, source 为参数名称, var 为obj存储的变量名。示例:
```
$osa3type(param.schema);
$osa3type(itemobj, objarr);
```

## Usage

```
osa2md -i openapi3.md -d openapi3.json
```