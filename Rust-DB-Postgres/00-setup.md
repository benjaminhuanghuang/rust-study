# Postgres setup

## Postgres in docker

```sh

docker pull postgres
# list only the containers that are currently running
docker ps
# list all containers, including those that are stopped
docker ps -a



docker run --name postgres-container -e POSTGRES_PASSWORD=<pwd> -p 5432:5432 -d postgres

# My postgres container
docker run --name my_postgres -e POSTGRES_USER=myuser -e POSTGRES_PASSWORD=mypassword -e POSTGRES_DB=file_share -p 5432:5432 -d postgres
```

--name postgres-container: Names the container as postgres-container (you can choose any name you prefer).
-e POSTGRES_PASSWORD=mysecretpassword: Sets the password for the postgres superuser
-p 5432:5432: Maps port 5432 on the host machine to port 5432 in the container, which is the default PostgreSQL port.
-d: Runs the container in detached mode (in the background).
postgres: The name of the image to use (this will use the official PostgreSQL image).

username: myuser
password: mypassword
