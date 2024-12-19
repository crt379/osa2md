$go(paths, paths);
$for(paths, path{/Orders}, m_o):
# $get(path);

$for(m_o, method{!parameters}, o):
## $get(method);

### 描述

$get(o.summary);

$tryget(o.description);
$;
$;