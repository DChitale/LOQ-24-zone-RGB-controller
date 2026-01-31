"use client";
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Cpu, Terminal } from 'lucide-react';

export default function Settings() {
  const [vid, setVid] = useState("17ef");
  const [pid, setPid] = useState("6124");
  const [status, setStatus] = useState<string | null>(null);

  const connect = async () => {
    try {
      const res: string = await invoke('connect_keyboard', { vidStr: vid, pidStr: pid });
      setStatus(`Success: ${res}`);
    } catch (e) {
      setStatus(`Error: ${String(e)}`);
    }
  };

  return (
    <div className="flex flex-col md:flex-row h-screen bg-[#09090b] text-zinc-400 font-sans">
      
      {/* LEFT: SIDEBAR */}
      <div className="w-full md:w-64 border-r border-zinc-900 p-8 flex flex-col">
        <div className="space-y-12">
          <div className="flex items-center gap-2">
            <Cpu size={14} className="text-zinc-600" />
            <h1 className="text-[10px] font-black uppercase tracking-[0.4em] text-zinc-600">Hardware</h1>
          </div>
          
          <div className="space-y-6">
            <div className="space-y-1">
              <label className="text-[9px] text-zinc-700 uppercase font-bold tracking-widest">Protocol</label>
              <div className="text-[11px] text-zinc-300 py-2 border-b border-zinc-900">
                HID_USB_OVERRIDE
              </div>
            </div>
          </div>
        </div>

        <div className="mt-auto space-y-4">
           <div className="h-[1px] w-full bg-zinc-900" />
           <div className="flex items-center gap-2">
              <div className={`w-1 h-1 rounded-full ${status?.includes('Error') ? 'bg-red-500' : status ? 'bg-emerald-500' : 'bg-zinc-800'}`} />
              <p className="text-[8px] uppercase tracking-widest text-zinc-800 font-mono">
                {status ? 'Log_Updated' : 'Hardware_Standby'}
              </p>
           </div>
        </div>
      </div>

      {/* RIGHT: WORKSPACE */}
      <div className="flex-1 overflow-y-auto px-10 md:px-20 py-16">
        <div className="max-w-xl space-y-16">
          <header className="space-y-2">
            <h2 className="text-3xl font-light tracking-tight text-zinc-100 italic">USB Configuration</h2>
            <p className="text-[11px] text-zinc-600 uppercase tracking-widest font-medium">Link device via Vendor and Product identifiers</p>
          </header>

          <div className="space-y-12">
            {/* Input Fields */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
              <div className="space-y-3 group">
                <label className="text-[10px] font-bold text-zinc-500 uppercase tracking-[0.2em] group-focus-within:text-white transition-colors">Vendor_ID</label>
                <input 
                  value={vid} 
                  onChange={(e) => setVid(e.target.value)}
                  className="w-full bg-transparent border-b border-zinc-900 py-2 text-sm font-mono text-zinc-200 outline-none focus:border-zinc-500 transition-colors"
                  placeholder="0000"
                />
              </div>
              <div className="space-y-3 group">
                <label className="text-[10px] font-bold text-zinc-500 uppercase tracking-[0.2em] group-focus-within:text-white transition-colors">Product_ID</label>
                <input 
                  value={pid} 
                  onChange={(e) => setPid(e.target.value)}
                  className="w-full bg-transparent border-b border-zinc-900 py-2 text-sm font-mono text-zinc-200 outline-none focus:border-zinc-500 transition-colors"
                  placeholder="0000"
                />
              </div>
            </div>

            {/* Action Button */}
            <div className="pt-8">
              <button 
                onClick={connect}
                className="group relative flex items-center gap-4 text-white uppercase text-[10px] font-bold tracking-[0.3em] transition-all hover:gap-6"
              >
                <span className="relative z-10">Initialize Interface</span>
                <div className="h-[1px] w-12 bg-zinc-800 group-hover:w-20 group-hover:bg-white transition-all" />
              </button>
            </div>

            {/* Minimal Console Output */}
            {status && (
              <div className="mt-12 p-4 bg-zinc-950/50 border border-zinc-900 animate-in fade-in slide-in-from-top-2">
                <div className="flex items-center gap-2 mb-2">
                  <Terminal size={10} className="text-zinc-700" />
                  <span className="text-[8px] uppercase tracking-widest text-zinc-700 font-bold">System_Response</span>
                </div>
                <p className="text-[11px] font-mono text-zinc-500 lowercase leading-relaxed">
                  {status}
                </p>
              </div>
            )}
          </div>
        </div>
      </div>

    </div>
  );
}