# Rust Weather API

This is a Rust-based HTTP API for retrieving weather data. The application uses `actix-web` as the web framework and `serde` for JSON serialization and deserialization. It provides endpoints to fetch weather data for various countries, cities, and months. Thisn is awesome

## Features

- Redirects the root route (`/`) to `/docs`.
- Lists all available countries with weather data.
- Fetches monthly weather data for a specific country and city.

---

## Installation

To set up the project, ensure you have the following prerequisites:

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.72 or later)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/rust-weather-api.git
   cd rust-weather-api/workshop/rust-app
   ```

---

## Usage

To run the application locally, use the following commands:

1. Build the project:
   ```bash
   cargo build
   ```

2. Run the project:
   ```bash
   cargo run
   ```

3. The API will be available at `http://localhost:8080`.

---

## Endpoints

### Redirect to Documentation
- **GET** `/`
  - Redirects to `/docs`.

### List Countries
- **GET** `/countries`
  - Returns a list of all available countries with weather data.

### Fetch Monthly Weather Data
- **GET** `/weather/{country}/{city}/{month}`
  - Parameters:
    - `country`: The name of the country (e.g., "USA").
    - `city`: The name of the city (e.g., "New York").
    - `month`: The month (e.g., "January").
  - Returns:
    - JSON object containing weather data for the specified country, city, and month.

---

## Contributing

We welcome contributions! To contribute:

1. Fork the repository.
2. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature-name
   ```
3. Make your changes and commit them:
   ```bash
   git commit -m "Description of changes"
   ```
4. Push your branch:
   ```bash
   git push origin feature-name
   ```
5. Open a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.
