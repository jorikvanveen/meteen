import { useEffect, useState } from "react";
import CategoryButton from "./CategoryButton";

export default function Overview() {
  const categories = [
    { name: "Home", id: 1 },
    { name: "Work", id: 2 }
  ];

  const [focusedCategory, setFocusedCategory] = useState(0);

  useEffect(() => {

  }, [focusedCategory])

  return (
    <div className="flex w-full h-full flex-row flex-nowrap">
      <div className="grow-2">a</div>
      <div className="h-full border-0 border-l-1 grow-1 p-4 flex flex-col gap-2">
        {categories.map((category, i) => (<CategoryButton category={category.name} current={focusedCategory == i} onSelected={() => setFocusedCategory(i)} />))}
      </div>
    </div>
  );
}
