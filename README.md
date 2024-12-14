# osa2md

A simple tool to convert OSA(openapi) files to Markdown.

一个将 OSA(openapi) 文件转换为 Markdown 的简单工具。以自定义的脚本语言来实现 Markdown 生成，本工具对脚本语言进行解析，并生成 Markdown 文件。

## 语言格式

\$ 表示命令, 以 \$ 开始 ; 结束，\$后面跟着要执行的函数, 例如： \$get(); 

： 表示代码块开始，以\$; 结束, 例如：$for(paths, path, pathobj);

{}: 表示条件, {} 中的为条件，例如 \$for(paths, path{/Alphabetical_list_of_products}, pathobj), /Alphabetical_list_of_products 为符合条件的路径，pathobj 为后续使用的对象名称;

!: 表示非，配合 {} 使用;

函数：

go(path, name): 进入某个 object, path 为路径，name 为后续这个使用这个object的名称, 例如：go(paths, paths);

get(name): 获取参数, name 为参数名称, 例如：get(paths);

for(source, name, ...)：循环某个 object, source 为对象名称，name 为后续使用这个对象的没一项的名称, 例如：for(paths, path), for 需要 ：跟着后续操作或命令;


## Usage

```markdown
$go(paths, paths);
$for(paths, path, path_map):
# $get(path);

$for(path_map, method{!parameters}, map):
## $get(method);
$;
$;
```

以下面json为数据源：

```json
{
    "paths": {
        "/Orders": {
            "get": {
                "summary": "Get entities from Orders",
                "tags": [
                    "Orders"
                ],
                "parameters": [
                    {
                        "$ref": "#/components/parameters/top"
                    },
                    {
                        "$ref": "#/components/parameters/skip"
                    },
                    {
                        "name": "$filter",
                        "description": "Filter items by property values, see [Filtering](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionfilter)",
                        "in": "query",
                        "schema": {
                            "type": "string"
                        }
                    },
                    {
                        "$ref": "#/components/parameters/count"
                    },
                    {
                        "name": "$orderby",
                        "description": "Order items by property values, see [Sorting](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionorderby)",
                        "in": "query",
                        "explode": false,
                        "schema": {
                            "type": "array",
                            "uniqueItems": true,
                            "items": {
                                "type": "string",
                                "enum": [
                                    "OrderID",
                                    "OrderID desc",
                                    "CustomerID",
                                    "CustomerID desc",
                                    "EmployeeID",
                                    "EmployeeID desc",
                                    "OrderDate",
                                    "OrderDate desc",
                                    "RequiredDate",
                                    "RequiredDate desc",
                                    "ShippedDate",
                                    "ShippedDate desc",
                                    "ShipVia",
                                    "ShipVia desc",
                                    "Freight",
                                    "Freight desc",
                                    "ShipName",
                                    "ShipName desc",
                                    "ShipAddress",
                                    "ShipAddress desc",
                                    "ShipCity",
                                    "ShipCity desc",
                                    "ShipRegion",
                                    "ShipRegion desc",
                                    "ShipPostalCode",
                                    "ShipPostalCode desc",
                                    "ShipCountry",
                                    "ShipCountry desc"
                                ]
                            }
                        }
                    },
                    {
                        "name": "$select",
                        "description": "Select properties to be returned, see [Select](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionselect)",
                        "in": "query",
                        "explode": false,
                        "schema": {
                            "type": "array",
                            "uniqueItems": true,
                            "items": {
                                "type": "string",
                                "enum": [
                                    "OrderID",
                                    "CustomerID",
                                    "EmployeeID",
                                    "OrderDate",
                                    "RequiredDate",
                                    "ShippedDate",
                                    "ShipVia",
                                    "Freight",
                                    "ShipName",
                                    "ShipAddress",
                                    "ShipCity",
                                    "ShipRegion",
                                    "ShipPostalCode",
                                    "ShipCountry"
                                ]
                            }
                        }
                    },
                    {
                        "name": "$expand",
                        "description": "Expand related entities, see [Expand](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionexpand)",
                        "in": "query",
                        "explode": false,
                        "schema": {
                            "type": "array",
                            "uniqueItems": true,
                            "items": {
                                "type": "string",
                                "enum": [
                                    "*",
                                    "Customer",
                                    "Employee",
                                    "Order_Details",
                                    "Shipper"
                                ]
                            }
                        }
                    }
                ],
                "responses": {
                    "200": {
                        "description": "Retrieved entities",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "title": "Collection of Order",
                                    "properties": {
                                        "@odata.count": {
                                            "$ref": "#/components/schemas/count"
                                        },
                                        "value": {
                                            "type": "array",
                                            "items": {
                                                "$ref": "#/components/schemas/NorthwindModel.Order"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "4XX": {
                        "$ref": "#/components/responses/error"
                    }
                }
            }
        }
    }
}
```

将转换成这样的markdown:
```markdown
# /Orders

## get

## post
```