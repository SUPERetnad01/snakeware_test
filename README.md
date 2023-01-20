A rest api that checks if a number is in the fibonacci sequence and if it is prime number

# run with docker

docker build -t fibonacci-or-prime .
docker run -p 8080:8080 --name fibonacci-or-prime fibonacci-or-prime
