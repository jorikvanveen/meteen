import { useEffect, useState } from "react";
import CategoryButton from "./CategoryButton";
import Backend from "@/lib/backend";
import type { CategoryModel, TaskModel } from "@/lib/backend-bindings/bindings";
import { Button } from "./ui/button";
import CreateCategory from "./CreateCategory";

export default function Overview() {
  const [categories, setCategories] = useState<CategoryModel[]>([]);
  const [focusedCategory, setFocusedCategory] = useState(0);
  const [tasks, setTasks] = useState<TaskModel[]>([])

  useEffect(() => {
    (async () => {
      console.log(categories[focusedCategory], focusedCategory);
      setTasks(await Backend.listCategoryTasks(categories[focusedCategory].id))
    })();
  }, [focusedCategory])

  useEffect(() => {
    (async () => {
      setCategories(await Backend.listCategories())
    })();
  }, [])

  function onNewCategory(category: CategoryModel) {
    setCategories([...categories, category])
    setFocusedCategory(categories.length)
  }

  return (
    <div className="flex w-full h-full flex-row flex-nowrap">
      <div className="w-full">
        {tasks.map((task, i) => <span>{JSON.stringify(task)}</span>)};
      </div>
      <div className="h-full border-0 border-l-1 p-4 flex flex-col gap-2">
        {categories.map((category, i) => (<CategoryButton category={category.name} current={focusedCategory == i} onSelected={() => setFocusedCategory(i)} />))}
        <CreateCategory onNewCategory={onNewCategory} />
      </div>
    </div>
  );
}
