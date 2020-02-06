HOST=$(ipconfig getifaddr en0)
echo "Running kafka on $HOST:9092"
sleep 1.5

LOCALHOST=$HOST docker-compose -f docker-compose-single-broker.yml up
