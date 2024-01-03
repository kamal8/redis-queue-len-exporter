# REDIS LLEN EXPORTER FOR PROMETHEUS
This is a simple exporter for prometheus to export the length of a redis list.
## USAGE
```
docker run -d --name rust_service \
  --network my_network \
  -p 8000:8000 \
  -e REDIS_URL=redis://redis:6379 \
  -e RUST_LOG=info \
  rust_service
```

Have a look at examples folder for a docker-compose example.