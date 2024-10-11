# CortexLabs Audio Plugin Store Backend

The CortexLabs Audio Plugin Store Backend is a Rust-based backend API built using the Actix-web framework. It manages audio products and categories, providing RESTful API endpoints for creating, reading, updating, and deleting audio products and categories. This backend allows for easy management of audio plugins and their categorization, with support for concurrency and asynchronous operations.

## Features

- **CRUD operations** for audio products and categories.
- **Concurrency support** using Actix's asynchronous model.
- **Category-Product association**: Automatically updates category information when products are created or modified.
- **Built-in error handling** and request validation.
- **Patch requests** to update categories with newly created or modified products.
- **Endpoint logging** for better traceability.

## Prerequisites

- **Rust** (latest stable version recommended)
- **Cargo** (comes with Rust)
- **Actix-web** framework

Ensure that Rust and Cargo are installed on your system. If not, you can install them by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Getting Started

### Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/CortexLabs/audio-plugin-store-backend.git
    cd audio-plugin-store-backend
    ```

2. **Install dependencies**:
    Run the following command to install dependencies specified in `Cargo.toml`:
    ```bash
    cargo build
    ```

3. **Configure environment variables**:
    You may need to set up environment variables for configuration. Create a `.env` file in the root directory and add the necessary variables:
    ```
    DATABASE_URL='postgres://user:password@localhost/audio_plugin_store'
    SERVER_PORT='8080'
    ```

### Running the Application

To start the application, run:
```bash
cargo run
```


## API Endpoints

**Audio Products**

- **GET** /plugin_store/products:** Get all audio products.
- **GET /plugin_store/products/{id}:** Get a specific audio product by its ID.
- **POST /plugin_store/products:** Create a new audio product.
    - Request body format:

```rust
{
  'name': 'Product Name',
  'price': 99.99,
  'description': 'Detailed description of the plugin',
  'plugin_format': 'VST/AU/AAX',
  'demo_href': 'http://example.com/demo',
  'image_src': 'http://example.com/image.png',
  'image_alt': 'Image description',
  'category_id': 1
}
```


- **PATCH /plugin_store/products/{id}:** Update an existing audio product.
    - Request body form

```rust
{
  'name': 'Updated Product Name',
  'price': 89.99,
  'description': 'Updated description',
  'plugin_format': 'AU',
  'demo_href': 'http://example.com/new_demo',
  'image_src': 'http://example.com/new_image.png',
  'image_alt': 'Updated image description'
}
```

- **DELETE /plugin_store/products/{id}:** Delete an audio product.

## Audio Categories

- **GET /plugin_store/categories:** Get all audio categories.
- **GET /plugin_store/categories/{id}:** Get a specific audio category by its ID.
- **POST /plugin_store/categories:** Create a new audio category.
    - Request body format:

```rust
{
  'name': 'Category Name',
  'products': [1, 2, 3]
}
```

- **PATCH /plugin_store/categories/{id}:** Update an existing audio category.
    - Request body format:

```rust
{
  'name': 'Updated Category Name',
  'products': [1, 4]
}
```

- **DELETE /plugin_store/categories/{id}:** Delete an audio category.

## Error Handling

- The backend provides appropriate HTTP status codes in response to errors:
    - **404 Not Found:** When a requested resource (product or category) is not found.
    - **400 Bad Request:** When the request body is missing required fields or contains invalid data.
    - **500 Internal Server Error:** For unexpected server-side errors.
    
## Concurrency Considerations

- The backend uses **Actix's** built-in support for asynchronous programming to avoid blocking.
- Locks (Mutex) are used to ensure safe access to shared state.
- To prevent deadlocks, locks are released before performing async operations.


