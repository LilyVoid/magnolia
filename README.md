# magnolia

Backend for lilypedia. In very early development.

## Setup

1. Clone the repo:

   ```sh
   git clone https://github.com/LilyVoid/magnolia.git
   cd magnolia
   ```

2. Create a `.env` file:

   ```sh
   cp .env.example .env
   ```

   Adjust credentials if needed.

3. Start the database:

   ```sh
   docker compose up -d db
   ```

4. Run migrations:

   ```sh
   cargo sqlx migrate run
   ```

5. Run the server:

   ```sh
   cargo run
   ```

---

## API (so far)

* `GET /info` – returns version
* `POST /markdown` – convert markdown to HTML

---

## Notes

* Using Postgres 16 via Docker
* All code and infra are WIP

---

MIT licensed
