$go(paths, paths);
$for(paths, path{/Orders}, pathobj)`path`:
# $get(path);

$for(pathobj, method{!parameters}, methodobj)`method`:
## $get(method);

### 描述

$get(methodobj.summary);

$tryget(methodobj.description);

### parameters

|参数名|类型|必填|说明|
|:-----:|:-----:|:-----:|:-----:|
$tryfor(methodobj.parameters, param{in:query})`param`:
|$get(param.name);|$get(param.schema.type);|$tryget(param.required, false);|$tryget(param.description, none);|
$(`param`);

### body

|参数名|类型|必填|说明|
|:-----:|:-----:|:-----:|:-----:|

### responses

$for(methodobj.responses, code, responseobj)`code`:
#### $get(code);

$tryget(responseobj.description);

$go(responseobj.content, contentobj);
$(`code`);
$(`method`);
$(`path`);