# MongoDB Docker

```sh

docker pull postgres


docker run --name postgres-container -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres

```

--name postgres-container: Names the container as postgres-container (you can choose any name you prefer).
-e POSTGRES_PASSWORD=mysecretpassword: Sets the password for the postgres superuser (replace mysecretpassword with your desired password).
-p 5432:5432: Maps port 5432 on the host machine to port 5432 in the container, which is the default PostgreSQL port.
-d: Runs the container in detached mode (in the background).
postgres: The name of the image to use (this will use the official PostgreSQL image).
