# /Products({ProductID})/Category

## get

### 描述

Get related Category


### parameters

|参数名|类型|必填|说明|
|-----|----|----|----|
|$select|[enum[CategoryID,CategoryName,Description,Picture]]|false|Select properties to be returned, see [Select](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionselect)|
|$expand|[enum[*,Products]]|false|Expand related entities, see [Expand](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionexpand)|

### body

|参数名|类型|必填|说明|
|----|----|----|----|

### responses

#### 200

Retrieved entity

|参数名|类型|必填|说明|
|-----|-----|-----|-----|
|CategoryID|integer|false|none|
|CategoryName|string|false|none|
|Description|string|false|none|
|Picture|string|false|none|
|Products|[Product]|false|none|
|Products@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|


<details>
<summary>Product</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Category|all[Category]|false|none|
|CategoryID|integer|false|none|
|Discontinued|boolean|false|none|
|Order_Details|[Order_Detail]|false|none|
|Order_Details@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|ProductID|integer|false|none|
|ProductName|string|false|none|
|QuantityPerUnit|string|false|none|
|ReorderLevel|integer|false|none|
|Supplier|all[Supplier]|false|none|
|SupplierID|integer|false|none|
|UnitPrice|or[number,string]|false|none|
|UnitsInStock|integer|false|none|
|UnitsOnOrder|integer|false|none|

<details>
<summary>Category</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|CategoryID|integer|false|none|
|CategoryName|string|false|none|
|Description|string|false|none|
|Picture|string|false|none|
|Products|[Product]|false|none|
|Products@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|


</details>

<details>
<summary>Order_Detail</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Discount|number|false|none|
|Order|object|false|none|
|OrderID|integer|false|none|
|Product|object|false|none|
|ProductID|integer|false|none|
|Quantity|integer|false|none|
|UnitPrice|or[number,string]|false|none|

</details>

<details>
<summary>Supplier</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Address|string|false|none|
|City|string|false|none|
|CompanyName|string|false|none|
|ContactName|string|false|none|
|ContactTitle|string|false|none|
|Country|string|false|none|
|Fax|string|false|none|
|HomePage|string|false|none|
|Phone|string|false|none|
|PostalCode|string|false|none|
|Products|[Product]|false|none|
|Products@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Region|string|false|none|
|SupplierID|integer|false|none|


</details>


</details>


# /Territories('{TerritoryID}')/Employees

## get

### 描述

Get entities from related Employees


### parameters

|参数名|类型|必填|说明|
|-----|----|----|----|
|$top|integer|false|Show only the first n items, see [Paging - Top](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptiontop)|
|$skip|integer|false|Skip the first n items, see [Paging - Skip](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionskip)|
|$filter|string|false|Filter items by property values, see [Filtering](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionfilter)|
|$count|boolean|false|Include count of items, see [Count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount)|
|$orderby|[enum[EmployeeID,EmployeeID desc,LastName,LastName desc,FirstName,FirstName desc,Title,Title desc,TitleOfCourtesy,TitleOfCourtesy desc,BirthDate,BirthDate desc,HireDate,HireDate desc,Address,Address desc,City,City desc,Region,Region desc,PostalCode,PostalCode desc,Country,Country desc,HomePhone,HomePhone desc,Extension,Extension desc,Photo,Photo desc,Notes,Notes desc,ReportsTo,ReportsTo desc,PhotoPath,PhotoPath desc]]|false|Order items by property values, see [Sorting](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionorderby)|
|$select|[enum[EmployeeID,LastName,FirstName,Title,TitleOfCourtesy,BirthDate,HireDate,Address,City,Region,PostalCode,Country,HomePhone,Extension,Photo,Notes,ReportsTo,PhotoPath]]|false|Select properties to be returned, see [Select](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionselect)|
|$expand|[enum[*,Employees1,Employee1,Orders,Territories]]|false|Expand related entities, see [Expand](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptionexpand)|

### body

|参数名|类型|必填|说明|
|----|----|----|----|

### responses

#### 200

Retrieved entities

