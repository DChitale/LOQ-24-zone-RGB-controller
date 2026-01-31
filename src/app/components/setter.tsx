"use client";
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect, useRef } from 'react';

// ... (Interfaces remain untouched)
interface ParameterConfig { name: string; label: string; param_type: ParameterType; min: number; max: number; default: number; step: number; }
interface ParameterType { type: string; [key: string]: any; }
interface PresetMetadata { name: string; display_name: string; description: string; parameters: ParameterConfig[]; }
interface ParameterValue { type: string; value: number | { r: number; g: number; b: number }; }
interface PresetConfig { name: string; parameters: { [key: string]: ParameterValue }; }

export default function PresetControls() {
  const [presets, setPresets] = useState<PresetMetadata[]>([]);
  const [currentPreset, setCurrentPreset] = useState<string>('');
  const [parameterValues, setParameterValues] = useState<{ [key: string]: any }>({});
  const [loading, setLoading] = useState(true);

  useEffect(() => { loadPresets(); }, []);

  // --- persistence keys --------------------------------------------------
  const LS_PRESET_KEY = 'loq.currentPreset';
  const LS_PARAMS_KEY = 'loq.parameterValues';

  const loadPresets = async () => {
    try {
      const presetData: PresetMetadata[] = await invoke('get_preset_metadata');
      setPresets(presetData);
      setLoading(false);
    } catch (error) {
      console.error('Failed to load presets:', error);
      setLoading(false);
    }
  };

  // Restore saved preset + parameter values (once metadata is available)
  useEffect(() => {
    if (!presets || presets.length === 0) return;

    (async () => {
      if (typeof window === 'undefined') return;
      try {
        const savedPreset = localStorage.getItem(LS_PRESET_KEY);
        const savedParams = localStorage.getItem(LS_PARAMS_KEY);
        if (!savedPreset) return;

        const found = presets.find(p => p.name === savedPreset);
        if (!found) return; // saved preset no longer exists in metadata

        // If we have saved parameter values, parse them and apply them on top of defaults.
        const parsedParams = savedParams ? JSON.parse(savedParams) : null;

        // Use selectPreset to initialise defaults and notify backend, then overwrite with saved values
        await selectPreset(savedPreset);

        if (parsedParams) {
          // apply saved UI values locally
          setParameterValues(parsedParams);

          // build payload in the same shape `set_preset` expects and re-apply to backend
          const payload: { [k: string]: ParameterValue } = {};
          Object.entries(parsedParams).forEach(([key, val]) => {
            // guard against null/invalid shapes from localStorage
            if (val == null) return;
            if (typeof val === 'number') {
              payload[key] = { type: 'Float', value: val };
              return;
            }
            const maybeColor = val as Partial<{ r: unknown; g: unknown; b: unknown }>;
            if (typeof maybeColor.r === 'number' && typeof maybeColor.g === 'number' && typeof maybeColor.b === 'number') {
              payload[key] = { type: 'Color', value: { r: maybeColor.r, g: maybeColor.g, b: maybeColor.b } };
            }
          });

          try {
            await invoke('set_preset', { presetName: savedPreset, parameters: payload });
          } catch (err) {
            console.warn('Failed to re-apply saved preset to backend:', err);
          }
        }
      } catch (err) {
        console.warn('Failed to restore saved preset from localStorage', err);
      }
    })();
  }, [presets]);

  // Persist current preset + parameter values (debounced to avoid floods while dragging)
  const saveTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  useEffect(() => {
    if (typeof window === 'undefined') return;
    if (saveTimer.current) clearTimeout(saveTimer.current);
    saveTimer.current = setTimeout(() => {
      try {
        if (currentPreset) localStorage.setItem(LS_PRESET_KEY, currentPreset);
        localStorage.setItem(LS_PARAMS_KEY, JSON.stringify(parameterValues || {}));
      } catch (e) {
        console.warn('localStorage write failed', e);
      }
    }, 150);
    return () => { if (saveTimer.current) clearTimeout(saveTimer.current); };
  }, [currentPreset, parameterValues]);

  const selectPreset = async (presetName: string) => {
    const preset = presets.find(p => p.name === presetName);
    if (!preset) return;
    const initialValues: { [key: string]: any } = {};
    preset.parameters.forEach(param => {
      if (param.param_type.type === 'Float') initialValues[param.name] = param.default;
      else if (param.param_type.type === 'Color') {
        const colorData = param.param_type as any;
        initialValues[param.name] = { r: colorData.r || 255, g: colorData.g || 0, b: colorData.b || 0 };
      }
    });
    setParameterValues(initialValues);
    setCurrentPreset(presetName);
    const config: PresetConfig = { name: presetName, parameters: {} };
    Object.entries(initialValues).forEach(([key, value]) => {
      if (typeof value === 'number') config.parameters[key] = { type: 'Float', value };
      else if (typeof value === 'object' && 'r' in value) config.parameters[key] = { type: 'Color', value };
    });
    try { await invoke('set_preset', { presetName, parameters: config.parameters }); } catch (error) { console.error(error); }
  };

  const updateParameter = async (paramName: string, value: any) => {
    const newValues = { ...parameterValues, [paramName]: value };
    setParameterValues(newValues);
    let paramValue: ParameterValue;
    if (typeof value === 'number') paramValue = { type: 'Float', value };
    else if (typeof value === 'object' && 'r' in value) paramValue = { type: 'Color', value };
    else return;
    try { await invoke('adjust_preset_parameter', { presetName: currentPreset, paramName, value: paramValue }); } catch (error) { console.error(error); }
  };

  const calculatePercentage = (val: number, min: number, max: number) => {
    if (max <= min) return 0; 
    const percent = ((val - min) / (max - min)) * 100;
    return Math.round(percent);
  };

  const renderParameterControl = (param: ParameterConfig) => {
    const currentValue = parameterValues[param.name] ?? param.default;
    
    if (param.param_type.type === 'Float') {
      const percent = calculatePercentage(currentValue, param.min, param.max);
      return (
        <div key={param.name} className="py-5 border-b border-zinc-900 group">
          <div className="flex justify-between items-baseline mb-4">
            <label className="text-[10px] font-bold text-zinc-500 uppercase tracking-[0.2em] group-hover:text-zinc-300 transition-colors">
              {param.label}
            </label>
            <span className="text-[10px] font-mono text-zinc-600 group-hover:text-white transition-colors tracking-tighter">
              {param.min === param.max ? 'FIXED' : `${percent}%`}
            </span>
          </div>
          <div className="relative flex items-center h-2">
            <div className="absolute w-full h-px bg-zinc-800" />
            <div className="absolute h-[1.5px] bg-white transition-all duration-75" style={{ width: `${percent}%` }} />
            <input
                type="range"
                min={param.min}
                max={param.max}
                step={param.step}
                value={currentValue}
                onChange={(e) => updateParameter(param.name, parseFloat(e.target.value))}
                className="absolute w-full h-full opacity-0 cursor-crosshair z-10"
            />
          </div>
        </div>
      );
    } else if (param.param_type.type === 'Color') {
      const colorValue = currentValue ? `#${currentValue.r.toString(16).padStart(2, '0')}${currentValue.g.toString(16).padStart(2, '0')}${currentValue.b.toString(16).padStart(2, '0')}` : '#ffffff';
      return (
        <div key={param.name} className="flex items-center justify-between py-5 border-b border-zinc-900 group">
          <label className="text-[10px] font-bold text-zinc-500 uppercase tracking-[0.2em] group-hover:text-zinc-300 transition-colors">{param.label}</label>
          <div className="flex items-center gap-3">
             <span className="text-[9px] font-mono text-zinc-700 uppercase">{colorValue}</span>
             <input
                type="color"
                value={colorValue}
                onChange={(e) => {
                    const hex = e.target.value;
                    const r = parseInt(hex.slice(1, 3), 16), g = parseInt(hex.slice(3, 5), 16), b = parseInt(hex.slice(5, 7), 16);
                    updateParameter(param.name, { r, g, b });
                }}
                className="w-4 h-4 bg-transparent border-none cursor-pointer"
            />
          </div>
        </div>
      );
    }
    return null;
  };

  if (loading) return (
    <div className="h-screen w-screen flex items-center justify-center bg-[#09090b]">
      <div className="w-4 h-4 border border-zinc-800 border-t-zinc-400 animate-spin rounded-full" />
    </div>
  );

  const currentPresetData = presets.find(p => p.name === currentPreset);

  return (
    <div className="flex flex-col md:flex-row  h-screen bg-[#09090b] text-zinc-400 font-sans">
      
      {/* LEFT: SELECTOR (SIDEBAR) */}
      <div className="w-full md:w-64 border-r border-zinc-900 p-8 flex flex-col">
        <div className="space-y-12">
          <div className="flex items-center gap-2">
            <div className="w-1.5 h-1.5 bg-zinc-600 rounded-full" />
            <h1 className="text-[10px] font-black uppercase tracking-[0.4em] text-zinc-600">Effects</h1>
          </div>
          
          <div className="space-y-3">
            <label className="text-[9px] text-zinc-700 uppercase font-bold tracking-widest">Library</label>
            <div className="relative">
              <select
                value={currentPreset}
                onChange={(e) => selectPreset(e.target.value)}
                className="w-full bg-transparent text-xs text-zinc-300 border-b border-zinc-800 py-2.5 focus:outline-none focus:border-zinc-500 transition-colors appearance-none cursor-pointer rounded-none"
              >
                <option value="" disabled className="bg-[#09090b]">Select Preset</option>
                {presets.map((p) => (
                  <option key={p.name} value={p.name} className="bg-[#09090b]">{p.display_name}</option>
                ))}
              </select>
              <div className="absolute right-0 top-1/2 -translate-y-1/2 pointer-events-none text-[8px] text-zinc-800">▼</div>
            </div>
          </div>
        </div>

        <div className="mt-auto pt-8">
           <div className="h-px w-full bg-zinc-900" />
           <p className="text-[8px] uppercase tracking-widest text-zinc-800 mt-4 font-mono">Status: Connected</p>
        </div>
      </div>

      {/* RIGHT: ADJUSTMENTS (WORKSPACE) */}
      <div className="flex-1 overflow-y-auto px-10 md:px-20 py-16">
        <div className="max-w-2xl">
          {currentPresetData ? (
            <div className="space-y-16">
              <header className="space-y-2">
                <h2 className="text-3xl font-light tracking-tight text-zinc-100 italic">{currentPresetData.display_name}</h2>
                <p className="text-[11px] text-zinc-600 uppercase tracking-widest font-medium">{currentPresetData.description}</p>
              </header>

              <div className="space-y-4">
                {currentPresetData.parameters.map(renderParameterControl)}
              </div>
            </div>
          ) : (
            <div className="h-full flex items-center justify-center opacity-10 select-none">
              <h1 className="text-8xl font-black tracking-tighter uppercase italic">Offline</h1>
            </div>
          )}
        </div>
      </div>

    </div>
  );
}