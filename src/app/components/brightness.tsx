"use client";
import { useState, useRef, useEffect } from 'react';
import { Sun } from 'lucide-react';
import { invoke } from '@tauri-apps/api/tauri';

export function VerticalBrightness() {
  const [brightness, setBrightness] = useState(50);
  const [isDragging, setIsDragging] = useState(false);
  const [status, setStatus] = useState('');
  const sliderRef = useRef<HTMLDivElement>(null);
  const debounceRef = useRef<number | null>(null);

  // Load persisted brightness on mount
  useEffect(() => {
    let mounted = true;
    (async () => {
      try {
        const s = await invoke<any>('get_settings');
        if (!mounted) return;
        const v = Math.round((s?.brightness_level ?? 1) * 100);
        setBrightness(v);
      } catch (err) {
        // ignore
      }
    })();
    return () => { mounted = false; };
  }, []);

  const sendToBackend = (pct: number, immediate = false) => {
    const value = Math.max(0, Math.min(100, pct));
    const b = value / 100;

    const doInvoke = async () => {
      try {
        const res = await invoke<string>('set_brightness', { brightness: b });
        setStatus(String(res));
        setTimeout(() => setStatus(''), 1500);
      } catch (err) {
        console.error('set_brightness failed', err);
        setStatus('Failed');
        setTimeout(() => setStatus(''), 2000);
      }
    };

    if (immediate) {
      if (debounceRef.current) {
        window.clearTimeout(debounceRef.current);
        debounceRef.current = null;
      }
      void doInvoke();
      return;
    }

    if (debounceRef.current) window.clearTimeout(debounceRef.current);
    debounceRef.current = window.setTimeout(() => {
      void doInvoke();
      debounceRef.current = null;
    }, 10);
  };

  const handleUpdate = (clientY: number) => {
    if (!sliderRef.current) return;
    
    const rect = sliderRef.current.getBoundingClientRect();
    const height = rect.height;
    // Calculate position from bottom up
    const y = Math.min(Math.max(0, rect.bottom - clientY), height);
    const percent = Math.round((y / height) * 100);
    setBrightness(percent);
    sendToBackend(percent, false);
  };

  const onMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    handleUpdate(e.clientY);
  };

  useEffect(() => {
    const onMouseMove = (e: MouseEvent) => {
      if (isDragging) handleUpdate(e.clientY);
    };
    const onMouseUp = () => {
      setIsDragging(false);
      // commit immediately on release
      sendToBackend(brightness, true);
    };

    if (isDragging) {
      window.addEventListener('mousemove', onMouseMove);
      window.addEventListener('mouseup', onMouseUp);
      document.body.style.cursor = 'crosshair'; // Locks cursor to + during drag
    }

    return () => {
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
      document.body.style.cursor = 'default';
    };
  }, [isDragging, brightness]);

  return (
    <div className="flex flex-col items-center gap-6 h-full w-full py-2 select-none">

        <div className="w-full h-[1px] bg-zinc-900 shadow-[0_1px_0_rgba(255,255,255,0.02)]" />
      
      {/* High Brightness Icon */}
      <Sun size={18} className="text-white/80" strokeWidth={2} />

      {/* Custom Slider Track */}
      <div 
        ref={sliderRef}
        onMouseDown={onMouseDown}
        className="relative flex-1 w-8 flex justify-center cursor-crosshair group"
      >
        {/* Visual Line (The Track) */}
        <div className="absolute w-[2px] h-full bg-zinc-800 group-hover:bg-zinc-700 transition-colors" />
        
        {/* The Fill (White Line) */}
        <div 
          className="absolute bottom-0 w-[4px] bg-white transition-all duration-75"
          style={{ height: `${brightness}%` }}
        >
            {/* The "Thumb" - A simple dot or small horizontal line */}
            <div className="absolute top-0 left-1/2 -translate-x-1/2 w-3 h-[1px] bg-white" />
        </div>
      </div>

      {/* Low Brightness Icon */}
      <Sun size={18} className="text-zinc-700" strokeWidth={1.5} />

      {/* Percentage & status */}
      <div className="flex flex-col items-center gap-1">
        <span className="text-[9px] font-mono text-zinc-500 tabular-nums">{brightness}%</span>
   
      </div>

    </div>
  );
}