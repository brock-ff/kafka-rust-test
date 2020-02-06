echo "make sure you've changed KAFKA_ADVERTISED_HOST_NAME in `docker-compose-single-broker.yml` to match your own local IP (not localhost)."

docker-compose -f docker-compose-single-broker.yml up
