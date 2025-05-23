import { useState } from "react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import Backend from "@/lib/backend";
import type { CategoryModel } from "@/lib/backend-bindings/bindings";

interface Props {
  onNewCategory: (_: CategoryModel) => void;
}

export default function CreateCategory(props: Props) {
  const [expanded, setExpanded] = useState(false);
  const [categoryName, setCategoryName] = useState("");

  async function onSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()

    if (!expanded) {
      setExpanded(true) 
      return
    }
    
    const newCategory = await Backend.createCategory({ name: categoryName });
    setCategoryName("")
    setExpanded(false)
    props.onNewCategory(newCategory)
  }

  return <form className={`transition-all flex flex-row justify-between mt-2 gap-4 ${expanded ? "p-4 border" : ""}`} onSubmit={onSubmit}>
    <div className={`${expanded ? "" : "invisible w-0"}`}>
      <Input value={categoryName} onBlur={() => setExpanded(false)} onChange={(e) => setCategoryName(e.target.value)} />
    </div>
    <Button className="w-12" variant={expanded ? "default" : "secondary"}>+</Button>
  </form>
}
