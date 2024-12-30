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
|-----|----|----|----|
$tryfor(methodobj.parameters, param{in:query})`param`:
|$get(param.name);|$osa3type(param.schema);|$tryget(param.required, false);|$tryget(param.description, none);|
$(`param`);

### body

|参数名|类型|必填|说明|
|----|----|----|----|

### responses

$for(methodobj.responses, code{!4XX}, responseobj)`code`:
#### $get(code);

$tryget(responseobj.description);

$go(responseobj.content, contentobj);
|参数名|类型|必填|说明|
|-----|-----|-----|-----|
$for(contentobj, mediaType{application/json}, mediaTypeobj)`mediaType`:
$for(mediaTypeobj.schema.properties, item, itemobj)`item`:
|$get(item);|$osa3type(itemobj);|$tryget(itemobj.required, false);|$tryget(itemobj.description, none);|
$()`item`;
$()`mediaType`;

$()`code`;
$()`method`;
$()`path`;