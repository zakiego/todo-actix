# Todo App with Actix Web 📝🌐

This is a simple Todo application built with Actix Web, a powerful and ergonomic web framework for Rust. Manage your tasks effortlessly with this easy-to-use RESTful API.

## Features 🚀

- **Get all Todos:** Retrieve a list of all your todos. 📋
- **Get Todo by ID:** Fetch a specific todo based on its ID. 🕵️‍♂️
- **Update Todo:** Modify a todo's title or completion status. 🛠️
- **Create Todo:** Add a brand new todo to your list. 🆕
- **Delete Todo:** Remove a todo based on its ID. 🗑️

## Usage 🛠️

### Getting Started

1. Clone this repository and enter the Todo Funhouse.

```bash
git clone https://github.com/zakiego/todo-actix.git
cd todo-actix
```

2. Run the magic potion to unleash the Todo spirits!

```bash
cargo run
```

3. Access the enchanted API at [http://127.0.0.1:8080](http://127.0.0.1:8080).

### API Endpoints

- **Get all Todos:**

  ```http
  GET /todos
  ```

- **Get Todo by ID:**

  ```http
  GET /todo/{id}
  ```

- **Update Todo by ID:**

  ```http
  POST /todo/update/{id}
  ```

- **Create Todo:**

  ```http
  POST /todo/create
  ```

- **Delete Todo by ID:**

  ```http
  GET /todo/delete/{id}
  ```

## Dependencies 📦

- `actix-web`: The secret sauce for our web application.
- `serde`: The wizard behind the curtain for serializing and deserializing our data.

---

Feel free to add your own touch of magic! 🌟
