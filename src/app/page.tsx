import Greet from "./greet";
//import Sidebar from "@/app/components/Sidebar";
import Keyboard from "@/app/components/Keyboard";


export default function Home() {
  return (

    <div className="flex h-screen overflow-hidden bg-zinc-950">
      
      <main className="flex-1 overflow-y-auto">
        {/* Your Keyboard Grid goes here */}
        <Keyboard />
       
      </main>
    </div>
  );
}
