import { Card } from "./ui/card";

interface Props {
  children?: React.ReactNode;
  className: string;
  title: string;
  titleRight?: boolean;
}

export default function HomeFunction(props: Props) {
  return <>
    <div className={`flex flex-col gap-4 h-full ${props.className}`}>
      <h1 className={`text-6xl font-bold ${props.titleRight ? "text-right" : ""}`}>{props.title}</h1>
      <Card className="h-full p-0">
        {props.children}
      </Card>
    </div>
  </>
}
