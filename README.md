# Quidditch Players Rust 2026

Quidditch Players for Hogwarts Houses Rust 2026

Build Tools & Versions Used
----
RustRover<br />
Rust Version: 1.95.0<br />

Branches
----
- [master](https://github.com/CJMobileApps/quidditch-players-rust-2026/tree/master) - branch uses REST APIs

Dependencies
---

**Libraries**
- [axum](https://github.com/tokio-rs/axum) - Ergonomic and modular web framework built with Tokio, Tower, and Hyper.
- [tokio](https://tokio.rs) - An asynchronous runtime for the Rust programming language.
- [serde](https://serde.rs) - A framework for serializing and deserializing Rust data structures efficiently and generically.
- [uuid](https://github.com/uuid-rs/uuid) - Generate and parse UUIDs.
- [headers](https://docs.rs/headers) - Typed HTTP header types for use with hyper and related crates.
- [rand](https://github.com/rust-random/rand) - A Rust library for random number generation.

REST APIs
---

Base URL: `http://localhost:8080/api/v1/quidditchplayers`

All responses are wrapped in a `ResponseWrapper`:

```json
{
    "data": [],
    "status_code": 200
}
```

---

### GET http://localhost:8080/api/v1/quidditchplayers/house/

Returns all Hogwarts houses.

```json
{
    "data": [
        {
            "name": "GRYFFINDOR",
            "imageUrl": "https://static.wikia.nocookie.net/harrypotter/images/9/98/Gryffindor.jpg/revision/latest",
            "emoji": "🦁"
        },
        {
            "name": "SLYTHERIN",
            "imageUrl": "https://static.wikia.nocookie.net/harrypotter/images/6/6e/Slytherin.jpg/revision/latest",
            "emoji": "🐍"
        },
        {
            "name": "RAVENCLAW",
            "imageUrl": "https://static.wikia.nocookie.net/harrypotter/images/3/3c/RavenclawCrest.jpg/revision/latest",
            "emoji": "🦅"
        },
        {
            "name": "HUFFLEPUFF",
            "imageUrl": "https://static.wikia.nocookie.net/harrypotter/images/e/e4/Hufflepuff.jpg/revision/latest",
            "emoji": "🤡"
        }
    ],
    "status_code": 200
}
```

---

### GET http://localhost:8080/api/v1/quidditchplayers/position/

Returns all Quidditch positions.

```json
{
    "data": {
        "1": {
            "positionName": "Chaser"
        },
        "4": {
            "positionName": "Seeker"
        },
        "3": {
            "positionName": "Keeper"
        },
        "2": {
            "positionName": "Beater"
        }
    },
    "status_code": 200
}
```

---

### GET http://localhost:8080/api/v1/quidditchplayers/player/

Returns all players. Optionally filter by house with query param `?houseName=GRYFFINDOR`.

```json
{
    "data": [
        {
            "id": "fd1f2deb-9637-4214-b991-a1b8daf18a7b",
            "firstName": "Harry",
            "lastName": "Potter",
            "yearsPlayed": [1991, 1992, 1993, 1994, 1995, 1996, 1997],
            "favoriteSubject": "Defense Against The Dark Arts",
            "position": 4,
            "imageUrl": "https://static.wikia.nocookie.net/harrypotter/images/c/ce/Harry_Potter_DHF1.jpg/revision/latest",
            "house": "GRYFFINDOR"
        }
    ],
    "status_code": 200
}
```

---

### GET http://localhost:8080/api/v1/quidditchplayers/player/status/

Returns status for all players. Optionally filter by house with query param `?houseName=GRYFFINDOR`.

```json
{
    "data": [
        {
            "playerId": "fd1f2deb-9637-4214-b991-a1b8daf18a7b",
            "status": "Harry is practicing Quidditch 🏏"
        }
    ],
    "status_code": 200
}
```

---

### GET http://localhost:8080/api/v1/quidditchplayers/player/status/{player_id}

Returns status for a single player by UUID.

```json
{
    "data": [
        {
            "playerId": "fd1f2deb-9637-4214-b991-a1b8daf18a7b",
            "status": "Harry is eating in the Great Hall 🍗"
        }
    ],
    "status_code": 200
}
```