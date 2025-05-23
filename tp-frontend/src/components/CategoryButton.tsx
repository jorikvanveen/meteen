import { Button } from "./ui/button"

interface Props {
  category: string  
  current: boolean;
  onSelected: () => void;
}

export default function CategoryButton(props: Props) {
  return <Button onClick={props.onSelected} variant={props.current ? "default" : "outline"}>{props.category}</Button>
}
