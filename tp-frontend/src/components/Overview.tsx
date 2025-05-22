export default function Overview() {
  const categories = [
    {name: "Home", id: 1},
    {name: "Work", id: 2}
  ];

  return (
    <div className="flex w-full h-full flex-row flex-nowrap">
      <div className="grow-2">a</div>
      <div className="h-full border-0 border-l-1 grow-1 p-4">{categories.map(category => (<>
         
      </>))}</div>

    </div>
  );
}
