# HNG Stage 0 - Name Classification API

##  Description

This is a simple REST API built with Rust (Actix Web) that classifies a given name by gender using the Genderize API.
It processes the response and returns a structured JSON output based on specified rules.

---

##  Endpoint

### GET /api/classify?name={name}

---

##  Example Request

```bash
GET /api/classify?name=john
```

---

## Example Response

```json
{
  "status": "success",
  "data": {
    "name": "john",
    "gender": "male",
    "probability": 0.99,
    "sample_size": 1234,
    "is_confident": true,
    "processed_at": "2026-04-15T12:32:16Z"
  },
  "message": null
}
```

---

##  Error Response Example

```json
{
  "status": "error",
  "message": "Name is required"
}
```

---

##  Technologies Used

* Rust
* Actix Web
* Reqwest
* Serde
* Chrono
* Actix CORS

---

##  Live URL

```
https://your-app-url/api/classify?name=john
```

---

##  Logic Implemented

* Extracts gender, probability, and count from Genderize API
* Renames count → sample_size
* Computes `is_confident`:

  * true if probability ≥ 0.7 AND sample_size ≥ 100
* Generates timestamp in UTC (ISO 8601 format)
* Handles edge cases (null gender or count = 0)

---

## Author

Abdulmalik Sani Zagam
