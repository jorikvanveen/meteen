import type * as bindings from "./backend-bindings/bindings";

const BASE_URL = "http://localhost:3000";

export default class Backend {
  public static async createTask(task: bindings.CreateTask) {
    const response = await fetch(`${BASE_URL}/task`, {
      method: "post",
      body: JSON.stringify(task),
      headers: {
        "Content-Type": "application/json"
      }
    })

    if (response.ok) {
      return
    }
  }

  public static async patchTask(task: bindings.PatchTask, id: number) {
    const response = await fetch(`${BASE_URL}/task/${id}`, {
      method: "PATCH",
      body: JSON.stringify(task),
      headers: {
        "Content-Type": "application/json"
      }
    })

    if (!response.ok) {
      console.error(await response.text()) 
      throw new Error("Failed to update task")
    }
  }

  public static async createCategory(category: bindings.CreateCategory): Promise<bindings.CategoryModel> {
    const response = await fetch(`${BASE_URL}/category`, {
      method: "POST",
      body: JSON.stringify(category),
      headers: {
        "Content-Type": "application/json"
      }
    })
  
    if (!response.ok) {
      console.error(await response.text())
      throw new Error("Failed to create category")
    }

    return await response.json()
  }

  public static async listCategories(): Promise<bindings.CategoryModel[]> {
    const response = await fetch(`${BASE_URL}/category`); 

    if (!response.ok) {
      console.error(await response.text())
      throw new Error("Failed to list categories")
    }

    return await response.json() as bindings.CategoryModel[]
  }

  public static async listCategoryTasks(category_id: number): Promise<bindings.TaskModel[]> {
    const response = await fetch(`${BASE_URL}/category/${category_id}/tasks`);

    if (!response.ok) {
      console.error(await response.text())
      throw new Error("Failed to list category")
    }

    return await response.json() as bindings.TaskModel[]
  }
}
