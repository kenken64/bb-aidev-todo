#!/bin/bash

echo "Starting Todo App..."

# Start the Rust backend in the background
echo "Starting Rust backend server..."
cargo run &
BACKEND_PID=$!

# Wait a moment for the backend to start
sleep 3

# Start the Angular frontend
echo "Starting Angular frontend..."
cd frontend
npm start &
FRONTEND_PID=$!

# Wait for user to stop the servers
echo "Servers started. Press Ctrl+C to stop both servers."
trap "echo 'Stopping servers...'; kill $BACKEND_PID $FRONTEND_PID" EXIT

wait