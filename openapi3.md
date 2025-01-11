$for(paths, path{"/Products({ProductID})/Category", "/Territories('{TerritoryID}')/Employees"}, pathobj)`path`:
# $get(path);

$for(pathobj, method{!parameters}, methodobj)`method`:
## $get(method);

### 描述

$get(methodobj.summary);

$get(methodobj.description);

### parameters

|参数名|类型|必填|说明|
|-----|----|----|----|
$for(methodobj.parameters, param{in:query})`param`:
|$get(param.name);|$osa3type(param.schema);|$get(param.required, false);|$get(param.description, none);|
$(`param`);

### body

|参数名|类型|必填|说明|
|----|----|----|----|
$for(methodobj.requestBody.content, mediaType{application/json}, mediaTypeobj, objarr)`mediaType`:
$for(mediaTypeobj.schema.properties, item, itemobj, objarr)`item`:
|$get(item);|$osa3type(itemobj, objarr);|$get(itemobj.required, false);|$get(itemobj.description, none);|
$()`item`;
$()`mediaType`;

### responses

$for(methodobj.responses, code{!4XX}, responseobj)`code`:
#### $get(code);

$get(responseobj.description);

|参数名|类型|必填|说明|
|-----|-----|-----|-----|
$for(responseobj.content, mediaType{application/json}, mediaTypeobj, objarr)`mediaType`:
$for(mediaTypeobj.schema.properties, item, itemobj, objarr)`item`:
|$get(item);|$osa3type(itemobj, objarr);|$get(itemobj.required, false);|$get(itemobj.description, none);|
$()`item`;
$()`mediaType`;

$global(already, arrays);
$recurs(recurs, objarr, arr)`ddd`:

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
$()`ddd`;
$drop(already);

$()`code`;
$()`method`;
$()`path`;