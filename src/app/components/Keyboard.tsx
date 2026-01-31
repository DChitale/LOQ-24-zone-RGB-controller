"use client";
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import { motion } from 'framer-motion';
import { listen } from '@tauri-apps/api/event';

interface Color {
  r: number;
  g: number;
  b: number;
}

export default function KeyboardDisplay() {
  const [zones, setZones] = useState<string[]>(Array(24).fill("#000000"));

  useEffect(() => {
    const unlisten = listen<Color[]>('new-colors', (event) => {
      const frame = event.payload;
      setZones(frame.map(c => `rgb(${c.r}, ${c.g}, ${c.b})`));
    });

    return () => {
      unlisten.then(f => f());
    };
  }, []);

  return (
    <div className="flex flex-col items-center justify-center w-full min-h-[200px] p-8 lg:p-20 select-none bg-[#050505]">
      
      {/* Container: Matches the Studio border-zinc-900 theme */}
      <div className="relative w-full max-w-[1100px] aspect-[1280/510] bg-[#080808] border border-zinc-900 rounded-sm overflow-hidden">

        {/* LAYER 1: The Underglow - Softened and desaturated slightly for minimalism */}
        <div className="absolute inset-0 flex px-1 pt-6 pb-2 justify-between items-stretch">
          {zones.map((color, i) => (
            <motion.div
              key={i}
              animate={{
                backgroundColor: color,
              }}
              transition={{ duration: 0.1, ease: "linear" }}
              className="h-full flex-1"
              style={{ 
                filter: 'blur(24px)', 
                opacity: 0.6 // Controlled opacity for studio look
              }}
            />
          ))}
        </div>

        {/* LAYER 2: The Physical Keyboard PNG */}
        <img
          src="layout.png"
          alt="Keyboard Layout"
          className="absolute inset-0 z-10 w-full h-full object-contain object-bottom pointer-events-none brightness-90 grayscale-[0.2]"
        />

        {/* LAYER 3: Interactive Hitboxes */}
        <div className="absolute inset-0 z-20 flex px-2 pt-6 pb-2">
          {zones.map((color, i) => (
            <button
              key={i}
              onClick={() => console.log(`Zone ${i + 1}`, color)}
              className="h-full flex-1 transition-all group relative"
            >
              {/* Minimal vertical indicator on hover */}
              <div className="absolute inset-x-[2px] top-0 bottom-0 border-x border-transparent group-hover:border-zinc-800 transition-colors" />
              
              <div className="absolute inset-x-0 bottom-8 flex justify-center opacity-0 group-hover:opacity-100 transition-all">
                <span className="bg-[#09090b] text-[9px] font-mono text-zinc-500 px-1.5 py-0.5 border border-zinc-800 tracking-widest uppercase">
                  Z_{String(i + 1).padStart(2, '0')}
                </span>
              </div>
            </button>
          ))}
        </div>

      </div>

      

    </div>
  );
}