|参数名|类型|必填|说明|
|-----|-----|-----|-----|
|@odata.count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|value|[Employee]|false|none|


<details>
<summary>Employee</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Address|string|false|none|
|BirthDate|string|false|none|
|City|string|false|none|
|Country|string|false|none|
|Employee1|all[Employee]|false|none|
|EmployeeID|integer|false|none|
|Employees1|[Employee]|false|none|
|Employees1@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Extension|string|false|none|
|FirstName|string|false|none|
|HireDate|string|false|none|
|HomePhone|string|false|none|
|LastName|string|false|none|
|Notes|string|false|none|
|Orders|[Order]|false|none|
|Orders@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Photo|string|false|none|
|PhotoPath|string|false|none|
|PostalCode|string|false|none|
|Region|string|false|none|
|ReportsTo|integer|false|none|
|Territories|[Territory]|false|none|
|Territories@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Title|string|false|none|
|TitleOfCourtesy|string|false|none|

<details>
<summary>Order</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Customer|all[Customer]|false|none|
|CustomerID|string|false|none|
|Employee|all[Employee]|false|none|
|EmployeeID|integer|false|none|
|Freight|or[number,string]|false|none|
|OrderDate|string|false|none|
|OrderID|integer|false|none|
|Order_Details|[Order_Detail]|false|none|
|Order_Details@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|RequiredDate|string|false|none|
|ShipAddress|string|false|none|
|ShipCity|string|false|none|
|ShipCountry|string|false|none|
|ShipName|string|false|none|
|ShipPostalCode|string|false|none|
|ShipRegion|string|false|none|
|ShipVia|integer|false|none|
|ShippedDate|string|false|none|
|Shipper|all[Shipper]|false|none|

<details>
<summary>Customer</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Address|string|false|none|
|City|string|false|none|
|CompanyName|string|false|none|
|ContactName|string|false|none|
|ContactTitle|string|false|none|
|Country|string|false|none|
|CustomerDemographics|[CustomerDemographic]|false|none|
|CustomerDemographics@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|CustomerID|string|false|none|
|Fax|string|false|none|
|Orders|[Order]|false|none|
|Orders@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Phone|string|false|none|
|PostalCode|string|false|none|
|Region|string|false|none|

<details>
<summary>CustomerDemographic</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|CustomerDesc|string|false|none|
|CustomerTypeID|string|false|none|
|Customers|[Customer]|false|none|
|Customers@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|


</details>


</details>

<details>
<summary>Order_Detail</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Discount|number|false|none|
|Order|object|false|none|
|OrderID|integer|false|none|
|Product|object|false|none|
|ProductID|integer|false|none|
|Quantity|integer|false|none|
|UnitPrice|or[number,string]|false|none|

</details>

<details>
<summary>Shipper</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|CompanyName|string|false|none|
|Orders|[Order]|false|none|
|Orders@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Phone|string|false|none|
|ShipperID|integer|false|none|


</details>


</details>

<details>
<summary>Territory</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Employees|[Employee]|false|none|
|Employees@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Region|object|false|none|
|RegionID|integer|false|none|
|TerritoryDescription|string|false|none|
|TerritoryID|string|false|none|


</details>


</details>


## post

### 描述

Add new entity to related Employees


### parameters

|参数名|类型|必填|说明|
|-----|----|----|----|

### body

|参数名|类型|必填|说明|
|----|----|----|----|
|Address|string|false|none|
|BirthDate|string|false|none|
|City|string|false|none|
|Country|string|false|none|
|EmployeeID|integer|false|none|
|Extension|string|false|none|
|FirstName|string|false|none|
|HireDate|string|false|none|
|HomePhone|string|false|none|
|LastName|string|false|none|
|Notes|string|false|none|
|Photo|string|false|none|
|PhotoPath|string|false|none|
|PostalCode|string|false|none|
|Region|string|false|none|
|ReportsTo|integer|false|none|
|Title|string|false|none|
|TitleOfCourtesy|string|false|none|

### responses

#### 201

Created entity

