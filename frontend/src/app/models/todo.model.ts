export interface Todo {
  id: string;
  title: string;
  completed: boolean;
  created_at: string;
}

export interface CreateTodo {
  title: string;
}

export interface UpdateTodo {
  title?: string;
  completed?: boolean;
}