"use client";
import { useState } from 'react';
import { motion } from 'framer-motion';

export default function KeyboardDisplay() {
  // 24 zones for the Lenovo LOQ
  const [zones, setZones] = useState<string[]>(Array(24).fill("#1a1a1a"));

  return (
    <div className="flex flex-col items-center justify-center w-full min-h-[500px] p-4 lg:p-12 select-none">
      
      {/* WRAPPER ADJUSTMENTS:
        - Aspect changed to 1280/510 to give room for protruding arrows.
        - Overflow-hidden removed or handled carefully to prevent clipping the protrusion.
      */}
      <div className="relative w-full max-w-[1280px] min-w-[640px] aspect-[1280/510] bg-[#050505] rounded-2xl border border-zinc-900 shadow-[0_0_60px_rgba(0,0,0,0.6)]">
        
        {/* LAYER 1: The Underglow 
            Increased blur and slight vertical padding adjustment to reach the arrow zone.
        */}
        <div className="absolute inset-0 flex px-1 pt-4 pb-2 justify-between items-stretch">
          {zones.map((color, i) => (
            <motion.div
              key={i}
              animate={{ 
                backgroundColor: color,
                boxShadow: `0 0 70px 25px ${color}10` 
              }}
              className="h-full flex-1 transition-all duration-700"
              style={{ filter: 'blur(18px)' }} 
            />
          ))}
        </div>

        {/* LAYER 2: The Physical Keyboard PNG
            Using 'object-bottom' so the main chassis sits at the base, 
            letting the protrusion occupy the natural bottom space.
        */}
        <img 
          src="layout.png" 
          alt="Lenovo LOQ US Layout"
          className="absolute inset-0 z-10 w-full h-full object-contain object-bottom pointer-events-none drop-shadow-2xl"
        />

        {/* LAYER 3: Interactive Hitboxes */}
        <div className="absolute inset-0 z-20 flex px-1 pt-4 pb-2">
          {zones.map((_, i) => (
            <button
              key={i}
              onClick={() => console.log(`Zone ${i + 1} clicked`)}
              className="h-full flex-1 hover:bg-white/5 transition-colors group relative rounded-b-md"
            >
              <div className="absolute inset-x-0 bottom-6 flex justify-center opacity-0 group-hover:opacity-100 transition-all transform translate-y-2 group-hover:translate-y-0">
                <span className="bg-zinc-950/90 text-[8px] font-mono text-zinc-500 px-2 py-1 rounded border border-zinc-800 shadow-xl tracking-tighter">
                  ZONE {String(i + 1).padStart(2, '0')}
                </span>
              </div>
            </button>
          ))}
        </div>

      </div>

      {/* Info Footer */}
      <div className="mt-12 flex items-center gap-8 opacity-40 hover:opacity-100 transition-opacity duration-500">
        <div className="flex flex-col items-start">
          <span className="text-[8px] text-zinc-600 uppercase tracking-[0.3em] font-black">Resolution</span>
          <span className="text-[10px] text-zinc-400 font-mono">1280x480 (1:1 Native)</span>
        </div>
        <div className="h-6 w-px bg-zinc-800" />
        <div className="flex flex-col items-start">
          <span className="text-[8px] text-zinc-600 uppercase tracking-[0.3em] font-black">Geometry</span>
          <span className="text-[10px] text-zinc-400 font-mono">LOQ Offset-Arrow array</span>
        </div>
      </div>

    </div>
  );
}