|参数名|类型|必填|说明|
|-----|-----|-----|-----|
|Address|string|false|none|
|BirthDate|string|false|none|
|City|string|false|none|
|Country|string|false|none|
|Employee1|all[Employee]|false|none|
|EmployeeID|integer|false|none|
|Employees1|[Employee]|false|none|
|Employees1@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Extension|string|false|none|
|FirstName|string|false|none|
|HireDate|string|false|none|
|HomePhone|string|false|none|
|LastName|string|false|none|
|Notes|string|false|none|
|Orders|[Order]|false|none|
|Orders@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Photo|string|false|none|
|PhotoPath|string|false|none|
|PostalCode|string|false|none|
|Region|string|false|none|
|ReportsTo|integer|false|none|
|Territories|[Territory]|false|none|
|Territories@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Title|string|false|none|
|TitleOfCourtesy|string|false|none|


<details>
<summary>Employee</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Address|string|false|none|
|BirthDate|string|false|none|
|City|string|false|none|
|Country|string|false|none|
|Employee1|all[Employee]|false|none|
|EmployeeID|integer|false|none|
|Employees1|[Employee]|false|none|
|Employees1@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Extension|string|false|none|
|FirstName|string|false|none|
|HireDate|string|false|none|
|HomePhone|string|false|none|
|LastName|string|false|none|
|Notes|string|false|none|
|Orders|[Order]|false|none|
|Orders@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Photo|string|false|none|
|PhotoPath|string|false|none|
|PostalCode|string|false|none|
|Region|string|false|none|
|ReportsTo|integer|false|none|
|Territories|[Territory]|false|none|
|Territories@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Title|string|false|none|
|TitleOfCourtesy|string|false|none|

<details>
<summary>Order</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Customer|all[Customer]|false|none|
|CustomerID|string|false|none|
|Employee|all[Employee]|false|none|
|EmployeeID|integer|false|none|
|Freight|or[number,string]|false|none|
|OrderDate|string|false|none|
|OrderID|integer|false|none|
|Order_Details|[Order_Detail]|false|none|
|Order_Details@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|RequiredDate|string|false|none|
|ShipAddress|string|false|none|
|ShipCity|string|false|none|
|ShipCountry|string|false|none|
|ShipName|string|false|none|
|ShipPostalCode|string|false|none|
|ShipRegion|string|false|none|
|ShipVia|integer|false|none|
|ShippedDate|string|false|none|
|Shipper|all[Shipper]|false|none|

<details>
<summary>Customer</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Address|string|false|none|
|City|string|false|none|
|CompanyName|string|false|none|
|ContactName|string|false|none|
|ContactTitle|string|false|none|
|Country|string|false|none|
|CustomerDemographics|[CustomerDemographic]|false|none|
|CustomerDemographics@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|CustomerID|string|false|none|
|Fax|string|false|none|
|Orders|[Order]|false|none|
|Orders@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Phone|string|false|none|
|PostalCode|string|false|none|
|Region|string|false|none|

<details>
<summary>CustomerDemographic</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|CustomerDesc|string|false|none|
|CustomerTypeID|string|false|none|
|Customers|[Customer]|false|none|
|Customers@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|


</details>


</details>

<details>
<summary>Order_Detail</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Discount|number|false|none|
|Order|object|false|none|
|OrderID|integer|false|none|
|Product|object|false|none|
|ProductID|integer|false|none|
|Quantity|integer|false|none|
|UnitPrice|or[number,string]|false|none|

</details>

<details>
<summary>Shipper</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|CompanyName|string|false|none|
|Orders|[Order]|false|none|
|Orders@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Phone|string|false|none|
|ShipperID|integer|false|none|


</details>


</details>

<details>
<summary>Territory</summary>

|参数名|类型|必填|说明|
|----|----|----|----|
|Employees|[Employee]|false|none|
|Employees@count|or[integer,string]|false|The number of entities in the collection. Available when using the [$count](http://docs.oasis-open.org/odata/odata/v4.01/odata-v4.01-part1-protocol.html#sec_SystemQueryOptioncount) query option.|
|Region|object|false|none|
|RegionID|integer|false|none|
|TerritoryDescription|string|false|none|
|TerritoryID|string|false|none|


</details>


</details>


