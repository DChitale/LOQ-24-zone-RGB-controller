"use client";
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Cpu, RefreshCw } from 'lucide-react';

export default function settings() {
  const [vid, setVid] = useState("17ef");
  const [pid, setPid] = useState("6124");

  const connect = async () => {
    try {
      const res = await invoke('connect_keyboard', { vidStr: vid, pidStr: pid });
      alert(res);
    } catch (e) {
      alert(e);
    }
  };

  return (
    <div className="w-full max-w-lg p-12 bg-zinc-950/50 rounded-3xl border border-zinc-900 animate-in fade-in duration-500">
      <div className="flex items-center gap-4 mb-10">
        <Cpu className="text-zinc-500" size={20} />
        <h2 className="text-xs font-bold tracking-[0.4em] uppercase">USB Configuration</h2>
      </div>
      
      <div className="space-y-6">
        <div className="grid grid-cols-2 gap-4">
          <input 
            value={vid} onChange={(e) => setVid(e.target.value)}
            className="bg-zinc-900 border border-zinc-800 p-3 rounded-xl font-mono text-xs outline-none focus:border-white/20"
            placeholder="VID"
          />
          <input 
            value={pid} onChange={(e) => setPid(e.target.value)}
            className="bg-zinc-900 border border-zinc-800 p-3 rounded-xl font-mono text-xs outline-none focus:border-white/20"
            placeholder="PID"
          />
        </div>
        <button 
          onClick={connect}
          className="w-full h-12 bg-white text-black text-[10px] font-bold uppercase tracking-widest rounded-xl hover:bg-zinc-200"
        >
          Initialize Interface
        </button>
      </div>
    </div>
  );
}