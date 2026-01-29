"use client";
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';

// ... (Interfaces remain identical to your original code)
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

  useEffect(() => {
    loadPresets();
  }, []);

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

  const selectPreset = async (presetName: string) => {
    const preset = presets.find(p => p.name === presetName);
    if (!preset) return;

    const initialValues: { [key: string]: any } = {};
    preset.parameters.forEach(param => {
      if (param.param_type.type === 'Float') {
        initialValues[param.name] = param.default;
      } else if (param.param_type.type === 'Color') {
        const colorData = param.param_type as any;
        initialValues[param.name] = { 
          r: colorData.r || 255, 
          g: colorData.g || 0, 
          b: colorData.b || 0 
        };
      }
    });

    setParameterValues(initialValues);
    setCurrentPreset(presetName);

    const config: PresetConfig = {
      name: presetName,
      parameters: {}
    };

    Object.entries(initialValues).forEach(([key, value]) => {
      if (typeof value === 'number') {
        config.parameters[key] = { type: 'Float', value };
      } else if (typeof value === 'object' && 'r' in value) {
        config.parameters[key] = { type: 'Color', value };
      }
    });

    try {
      await invoke('set_preset', { presetName, parameters: config.parameters });
    } catch (error) {
      console.error('Failed to set preset:', error);
    }
  };

  const updateParameter = async (paramName: string, value: any) => {
    const newValues = { ...parameterValues, [paramName]: value };
    setParameterValues(newValues);

    let paramValue: ParameterValue;
    if (typeof value === 'number') {
      paramValue = { type: 'Float', value };
    } else if (typeof value === 'object' && 'r' in value) {
      paramValue = { type: 'Color', value };
    } else {
      return;
    }

    try {
      await invoke('adjust_preset_parameter', {
        presetName: currentPreset,
        paramName,
        value: paramValue
      });
    } catch (error) {
      console.error('Failed to adjust parameter:', error);
    }
  };

  const renderParameterControl = (param: ParameterConfig) => {
    const currentValue = parameterValues[param.name];

    if (param.param_type.type === 'Float') {
      return (
        <div key={param.name} className="space-y-2">
          <div className="flex justify-between">
            <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider">{param.label}</label>
            <span className="text-xs text-zinc-500">{(currentValue ?? param.default).toFixed(2)}</span>
          </div>
          <input
            type="range"
            min={param.min}
            max={param.max}
            step={param.step}
            value={currentValue ?? param.default}
            onChange={(e) => updateParameter(param.name, parseFloat(e.target.value))}
            className="w-full h-1.5 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-white"
          />
        </div>
      );
    } else if (param.param_type.type === 'Color') {
      const colorValue = currentValue ? `#${currentValue.r.toString(16).padStart(2, '0')}${currentValue.g.toString(16).padStart(2, '0')}${currentValue.b.toString(16).padStart(2, '0')}` : '#ff0000';
      return (
        <div key={param.name} className="space-y-2">
          <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider">{param.label}</label>
          <input
            type="color"
            value={colorValue}
            onChange={(e) => {
              const hex = e.target.value;
              const r = parseInt(hex.slice(1, 3), 16);
              const g = parseInt(hex.slice(3, 5), 16);
              const b = parseInt(hex.slice(5, 7), 16);
              updateParameter(param.name, { r, g, b });
            }}
            className="w-full h-8 bg-zinc-800 border-none rounded cursor-pointer"
          />
        </div>
      );
    }
    return null;
  };

  if (loading) {
    return (
      <div className="w-full max-w-4xl p-12 text-center text-zinc-500 animate-pulse">
        Initializing Engine...
      </div>
    );
  }

  const currentPresetData = presets.find(p => p.name === currentPreset);

  return (
    <div className="flex flex-col md:flex-row gap-6 w-full max-w-5xl mx-auto p-4 overflow-y-auto min-h-screen">
      {/* LEFT COLUMN: PARAMETERS */}
      <div className="flex-1 min-h-[400px] p-6 bg-zinc-950/40 rounded-3xl border border-zinc-900 backdrop-blur-sm">
        <div className="flex items-center gap-3 mb-8">
          <div className="w-2 h-2 rounded-full bg-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.5)]" />
          <h2 className="text-xs font-bold tracking-[0.3em] uppercase text-zinc-100">Adjustments</h2>
        </div>

        {currentPresetData ? (
          <div className="space-y-8 animate-in fade-in slide-in-from-left-4 duration-500">
            <div>
              <h3 className="text-lg font-medium text-white">{currentPresetData.display_name}</h3>
              <p className="text-sm text-zinc-500 mt-1">{currentPresetData.description}</p>
            </div>
            
            <div className="space-y-6">
              {currentPresetData.parameters.length > 0 ? (
                currentPresetData.parameters.map(renderParameterControl)
              ) : (
                <div className="text-zinc-600 italic text-sm py-10 text-center border border-dashed border-zinc-800 rounded-2xl">
                  No adjustable parameters for this effect.
                </div>
              )}
            </div>
          </div>
        ) : (
          <div className="h-full flex flex-col items-center justify-center text-zinc-600 space-y-2">
            <span className="text-2xl">👈</span>
            <p className="text-sm">Select a preset to begin</p>
          </div>
        )}
      </div>

      {/* RIGHT COLUMN: PRESET SELECTION */}
      <div className="w-full md:w-80 space-y-4">
        <div className="p-6 bg-zinc-900/50 rounded-3xl border border-zinc-800">
          <label className="block text-[10px] font-bold text-zinc-500 uppercase tracking-[0.2em] mb-4">
            Master Library
          </label>
          
          <div className="relative">
            <select
              value={currentPreset}
              onChange={(e) => selectPreset(e.target.value)}
              className="w-full bg-zinc-800 text-zinc-100 text-sm rounded-xl px-4 py-3 appearance-none border border-zinc-700 focus:outline-none focus:ring-2 focus:ring-white/10 transition-all cursor-pointer"
            >
              <option value="" disabled>Choose an effect...</option>
              {presets.map((preset) => (
                <option key={preset.name} value={preset.name}>
                  {preset.display_name}
                </option>
              ))}
            </select>
            <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-zinc-500 text-xs">
              ▼
            </div>
          </div>

          <div className="mt-6 pt-6 border-t border-zinc-800/50">
             <div className="flex justify-between text-[10px] text-zinc-600 uppercase font-bold tracking-tighter">
                <span>Status</span>
                <span className="text-zinc-400">{currentPreset ? 'Active' : 'Standby'}</span>
             </div>
          </div>
        </div>
      </div>
    </div>
  );
}