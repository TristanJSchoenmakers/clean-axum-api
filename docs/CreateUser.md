## Create User

`POST` **/transaction**

Creates a user which is super awesome


#### Request

```http
POST {{$dotenv BASE_URL}}/transaction HTTP/1.1
content-type: application/json

{
    "lastname": "sample"
}
```

``` sh
cd "something"
```


#### Response OK

```http
HTTP/1.1 200 OK
content-type: application/json
content-length: 14
date: Wed, 07 Dec 2022 16:50:40 GMT

{
  "user_id": 73
}
```