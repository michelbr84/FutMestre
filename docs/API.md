# CMRust API Reference

## Base URL

```
http://localhost:3000/api
```

## Endpoints

### Health

```http
GET /api/health
```

Response:
```json
{"status": "ok", "version": "0.1.0"}
```

### Clubs

```http
GET /api/clubs/:id
```

Response:
```json
{
  "id": "LIV",
  "name": "Liverpool",
  "reputation": 92
}
```

### Players

```http
GET /api/players/:id
```

Response:
```json
{
  "id": "P0001",
  "name": "Player Name",
  "position": "MC",
  "club_id": "LIV"
}
```

### Game State

```http
GET /api/state
```

Response:
```json
{
  "date": "2001-08-15",
  "manager": "Manager",
  "club_id": "LIV"
}
```

## Error Responses

```json
{
  "error": "Not found",
  "code": 404,
  "message": "Club with ID 'XYZ' not found"
}
```

## Running the Server

```bash
cargo run -p cm_server
```

Server starts on `http://127.0.0.1:3000`
