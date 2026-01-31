"use client";
import { useState } from 'react';
import { Settings, Shield, Zap, Info, Database } from 'lucide-react';

export default function SettingsPage() {
  const [performanceMode, setPerformanceMode] = useState(false);
  const [hardwareAccel, setHardwareAccel] = useState(true);

  return (
    <div className="flex flex-col md:flex-row h-screen bg-[#050505] text-zinc-400 font-sans selection:bg-zinc-800">
      
      {/* LEFT: SETTINGS CATEGORIES */}
      <div className="w-full md:w-72 border-r border-zinc-900 p-10 flex flex-col justify-between bg-[#080808]/30">
        <div className="space-y-12">
          <header className="space-y-2">
            <div className="w-4 h-[2px] bg-zinc-500" />
            <h2 className="text-[10px] font-black tracking-[0.4em] uppercase text-zinc-600">Preferences</h2>
          </header>

          <nav className="flex flex-col gap-1">
            {['General', 'Performance', 'Interface', 'Security', 'About'].map((item) => (
              <button 
                key={item}
                className={`text-left px-4 py-3 text-[11px] uppercase tracking-widest transition-all ${
                  item === 'General' ? 'text-white border-l border-white' : 'text-zinc-600 hover:text-zinc-300'
                }`}
              >
                {item}
              </button>
            ))}
          </nav>
        </div>

        <div className="space-y-4 opacity-40">
           <div className="flex items-center gap-2">
              <Info size={10} />
              <span className="text-[8px] uppercase tracking-widest font-mono">v2.4.0-Stable</span>
           </div>
        </div>
      </div>

      {/* RIGHT: CONFIGURATION WORKSPACE */}
      <div className="flex-1 overflow-y-auto px-10 md:px-24 py-20">
        <div className="max-w-2xl space-y-20">
          
          {/* Header */}
          <section className="space-y-4">
            <h1 className="text-4xl font-extralight tracking-tighter text-white italic">Control Center</h1>
            <p className="text-[10px] text-zinc-500 uppercase tracking-widest font-medium">Configure core engine parameters and hardware link state</p>
          </section>

          {/* Performance Section */}
          <section className="space-y-10">
            <div className="flex items-center gap-3">
              <Zap size={14} className="text-zinc-700" />
              <h3 className="text-[10px] font-bold text-zinc-500 uppercase tracking-[0.3em]">Engine_Performance</h3>
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between py-6 border-b border-zinc-900 group">
                <div>
                  <h4 className="text-sm font-light text-zinc-200 uppercase tracking-tight">Run Effects at 60 fps</h4>
                  <p className="text-[10px] text-zinc-600 mt-1">Default 25 fps for better performance</p>
                </div>
                <button 
                  onClick={() => setPerformanceMode(!performanceMode)}
                  className={`w-10 h-5 flex items-center px-1 transition-colors duration-300 ${performanceMode ? 'bg-white' : 'bg-zinc-800'}`}
                >
                  <div className={`w-3 h-3 transition-transform duration-300 ${performanceMode ? 'translate-x-5 bg-black' : 'translate-x-0 bg-zinc-500'}`} />
                </button>
              </div>

              <div className="flex items-center justify-between py-6 border-b border-zinc-900 group">
                <div>
                  <h4 className="text-sm font-light text-zinc-200 uppercase tracking-tight">GPU Acceleration</h4>
                  <p className="text-[10px] text-zinc-600 mt-1">Offload visual effects to graphics processor</p>
                </div>
                <button 
                  onClick={() => setHardwareAccel(!hardwareAccel)}
                  className={`w-10 h-5 flex items-center px-1 transition-colors duration-300 ${hardwareAccel ? 'bg-white' : 'bg-zinc-800'}`}
                >
                  <div className={`w-3 h-3 transition-transform duration-300 ${hardwareAccel ? 'translate-x-5 bg-black' : 'translate-x-0 bg-zinc-500'}`} />
                </button>
              </div>
            </div>
          </section>

          {/* Diagnostics Section */}
          <section className="space-y-10">
            <div className="flex items-center gap-3">
              <Database size={14} className="text-zinc-700" />
              <h3 className="text-[10px] font-bold text-zinc-500 uppercase tracking-[0.3em]">Data_Management</h3>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
               <div className="p-6 border border-zinc-900 space-y-4">
                  <span className="text-[9px] text-zinc-700 uppercase tracking-widest font-bold">Local Storage</span>
                  <div className="text-xl font-light text-zinc-300 tabular-nums">14.2 MB</div>
                  <button className="text-[9px] text-zinc-500 uppercase font-black tracking-widest hover:text-white transition-colors">Clear Cache</button>
               </div>
               <div className="p-6 border border-zinc-900 space-y-4">
                  <span className="text-[9px] text-zinc-700 uppercase tracking-widest font-bold">Script Archives</span>
                  <div className="text-xl font-light text-zinc-300 tabular-nums">42 Units</div>
                  <button className="text-[9px] text-zinc-500 uppercase font-black tracking-widest hover:text-white transition-colors">Export All</button>
               </div>
            </div>
          </section>

          {/* Footer State */}
          <footer className="pt-10 flex items-center justify-between border-t border-zinc-900 opacity-20 group hover:opacity-100 transition-opacity">
            <div className="flex items-center gap-2">
              <Shield size={12} />
              <span className="text-[9px] uppercase tracking-[0.3em]">Encrypted_Link_Secure</span>
            </div>
            <span className="text-[9px] font-mono tracking-tighter">05:05:05_STANDBY</span>
          </footer>

        </div>
      </div>

    </div>
  );
}