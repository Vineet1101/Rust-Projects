import { Metadata } from "next";
import Link from "next/link";
export const metadata:Metadata={
  title:'Stock Details and Analysis | Portfolio AI',
  description:'AI-powered equity research and paper trading'
}


export default function Home() {
  return (
    <div className="">
      <h1 className="text-lg text-center m-6">Your AI Equity Research Analyst</h1>
      <div className="">
        <h2 className="text-center text-2xl">Features</h2>
        <div className="flex gap-3 ">
          <div>Feature1</div>
          <div>Feature2</div>
          <div>Feature3</div>
        </div>
      </div>
      <div className="mt-5">
        <p>CTA</p>
        <Link href="/dashboard">Dashbaord</Link>
      </div>
    </div>
  );
}
