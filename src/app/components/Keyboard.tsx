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
  // 24 zones for the Lenovo LOQ
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
    <div className="flex flex-col items-center justify-center w-full min-h-[500px] p-4 lg:p-12 select-none">

      <div className="relative w-full max-w-[1280px] min-w-[640px] aspect-[1280/510] bg-[#050505] rounded-2xl border border-zinc-900 shadow-[0_0_60px_rgba(0,0,0,0.6)]">

        {/* LAYER 1: The Underglow */}
        <div className="absolute inset-0 flex px-1 pt-4 pb-2 justify-between items-stretch">
          {zones.map((color, i) => (
            <motion.div
              key={i}
              animate={{
                backgroundColor: color,
                // Using the color string directly for the glow
                boxShadow: `0 0 2px 5px ${color.replace('rgb', 'rgba').replace(')', ', 0.1)')}`
                //boxShadow: 'none'
              }}
              // Faster transition for real-time responsiveness
              transition={{ duration: 0.1, ease: "linear" }}
              className="h-full flex-1"
              style={{ filter: 'blur(18px)' }}
            />
          ))}
        </div>

        {/* LAYER 2: The Physical Keyboard PNG */}
        <img
          src="layout.png"
          alt="Lenovo LOQ US Layout"
          className="absolute inset-0 z-10 w-full h-full object-contain object-bottom pointer-events-none drop-shadow-2xl"
        />

        {/* LAYER 3: Interactive Hitboxes */}
        <div className="absolute inset-0 z-20 flex px-1 pt-4 pb-2">
          {zones.map((color, i) => (
            <button
              key={i}
              onClick={() => console.log(`Zone ${i + 1} clicked`, color)}
              className="h-full flex-1 hover:bg-white/20 transition-colors group relative rounded-b-md"
            >
              <div className="absolute inset-x-0 bottom-6 flex justify-center opacity-0 group-hover:opacity-100 transition-all transform translate-y-2 group-hover:translate-y-0">
                <span className="bg-zinc-950/90 text-[8px] font-mono text-zinc-500 px-2 py-1 rounded border border-zinc-800 shadow-xl tracking-tighter">
                  -/{String(i + 1).padStart(2, '0')}
                </span>
              </div>
            </button>
          ))}
        </div>

      </div>

      {/* Info Footer */}
      {/* <div className="mt-12 flex items-center gap-8 opacity-40 hover:opacity-100 transition-opacity duration-500">
        <div className="flex flex-col items-start">
          <span className="text-[8px] text-zinc-600 uppercase tracking-[0.3em] font-black">Resolution</span>
          <span className="text-[10px] text-zinc-400 font-mono">1280x480 (1:1 Native)</span>
        </div>
        <div className="h-6 w-px bg-zinc-800" />
        <div className="flex flex-col items-start">
          <span className="text-[8px] text-zinc-600 uppercase tracking-[0.3em] font-black">Geometry</span>
          <span className="text-[10px] text-zinc-400 font-mono">LOQ Offset-Arrow array</span>
        </div>
        <div className="h-6 w-px bg-zinc-800" />
        <div className="flex flex-col items-start">
          <span className="text-[8px] text-zinc-600 uppercase tracking-[0.3em] font-black">Backend</span>
          <span className="text-[10px] text-zinc-400 font-mono">Connected via HID Mutex</span>
        </div>
      </div> */}

    </div>
  );
}