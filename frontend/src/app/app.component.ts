import { Component, OnInit, ViewChild, ElementRef } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { TodoService } from './services/todo.service';
import { Todo, CreateTodo, UpdateTodo } from './models/todo.model';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'Todo App';
  todos: Todo[] = [];
  newTodoTitle = '';
  editingTodo: Todo | null = null;
  editTitle = '';

  @ViewChild('editInput') editInput!: ElementRef<HTMLInputElement>;

  constructor(private todoService: TodoService) {}

  ngOnInit() {
    this.loadTodos();
  }

  loadTodos() {
    this.todoService.getTodos().subscribe({
      next: (todos) => this.todos = todos,
      error: (error) => console.error('Error loading todos:', error)
    });
  }

  addTodo() {
    if (this.newTodoTitle.trim()) {
      const createTodo: CreateTodo = {
        title: this.newTodoTitle.trim()
      };
      
      this.todoService.createTodo(createTodo).subscribe({
        next: (todo) => {
          this.todos.unshift(todo);
          this.newTodoTitle = '';
        },
        error: (error) => console.error('Error creating todo:', error)
      });
    }
  }

  toggleComplete(todo: Todo) {
    const updateTodo: UpdateTodo = {
      completed: !todo.completed
    };

    this.todoService.updateTodo(todo.id, updateTodo).subscribe({
      next: (updatedTodo) => {
        const index = this.todos.findIndex(t => t.id === todo.id);
        if (index !== -1) {
          this.todos[index] = updatedTodo;
        }
      },
      error: (error) => console.error('Error updating todo:', error)
    });
  }

  startEditing(todo: Todo) {
    this.editingTodo = todo;
    this.editTitle = todo.title;
    
    // Focus the input field after view update
    setTimeout(() => {
      if (this.editInput) {
        this.editInput.nativeElement.focus();
        this.editInput.nativeElement.select();
      }
    }, 0);
  }

  cancelEditing() {
    this.editingTodo = null;
    this.editTitle = '';
  }

  saveEdit() {
    console.log('saveEdit called', { editingTodo: this.editingTodo, editTitle: this.editTitle });
    
    if (this.editingTodo && this.editTitle.trim()) {
      const updateTodo: UpdateTodo = {
        title: this.editTitle.trim()
      };

      console.log('Sending update request', { id: this.editingTodo.id, updateTodo });

      this.todoService.updateTodo(this.editingTodo.id, updateTodo).subscribe({
        next: (updatedTodo) => {
          console.log('Update successful', updatedTodo);
          const index = this.todos.findIndex(t => t.id === this.editingTodo!.id);
          if (index !== -1) {
            this.todos[index] = updatedTodo;
          }
          this.cancelEditing();
        },
        error: (error) => {
          console.error('Error updating todo:', error);
          // Don't cancel editing on error so user can try again
        }
      });
    } else {
      console.log('Save conditions not met', { 
        hasEditingTodo: !!this.editingTodo, 
        hasTitleContent: !!this.editTitle.trim() 
      });
    }
  }

  deleteTodo(todo: Todo) {
    this.todoService.deleteTodo(todo.id).subscribe({
      next: () => {
        this.todos = this.todos.filter(t => t.id !== todo.id);
      },
      error: (error) => console.error('Error deleting todo:', error)
    });
  }

  onKeyPress(event: KeyboardEvent, action: 'add' | 'edit') {
    if (event.key === 'Enter') {
      if (action === 'add') {
        this.addTodo();
      } else if (action === 'edit') {
        this.saveEdit();
      }
    }
  }
}