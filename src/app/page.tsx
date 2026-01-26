'use client';
import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';
import KeyboardDisplay from '@/app/components/Keyboard';


export default function LedDashboard() {
  const [color, setColor] = useState({ r: 255, g: 0, b: 0 });

  const startRainbow = () => invoke('apply_effect', { 
    effectType: 'rainbow', r: 0, g: 0, b: 0 
  });

  const setStatic = () => invoke('apply_effect', { 
    effectType: 'static', ...color 
  });

  const turnOff = () => invoke('apply_effect', { 
    effectType: 'off', r: 0, g: 0, b: 0 
  });

  return (
    <main className="p-8 flex flex-col gap-4 items-center">
      <KeyboardDisplay />
      <h1 className="text-2xl font-bold">LED Controller</h1>
      
      <div className="flex gap-2">
        <button onClick={startRainbow} className="bg-blue-600 px-4 py-2 rounded">
          Rainbow (Animated 0x04)
        </button>
        
        <button onClick={setStatic} className="bg-red-600 px-4 py-2 rounded">
          Static (Range 0x05)
        </button>

        <button onClick={turnOff} className="bg-gray-700 px-4 py-2 rounded">
          All Off
        </button>
      </div>

      <input 
        type="color" 
        onChange={(e) => {
          // Convert hex to RGB for the static effect
          const hex = e.target.value;
          setColor({
            r: parseInt(hex.slice(1, 3), 16),
            g: parseInt(hex.slice(3, 5), 16),
            b: parseInt(hex.slice(5, 7), 16),
          });
        }}
      />
    </main>
  );
}