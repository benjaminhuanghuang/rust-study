# Chapter 17: MySQL

## Run the MySQL Docker Container

```bash
  # build the image
  docker build -t mysql-image .

  # run the container
  docker image ls
  docker image ls mysql-image
  
  docker container stop mysql-container # if there is a container running
  docker container rm mysql-container 

  docker run --name mysql-container -d -p 3306:3306 mysql-image
  docker container ls  # show the running containTor

```


## Install mysql crate
```hash
  cargo install mysql
```
