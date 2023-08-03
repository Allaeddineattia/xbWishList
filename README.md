# XbWishlist

This project is made from and for the community members of xbox gamers to make their custom wishlists. This project tries to provide you with the prices of your favorite games from different markets.

## License
This Project is Licenced under gpl-3.0. So you are very welcome to contribute and offer better features. Please see the LICENCE file.

## How to use
Using the docker file you can build the program and run it. 

Build the docker image: 
- docker build -t xb_wishlist -f ./dockerfile .

In order to run the app you need a mongodb service running.

- docker run -d --name my_mongodb --network my_network -p 27017:27017 mongo

Then run a container from the built image on the same network of the mongodb.

- docker run -d --name xb_wishlist --network my_network -p 8080:8080 xb_wishlist

visit the swagger documentation

http://0.0.0.0:8080/swagger-ui/