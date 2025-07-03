# Full-Stack Todo Application

A modern todo application built with Angular frontend and Rust backend, featuring SQLite database for persistence.

## Tech Stack

- **Frontend**: Angular 17 with standalone components
- **Backend**: Rust with Axum web framework
- **Database**: SQLite with SQLx
- **Communication**: REST API with CORS support

## Features

- ✅ Create new todos
- ✅ Mark todos as complete/incomplete
- ✅ Edit existing todos (double-click or use edit button)
- ✅ Delete individual todos
- ✅ Persistent storage with SQLite
- ✅ Real-time UI updates
- ✅ Responsive design

## Project Structure

```
├── src/                    # Rust backend source
│   └── main.rs            # Main server implementation
├── migrations/            # Database migrations
│   └── 001_create_todos.sql
├── frontend/              # Angular application
│   ├── src/app/
│   │   ├── models/        # TypeScript interfaces
│   │   ├── services/      # HTTP services
│   │   └── app.component.* # Main component
│   └── proxy.conf.json    # Proxy configuration
├── Cargo.toml             # Rust dependencies
└── start.sh               # Convenience script to run both servers
```

## Setup and Installation

### Prerequisites

- Node.js (v20.11+)
- Rust (latest stable)
- npm or yarn

### Running the Application

1. **Option 1: Use the convenience script**
   ```bash
   ./start.sh
   ```

2. **Option 2: Run manually**
   
   Terminal 1 - Start the Rust backend:
   ```bash
   cargo run
   ```
   
   Terminal 2 - Start the Angular frontend:
   ```bash
   cd frontend
   npm start
   ```

3. Open your browser and navigate to `http://localhost:4200`

## API Endpoints

The Rust backend provides the following REST API endpoints:

- `GET /api/todos` - Get all todos
- `POST /api/todos` - Create a new todo
  ```json
  { "title": "Todo title" }
  ```
- `PUT /api/todos/:id` - Update a todo
  ```json
  { "title": "New title", "completed": true }
  ```
- `DELETE /api/todos/:id` - Delete a todo

## Configuration

### Backend Configuration

- Server runs on `http://localhost:3000`
- SQLite database file: `todos.db`
- CORS is configured to allow all origins for development

### Frontend Configuration

- Angular dev server runs on `http://localhost:4200` 
- Proxy configuration redirects `/api/*` calls to backend
- Configured in `frontend/proxy.conf.json`

## Development

### Backend Development

The Rust backend uses:
- **Axum** for the web framework
- **SQLx** for database operations
- **Tokio** for async runtime
- **Serde** for JSON serialization
- **Tower-HTTP** for CORS middleware

### Frontend Development

The Angular frontend uses:
- **Standalone Components** (Angular 17+)
- **HttpClient** for API communication
- **Reactive Forms** with ngModel
- **TypeScript** for type safety

## Database Schema

```sql
CREATE TABLE todos (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);
```

## Usage

1. **Add a todo**: Type in the input field and press Enter or click "Add"
2. **Mark complete**: Click the checkbox next to any todo
3. **Edit a todo**: Double-click the todo text or click the "Edit" button
4. **Delete a todo**: Click the "Delete" button
5. **Save edits**: Press Enter or click "Save" when editing

All changes are automatically persisted to the SQLite database.