# Clean Rust

A cleanly designed rust REST Api


### prerequisites

- Cargo
- Docker
- Docker-compose
- Diesel_CLI


### Getting started


##### 1 - Run required docker containers

``` bash
docker-compose up
```


##### 2 - Run database migrations

``` bash
diesel migration run --database-url postgresql://postgres:example@localhost:5432/postgres
```


##### 3 - Run the application


### Testing

``` bash
# Get all blogs
curl http://localhost:8000/blog
```

``` bash
# Create blog
curl -d '{"id": 1,"title":"my_title", "body": "my_body", "published": true}' http://localhost:8000/blog
```