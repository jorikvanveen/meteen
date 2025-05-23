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

  public static async patchTask(task: bindings.PatchTask, id: BigInt) {
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

  public static async listCategoryTasks(category_id: BigInt) {
    const response = await fetch(`${BASE_URL}/category/${category_id}/tasks`);

    if (!response.ok) {
      console.error(await response.text())
      throw new Error("Failed to list category")
    }
  }
}
