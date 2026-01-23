"use client";

import { useEffect, useMemo, useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Terminal,
  Trash2,
  CornerDownRight,
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

const generateVersionedName = (base: string, existing: ScriptEntry[]) => {
  let index = 1;
  let candidate = `${base} (${index})`;

  while (existing.some(s => s.name === candidate)) {
    index++;
    candidate = `${base} (${index})`;
  }

  return candidate;
};

/* ---------------------------------------------
 * Component
 * -------------------------------------------- */

export default function CinematicConsole() {
  const [code, setCode] = useState(`// LOQ core_init.json
{
  "sequence": "intercept",
  "zones": [1, 24],
  "rgb": [255, 255, 255],
  "pulse": true
}`);
  const [scripts, setScripts] = useState<ScriptEntry[]>([]);
  const [isFocused, setIsFocused] = useState(false);

  /* ---------------------------------------------
   * Initialization
   * -------------------------------------------- */

  useEffect(() => {
    setScripts(loadScripts());
  }, []);

  /* ---------------------------------------------
   * Actions
   * -------------------------------------------- */

  const commitScript = () => {
    let name = prompt("Enter Identifier:");
    if (!name) return;

    const normalized = name.toLowerCase();
    const existingIndex = scripts.findIndex(
      s => s.name.toLowerCase() === normalized
    );

    let nextScripts = [...scripts];

    if (existingIndex !== -1) {
      const overwrite = confirm(
        `"${name}" already exists.\nOverwrite existing version?`
      );

      if (overwrite) {
        nextScripts[existingIndex] = {
          ...nextScripts[existingIndex],
          content: code,
        };
      } else {
        name = generateVersionedName(name, scripts);
        nextScripts.unshift({
          id: Date.now(),
          name,
          content: code,
        });
      }
    } else {
      nextScripts.unshift({
        id: Date.now(),
        name,
        content: code,
      });
    }

    setScripts(nextScripts);
    persistScripts(nextScripts);
  };

  const removeScript = (id: number) => {
    const next = scripts.filter(s => s.id !== id);
    setScripts(next);
    persistScripts(next);
  };

  const loadScript = (content: string) => {
    setCode(content);
  };

  /* ---------------------------------------------
   * Render
   * -------------------------------------------- */

  return (
    <div className="h-full w-full bg-[#020202] p-8 lg:p-16 flex flex-col gap-16 max-w-[1300px] mx-auto selection:bg-white selection:text-black">

      {/* Header */}
      <header className="flex justify-between items-center group">
        <div className="space-y-3">
          <div className="flex items-center gap-3 text-zinc-800 group-hover:text-zinc-500 transition-colors duration-700">
            <Terminal size={14} className="animate-pulse" />
            <span className="text-[10px] uppercase tracking-[0.6em] font-black">
              Core.Terminal.Execute
            </span>
          </div>

          <h1 className="text-3xl font-thin tracking-[0.3em] text-white uppercase leading-none">
            Script{" "}
            <span className="text-zinc-800 group-hover:text-white transition-all duration-1000">
              Engine
            </span>
          </h1>
        </div>

        <div className="flex items-center gap-10">
          <button
            onClick={commitScript}
            className="text-[11px] uppercase tracking-[0.3em] text-zinc-600 hover:text-white transition-all duration-500 hover:scale-110 active:scale-95"
          >
            Commit
          </button>

          <div className="h-10 w-px bg-zinc-900" />

          <button className="relative group/btn overflow-hidden px-10 py-3 bg-white text-black text-[11px] uppercase tracking-[0.3em] font-black transition-all hover:invert">
            <span className="relative z-10">Execute</span>
            <motion.div className="absolute inset-0 bg-zinc-200 translate-y-full group-hover/btn:translate-y-0 transition-transform duration-300" />
          </button>
        </div>
      </header>

      {/* Grid */}
      <div className="flex-1 grid grid-cols-1 lg:grid-cols-4 gap-20 min-h-0">

        {/* Editor */}
        <div className="lg:col-span-3 flex flex-col relative">
          <div className="flex items-center gap-4 mb-6 text-zinc-800">
            <CornerDownRight size={18} />
            <span className="text-[11px] font-mono tracking-tighter lowercase opacity-50">
              buffer://loq_logic.json
            </span>
          </div>

          <div className="relative flex-1">
            <motion.div
              animate={{
                height: isFocused ? "100%" : "20%",
                backgroundColor: isFocused ? "#fff" : "#18181b",
                boxShadow: isFocused
                  ? "0 0 20px #fff"
                  : "0 0 0px #000",
              }}
              className="absolute left-0 top-0 w-[1px] transition-all duration-700"
            />

            <textarea
              value={code}
              onChange={e => setCode(e.target.value)}
              onFocus={() => setIsFocused(true)}
              onBlur={() => setIsFocused(false)}
              spellCheck={false}
              className="w-full h-full bg-transparent pl-12 font-mono text-sm leading-[2] text-zinc-500 focus:text-white transition-all duration-700 outline-none resize-none overflow-y-auto custom-scrollbar"
            />
          </div>
        </div>

        {/* Archives */}
        <aside className="lg:col-span-1 flex flex-col gap-10 border-l border-zinc-900/50 pl-10">
          <h3 className="text-[10px] uppercase tracking-[0.5em] text-zinc-800 font-bold">
            Archives
          </h3>

          <div className="flex-1 overflow-y-auto space-y-6 pr-4 custom-scrollbar">
            <AnimatePresence mode="popLayout">
              {scripts.map(script => (
                <motion.div
                  key={script.id}
                  layout
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, x: -20 }}
                  className="group cursor-pointer"
                  onClick={() => loadScript(script.content)}
                >
                  <div className="flex items-center justify-between">
                    <span className="text-[12px] text-zinc-600 group-hover:text-white transition-all duration-500 tracking-tight font-light lowercase">
                      {script.name}
                    </span>

                    <button
                      onClick={e => {
                        e.stopPropagation();
                        removeScript(script.id);
                      }}
                      className="opacity-0 group-hover:opacity-100 transition-all duration-500"
                    >
                      <Trash2 size={12} className="text-zinc-900 hover:text-red-900" />
                    </button>
                  </div>

                  <div className="mt-2 h-[1px] w-0 group-hover:w-full bg-zinc-800 transition-all duration-700" />
                </motion.div>
              ))}
            </AnimatePresence>
          </div>
        </aside>
      </div>
    </div>
  );
}
