# Credit Card Transactions Websocket API
Mocked credit card transactions Websocket API for experimental/educational purposes.

## Overview
This project is a simple credit card transactions Websocket API. It is a mock of a real API that would be used to get credit card transactions,
useful for experimental/educational purposes, such as testing and developing ML models for fraud detection.

### Channels
The websocketAPI has two channels:
- `heartbeat`: for checking if the connection is alive
- `transactions`: for getting credit card transactions in realtime

## Local Setup

### Running the Server

Run `make run` to start the API locally.
The API will be available at `ws://0.0.0.0:9999/ws/v1`.

Alternatively, use cargo directly:
```bash
cargo run
```

### Configuration

#### Log Level
Control the logging verbosity using the `LOG_LEVEL` environment variable:

```bash
LOG_LEVEL=DEBUG cargo run

```

#### Graceful Shutdown
Press `Ctrl+C` to gracefully shutdown the server. The server will:
- Stop accepting new connections
- Complete existing requests
- Cleanly shutdown background tasks
- Exit cleanly

### Running with Docker

The project includes a Dockerfile using secure Chainguard base images.

#### Build the Docker image:
```bash
docker build -t txapi:latest .
```

#### Run the container:
```bash
# Run with default settings (INFO log level)
docker run -p 9999:9999 txapi:latest

# Run with DEBUG log level
docker run -p 9999:9999 -e LOG_LEVEL=DEBUG txapi:latest

# Run in detached mode
docker run -d -p 9999:9999 --name txapi txapi:latest
```

The WebSocket API will be available at `ws://localhost:9999/ws/v1`

#### Stop the container:
```bash
docker stop txapi
docker rm txapi
```

#### Health Check:
The application includes a health check endpoint at `/health` that returns:
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

You can also run a health check from the command line:
```bash
# Within Docker container
docker exec txapi /app/txapi --health

# Locally
cargo run -- --health

# Using curl
curl http://localhost:9999/health
```

## API Reference


## Subscribing to Channels

To subscribe to a channel, you need to send a message to the server with the following format:

```json
{
  "method": "subscribe",
  "params": {
    "channel": "transactions"
  }
}
```
### Transactions Response

```json
{
  "channel": "transactions",
  "data": [
    {
      "id": "11df919988c134d97bbff2678eb68e22",
      "timestamp": "2024-01-01T00:00:00Z",
      "cc_number": "4473593503484549",
      "category": "Grocery",
      "amount_usd_cents": 10000,
      "latitude": 37.774929,
      "longitude": -122.419418,
      "country_iso": "US",
      "city": "San Francisco",
    }
  ]
}
```

### Heartbeat

```json
{
  "channel": "heartbeat",
  "data": {
    "status": "ok"
  }
}
```
