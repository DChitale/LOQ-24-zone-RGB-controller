"use client";

import { useEffect, useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Terminal,
  Trash2,
  CornerDownRight,
  Save,
  Play,
} from "lucide-react";

/* ---------------------------------------------
 * Types & Constants
 * -------------------------------------------- */

type ScriptEntry = {
  id: number;
  name: string;
  content: string;
};

const STORAGE_KEY = "core_scripts";

/* ---------------------------------------------
 * Utilities
 * -------------------------------------------- */

const loadScripts = (): ScriptEntry[] => {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return raw ? JSON.parse(raw) : [];
  } catch {
    return [];
  }
};

const persistScripts = (scripts: ScriptEntry[]) => {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(scripts));
};

/* ---------------------------------------------
 * Component
 * -------------------------------------------- */

export default function ScriptEngine() {
  const [code, setCode] = useState(`// core_init.json\n{\n  "sequence": "intercept",\n  "zones": [1, 24],\n  "rgb": [255, 255, 255],\n  "pulse": true\n}`);
  const [scripts, setScripts] = useState<ScriptEntry[]>([]);
  const [activeScriptId, setActiveScriptId] = useState<number | null>(null);

  useEffect(() => {
    setScripts(loadScripts());
  }, []);

  const saveScript = () => {
    const name = prompt("Identifier:"); // Simple prompt remains for logic parity, but styled as a studio tool
    if (!name) return;

    const newScript = { id: Date.now(), name, content: code };
    const next = [newScript, ...scripts];
    setScripts(next);
    persistScripts(next);
    setActiveScriptId(newScript.id);
  };

  const removeScript = (id: number) => {
    const next = scripts.filter(s => s.id !== id);
    setScripts(next);
    persistScripts(next);
    if (activeScriptId === id) setActiveScriptId(null);
  };

  return (
    <div className="flex flex-col md:flex-row h-screen bg-[#09090b] text-zinc-400 font-sans selection:bg-zinc-800">
      
      {/* LEFT: ARCHIVES (SELECTOR) */}
      <div className="w-full md:w-72 border-r border-zinc-900 flex flex-col p-8">
        <div className="space-y-12 flex-1 overflow-hidden flex flex-col">
          <header className="space-y-2">
            <div className="flex items-center gap-2">
              <Terminal size={12} className="text-zinc-600" />
              <h1 className="text-[10px] font-black uppercase tracking-[0.4em] text-zinc-600">Storage</h1>
            </div>
            <h2 className="text-xs text-zinc-500 font-bold tracking-widest uppercase">Archives</h2>
          </header>

          <div className="flex-1 overflow-y-auto space-y-1 pr-2 custom-scrollbar">
            <AnimatePresence mode="popLayout">
              {scripts.length > 0 ? (
                scripts.map((script) => (
                  <motion.div
                    key={script.id}
                    layout
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    className={`group flex items-center justify-between px-3 py-3 cursor-pointer border-l-2 transition-all ${
                      activeScriptId === script.id 
                      ? "border-white bg-white/5 text-white" 
                      : "border-transparent text-zinc-600 hover:text-zinc-300 hover:bg-zinc-900/50"
                    }`}
                    onClick={() => {
                      setCode(script.content);
                      setActiveScriptId(script.id);
                    }}
                  >
                    <span className="text-[11px] uppercase tracking-tighter font-medium truncate pr-4">
                      {script.name}
                    </span>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        removeScript(script.id);
                      }}
                      className="opacity-0 group-hover:opacity-100 p-1 hover:text-red-500 transition-all"
                    >
                      <Trash2 size={10} />
                    </button>
                  </motion.div>
                ))
              ) : (
                <div className="py-10 text-[10px] uppercase tracking-[0.2em] text-zinc-800 text-center border border-dashed border-zinc-900">
                  Empty_Archive
                </div>
              )}
            </AnimatePresence>
          </div>
        </div>

        <div className="pt-8 border-t border-zinc-900 mt-4 opacity-30">
          <p className="text-[8px] uppercase tracking-[0.3em] font-mono">Kernel_Filesystem_Ready</p>
        </div>
      </div>

      {/* RIGHT: EDITOR (WORKSPACE) */}
      <div className="flex-1 flex flex-col overflow-hidden bg-[#050505]">
        
        {/* Workspace Toolbar */}
        <div className="h-16 border-b border-zinc-900 flex items-center justify-between px-8 bg-[#09090b]/50 backdrop-blur-md">
          <div className="flex items-center gap-4 text-zinc-600">
            <CornerDownRight size={14} />
            <span className="text-[10px] font-mono uppercase tracking-widest">
              {activeScriptId ? "active_buffer" : "scratchpad"}
            </span>
          </div>

          <div className="flex items-center gap-6">
            <button
              onClick={saveScript}
              className="flex items-center gap-2 text-[10px] font-bold uppercase tracking-widest text-zinc-500 hover:text-white transition-colors"
            >
              <Save size={12} />
              Commit
            </button>
            <button className="flex items-center gap-2 px-6 py-2 bg-white text-black text-[10px] font-black uppercase tracking-[0.2em] hover:bg-zinc-200 transition-all">
              <Play size={10} fill="currentColor" />
              Execute
            </button>
          </div>
        </div>

        {/* Text Area Container */}
        <div className="flex-1 relative p-8 md:p-12 overflow-hidden">
          <div className="absolute left-10 top-12 bottom-12 w-[1px] bg-zinc-900" />
          <textarea
            value={code}
            onChange={(e) => setCode(e.target.value)}
            spellCheck={false}
            className="w-full h-full bg-transparent pl-10 font-mono text-sm leading-[2] text-zinc-500 focus:text-zinc-200 transition-all outline-none resize-none overflow-y-auto custom-scrollbar"
            placeholder="// Begin logic sequence..."
          />
        </div>

        {/* Status Bar */}
        <footer className="h-8 border-t border-zinc-900 flex items-center px-8 justify-between text-[9px] uppercase tracking-widest text-zinc-700 font-bold">
           <div className="flex gap-6">
              <span>Lines: {code.split('\n').length}</span>
              <span>Chars: {code.length}</span>
           </div>
           <span>UTF-8 // JSON_STRICT</span>
        </footer>
      </div>

      <style jsx global>{`
        .custom-scrollbar::-webkit-scrollbar { width: 3px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: #18181b; }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: #27272a; }
      `}</style>
    </div>
  );
}