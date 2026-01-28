"use client";
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';

interface ParameterConfig {
  name: string;
  label: string;
  param_type: ParameterType;
  min: number;
  max: number;
  default: number;
  step: number;
}

interface ParameterType {
  type: string;
  [key: string]: any; // For Color type: r, g, b properties
}

interface PresetMetadata {
  name: string;
  display_name: string;
  description: string;
  parameters: ParameterConfig[];
}

interface ParameterValue {
  type: string;
  value: number | { r: number; g: number; b: number };
}

interface PresetConfig {
  name: string;
  parameters: { [key: string]: ParameterValue };
}

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
      console.log('Loaded presets:', presetData);
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

    console.log('Selecting preset:', presetName, preset);

    // Initialize parameter values with defaults
    const initialValues: { [key: string]: any } = {};
    preset.parameters.forEach(param => {
      console.log('Processing parameter:', param);
      if (param.param_type.type === 'Float') {
        initialValues[param.name] = param.default;
      } else if (param.param_type.type === 'Color') {
        // For color parameters, use the default color from the param_type
        const colorData = param.param_type as any;
        initialValues[param.name] = { 
          r: colorData.r || 255, 
          g: colorData.g || 0, 
          b: colorData.b || 0 
        };
        console.log('Color parameter initialized:', param.name, initialValues[param.name]);
      }
    });

    console.log('Initial parameter values:', initialValues);
    setParameterValues(initialValues);
    setCurrentPreset(presetName);

    // Create preset config
    const config: PresetConfig = {
      name: presetName,
      parameters: {}
    };

    // Convert parameter values to ParameterValue format
    Object.entries(initialValues).forEach(([key, value]) => {
      if (typeof value === 'number') {
        config.parameters[key] = { type: 'Float', value };
      } else if (typeof value === 'object' && 'r' in value) {
        config.parameters[key] = { type: 'Color', value };
      }
    });

    try {
      await invoke('set_preset', { 
        presetName: presetName,
        parameters: config.parameters
      });
    } catch (error) {
      console.error('Failed to set preset:', error);
    }
  };

  const updateParameter = async (paramName: string, value: any) => {
    console.log('Updating parameter:', paramName, value);
    const newValues = { ...parameterValues, [paramName]: value };
    setParameterValues(newValues);

    // Convert to ParameterValue format
    let paramValue: ParameterValue;
    if (typeof value === 'number') {
      paramValue = { type: 'Float', value };
    } else if (typeof value === 'object' && 'r' in value) {
      paramValue = { type: 'Color', value };
    } else {
      return;
    }

    try {
      console.log('Sending parameter update to backend:', { presetName: currentPreset, paramName, value: paramValue });
      await invoke('adjust_preset_parameter', {
        presetName: currentPreset,
        paramName: paramName,
        value: paramValue
      });
      console.log('Parameter update sent successfully');
    } catch (error) {
      console.error('Failed to adjust parameter:', error);
    }
  };

  const renderParameterControl = (param: ParameterConfig) => {
    const currentValue = parameterValues[param.name];
    console.log('Rendering parameter control:', param.name, 'current value:', currentValue, 'all values:', parameterValues);

    if (param.param_type.type === 'Float') {
      return (
        <div key={param.name} className="space-y-2">
          <label className="text-sm font-medium text-zinc-300">{param.label}</label>
          <input
            type="range"
            min={param.min}
            max={param.max}
            step={param.step}
            value={currentValue !== undefined ? currentValue : param.default}
            onChange={(e) => {
              console.log('Range input changed:', param.name, e.target.value);
              updateParameter(param.name, parseFloat(e.target.value));
            }}
            className="w-full h-2 bg-zinc-700 rounded-lg appearance-none cursor-pointer slider"
          />
          <div className="text-xs text-zinc-500 text-center">
            {(currentValue !== undefined ? currentValue : param.default)?.toFixed(2)}
          </div>
        </div>
      );
    } else if (param.param_type.type === 'Color') {
      const colorValue = currentValue ? `#${currentValue.r.toString(16).padStart(2, '0')}${currentValue.g.toString(16).padStart(2, '0')}${currentValue.b.toString(16).padStart(2, '0')}` : '#ff0000';
      console.log('Color value for', param.name, ':', colorValue);
      return (
        <div key={param.name} className="space-y-2">
          <label className="text-sm font-medium text-zinc-300">{param.label}</label>
          <input
            type="color"
            value={colorValue}
            onChange={(e) => {
              console.log('Color input changed:', param.name, e.target.value);
              const hex = e.target.value;
              const r = parseInt(hex.slice(1, 3), 16);
              const g = parseInt(hex.slice(3, 5), 16);
              const b = parseInt(hex.slice(5, 7), 16);
              updateParameter(param.name, { r, g, b });
            }}
            className="w-full h-10 bg-zinc-700 border border-zinc-600 rounded cursor-pointer"
          />
        </div>
      );
    }
    return null;
  };

  if (loading) {
    return (
      <div className="w-full max-w-lg p-6 bg-zinc-950/50 rounded-3xl border border-zinc-900">
        <div className="text-center text-zinc-400">Loading presets...</div>
      </div>
    );
  }

  const currentPresetData = presets.find(p => p.name === currentPreset);
  console.log('Current preset:', currentPreset, 'Data:', currentPresetData);
  console.log('Current parameter values:', parameterValues);

  return (
    <div className="w-full max-w-lg p-6 bg-zinc-950/50 rounded-3xl border border-zinc-900 animate-in fade-in duration-500">
      <div className="flex items-center gap-4 mb-6">
        <div className="text-zinc-500 text-sm">🎨</div>
        <h2 className="text-sm font-bold tracking-[0.4em] uppercase">Effect Presets</h2>
      </div>

      {/* Preset Selection */}
      <div className="space-y-4 mb-6">
        <label className="text-sm font-medium text-zinc-300">Select Preset</label>
        <div className="grid grid-cols-2 gap-2">
          {presets.map((preset) => (
            <button
              key={preset.name}
              onClick={() => selectPreset(preset.name)}
              className={`p-3 rounded-xl text-xs font-medium transition-all ${
                currentPreset === preset.name
                  ? 'bg-white text-black'
                  : 'bg-zinc-800 text-zinc-300 hover:bg-zinc-700'
              }`}
            >
              {preset.display_name}
            </button>
          ))}
        </div>
      </div>

      {/* Parameter Controls */}
      {currentPresetData && currentPresetData.parameters.length > 0 && (
        <div className="space-y-4">
          <div className="text-sm text-zinc-400 mb-2">
            {currentPresetData.description}
          </div>
          {currentPresetData.parameters.map(renderParameterControl)}
        </div>
      )}

      {/* No parameters message */}
      {currentPresetData && currentPresetData.parameters.length === 0 && (
        <div className="text-center text-zinc-500 text-sm py-4">
          No adjustable parameters for this preset
        </div>
      )}

      {/* Debug info */}
      {currentPresetData && (
        <div className="text-xs text-zinc-600 mt-4 p-2 bg-zinc-800 rounded">
          Debug: {currentPresetData.parameters.length} parameters found
        </div>
      )}
    </div>
  );
}