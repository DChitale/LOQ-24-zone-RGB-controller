"use client";
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';
import { Settings as SettingsIcon, Zap, Shield, Info, Activity, Clock } from 'lucide-react';

interface AppSettings {
    auto_fix_on_startup: boolean;
    startup_delay_seconds: number;
    fix_on_app_launch: boolean;
    brightness_level?: number;
}



export default function SettingsPage() {
    const [settings, setSettings] = useState<AppSettings>({
        auto_fix_on_startup: false,
        startup_delay_seconds: 60,
        fix_on_app_launch: true
    });
    const [isStartupInstalled, setIsStartupInstalled] = useState(false);
    const [loading, setLoading] = useState(false);
    const [refreshing, setRefreshing] = useState(false);
    const [saveStatus, setSaveStatus] = useState('');
    const [refreshStatus, setRefreshStatus] = useState('');

    useEffect(() => {
        loadSettings();
        checkStartupStatus();
    }, []);

    async function loadSettings() {
        try {
            const loadedSettings = await invoke<AppSettings>('get_settings');
            setSettings(loadedSettings);
        } catch (error) {
            console.error('Failed to load settings:', error);
        }
    }

    async function checkStartupStatus() {
        const installed = await invoke<boolean>('check_startup_installed');
        setIsStartupInstalled(installed);
    }

    async function saveSettings() {
        setLoading(true);
        setSaveStatus('PROCESSING...');
        
        try {
            await invoke('save_settings', { settings });
            
            if (settings.auto_fix_on_startup) {
                await invoke('install_startup_task', { 
                    delaySeconds: settings.startup_delay_seconds 
                });
                setSaveStatus('✓ CONFIGURATION_APPLIED');
            } else {
                if (isStartupInstalled) {
                    await invoke('uninstall_startup_task');
                }
                setSaveStatus('✓ SETTINGS_SAVED');
            }
            
            await checkStartupStatus();
            setTimeout(() => setSaveStatus(''), 3000);
        } catch (error) {
            setSaveStatus('✗ ERROR: ' + error);
        }
        
        setLoading(false);
    }

    async function setLightingNow() {
        setRefreshing(true);
        setRefreshStatus('EXECUTING...');
        
        try {
            const result = await invoke<string>('set_lighting_priority');
            setRefreshStatus('✓ CONTROL_ACQUIRED');
            setTimeout(() => setRefreshStatus(''), 3000);
        } catch (error) {
            setRefreshStatus('✗ FAILURE: ' + error);
            setTimeout(() => setRefreshStatus(''), 5000);
        }
        
        setRefreshing(false);
    }

    return (
        <div className="flex flex-col md:flex-row h-screen bg-[#050505] text-zinc-400 font-sans selection:bg-zinc-800">
            
            {/* LEFT: NAVIGATION */}
            <div className="w-full md:w-72 border-r border-zinc-900 p-10 flex flex-col justify-between bg-[#080808]/30">
                <div className="space-y-12">
                    <header className="space-y-2">
                        <div className="w-4 h-[2px] bg-zinc-500" />
                        <h2 className="text-[10px] font-black tracking-[0.4em] uppercase text-zinc-600">
                            Lighting_Control
                        </h2>
                    </header>

                    <nav className="flex flex-col gap-1">
                        <button className="text-left px-4 py-3 text-[11px] uppercase tracking-widest text-white border-l border-white">
                            Control
                        </button>
                        <button className="text-left px-4 py-3 text-[11px] uppercase tracking-widest text-zinc-600 hover:text-zinc-300 transition-all">
                            Automation
                        </button>
                        <button className="text-left px-4 py-3 text-[11px] uppercase tracking-widest text-zinc-600 hover:text-zinc-300 transition-all">
                            Status
                        </button>
                    </nav>
                </div>

                <div className="space-y-4 opacity-40">
                    <div className="flex items-center gap-2">
                        <Info size={10} />
                        <span className="text-[8px] uppercase tracking-widest font-mono">
                            v1.0.0-STABLE
                        </span>
                    </div>
                </div>
            </div>

            {/* RIGHT: MAIN WORKSPACE */}
            <div className="flex-1 overflow-y-auto px-10 md:px-24 py-20">
                <div className="max-w-2xl space-y-20">
                    
                    {/* Header */}
                    <section className="space-y-4">
                        <h1 className="text-4xl font-extralight tracking-tighter text-white italic">
                            Dynamic Lighting
                        </h1>
                        <p className="text-[10px] text-zinc-500 uppercase tracking-widest font-medium">
                            Windows controller priority management system
                        </p>
                    </section>

                    {/* Status Messages */}
                    {(refreshStatus || saveStatus) && (
                        <div className="space-y-2">
                            {refreshStatus && (
                                <div className={`px-4 py-2 border ${refreshStatus.startsWith('✓') ? 'border-white/20 text-white' : 'border-red-900 text-red-400'}`}>
                                    <span className="text-[10px] font-mono tracking-wider">{refreshStatus}</span>
                                </div>
                            )}
                            {saveStatus && (
                                <div className={`px-4 py-2 border ${saveStatus.startsWith('✓') ? 'border-white/20 text-white' : 'border-red-900 text-red-400'}`}>
                                    <span className="text-[10px] font-mono tracking-wider">{saveStatus}</span>
                                </div>
                            )}
                        </div>
                    )}

                    {/* Quick Action */}
                    <section className="p-8 border border-zinc-800 bg-zinc-900/20 space-y-6">
                        <div className="space-y-2">
                            <h3 className="text-[10px] font-bold text-zinc-500 uppercase tracking-[0.3em]">
                                Manual_Override
                            </h3>
                            <p className="text-[11px] text-zinc-600 leading-relaxed">
                                Execute immediate controller priority swap to Windows default
                            </p>
                        </div>
                        
                        <button 
                            onClick={setLightingNow}
                            disabled={refreshing}
                            className="w-full px-6 py-4 bg-white text-black text-[10px] font-black uppercase tracking-[0.3em] hover:bg-zinc-200 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                        >
                            {refreshing ? (
                                <span className="flex items-center justify-center gap-2">
                                    <Activity size={12} className="animate-spin" />
                                    EXECUTING...
                                </span>
                            ) : (
                                'TAKE_CONTROL_NOW'
                            )}
                        </button>
                    </section>

                    {/* Automation Settings */}
                    <section className="space-y-10">
                        <div className="flex items-center gap-3">
                            <Zap size={14} className="text-zinc-700" />
                            <h3 className="text-[10px] font-bold text-zinc-500 uppercase tracking-[0.3em]">
                                Automation_Parameters
                            </h3>
                        </div>

                        <div className="space-y-2">
                            {/* Auto-fix on startup */}
                            <div className="flex items-center justify-between py-6 border-b border-zinc-900 group">
                                <div>
                                    <h4 className="text-sm font-light text-zinc-200 uppercase tracking-tight">
                                        Auto-Fix on System Startup
                                    </h4>
                                    <p className="text-[10px] text-zinc-600 mt-1">
                                        Execute priority swap after login sequence
                                    </p>
                                </div>
                                <button 
                                    onClick={() => setSettings({
                                        ...settings,
                                        auto_fix_on_startup: !settings.auto_fix_on_startup
                                    })}
                                    className={`w-10 h-5 flex items-center px-1 transition-colors duration-300 ${
                                        settings.auto_fix_on_startup ? 'bg-white' : 'bg-zinc-800'
                                    }`}
                                >
                                    <div className={`w-3 h-3 transition-transform duration-300 ${
                                        settings.auto_fix_on_startup ? 'translate-x-5 bg-black' : 'translate-x-0 bg-zinc-500'
                                    }`} />
                                </button>
                            </div>

                            {/* Startup Delay */}
                            {settings.auto_fix_on_startup && (
                                <div className="py-6 border-b border-zinc-900 space-y-4">
                                    <div className="flex items-center justify-between">
                                        <div>
                                            <h4 className="text-sm font-light text-zinc-200 uppercase tracking-tight">
                                                Execution Delay
                                            </h4>
                                            <p className="text-[10px] text-zinc-600 mt-1">
                                                Wait period before override activation
                                            </p>
                                        </div>
                                        <div className="flex items-center gap-3">
                                            <Clock size={12} className="text-zinc-700" />
                                            <span className="text-xl font-light text-white tabular-nums">
                                                {settings.startup_delay_seconds}s
                                            </span>
                                        </div>
                                    </div>
                                    
                                    <div className="space-y-2">
                                        <input 
                                            type="range"
                                            min="30"
                                            max="300"
                                            step="15"
                                            value={settings.startup_delay_seconds}
                                            onChange={(e) => setSettings({
                                                ...settings,
                                                startup_delay_seconds: parseInt(e.target.value)
                                            })}
                                            className="w-full h-[2px] bg-zinc-800 appearance-none cursor-pointer
                                                [&::-webkit-slider-thumb]:appearance-none 
                                                [&::-webkit-slider-thumb]:w-3 
                                                [&::-webkit-slider-thumb]:h-3 
                                                [&::-webkit-slider-thumb]:bg-white 
                                                [&::-webkit-slider-thumb]:cursor-pointer
                                                [&::-moz-range-thumb]:w-3
                                                [&::-moz-range-thumb]:h-3
                                                [&::-moz-range-thumb]:bg-white
                                                [&::-moz-range-thumb]:border-0
                                                [&::-moz-range-thumb]:cursor-pointer"
                                        />
                                        <div className="flex justify-between text-[8px] text-zinc-700 uppercase tracking-widest font-mono">
                                            <span>30s</span>
                                            <span>Recommended: 60-90s</span>
                                            <span>300s</span>
                                        </div>
                                    </div>


                                </div>
                            )}

                            {/* Fix on app launch */}
                            <div className="flex items-center justify-between py-6 border-b border-zinc-900 group">
                                <div>
                                    <h4 className="text-sm font-light text-zinc-200 uppercase tracking-tight">
                                        Fix on Application Launch
                                    </h4>
                                    <p className="text-[10px] text-zinc-600 mt-1">
                                        Apply override when control panel initializes
                                    </p>
                                </div>
                                <button 
                                    onClick={() => setSettings({
                                        ...settings,
                                        fix_on_app_launch: !settings.fix_on_app_launch
                                    })}
                                    className={`w-10 h-5 flex items-center px-1 transition-colors duration-300 ${
                                        settings.fix_on_app_launch ? 'bg-white' : 'bg-zinc-800'
                                    }`}
                                >
                                    <div className={`w-3 h-3 transition-transform duration-300 ${
                                        settings.fix_on_app_launch ? 'translate-x-5 bg-black' : 'translate-x-0 bg-zinc-500'
                                    }`} />
                                </button>
                            </div>
                        </div>
                    </section>

                    {/* Save Configuration */}
                    <section className="pt-6">
                        <button 
                            onClick={saveSettings}
                            disabled={loading}
                            className="w-full px-6 py-4 border border-white text-white text-[10px] font-black uppercase tracking-[0.3em] hover:bg-white hover:text-black disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                        >
                            {loading ? 'PROCESSING...' : 'APPLY_CONFIGURATION'}
                        </button>
                    </section>

                    {/* System Status */}
                    <section className="space-y-10">
                        <div className="flex items-center gap-3">
                            <Activity size={14} className="text-zinc-700" />
                            <h3 className="text-[10px] font-bold text-zinc-500 uppercase tracking-[0.3em]">
                                System_State
                            </h3>
                        </div>

                        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                            <div className="p-6 border border-zinc-900 space-y-4">
                                <span className="text-[9px] text-zinc-700 uppercase tracking-widest font-bold">
                                    Scheduled Task
                                </span>
                                <div className={`text-xl font-light tabular-nums ${
                                    isStartupInstalled ? 'text-white' : 'text-zinc-600'
                                }`}>
                                    {isStartupInstalled ? 'ACTIVE' : 'INACTIVE'}
                                </div>
                                <div className={`text-[8px] uppercase tracking-widest ${
                                    isStartupInstalled ? 'text-zinc-500' : 'text-zinc-700'
                                }`}>
                                    {isStartupInstalled ? '✓ INSTALLED' : '○ NOT_CONFIGURED'}
                                </div>
                            </div>

                            <div className="p-6 border border-zinc-900 space-y-4">
                                <span className="text-[9px] text-zinc-700 uppercase tracking-widest font-bold">
                                    Auto-Fix Status
                                </span>
                                <div className={`text-xl font-light tabular-nums ${
                                    settings.auto_fix_on_startup ? 'text-white' : 'text-zinc-600'
                                }`}>
                                    {settings.auto_fix_on_startup ? 'ENABLED' : 'DISABLED'}
                                </div>
                                <div className={`text-[8px] uppercase tracking-widest ${
                                    settings.auto_fix_on_startup ? 'text-zinc-500' : 'text-zinc-700'
                                }`}>
                                    {settings.auto_fix_on_startup ? `DELAY: ${settings.startup_delay_seconds}s` : 'MANUAL_MODE'}
                                </div>
                            </div>

                            <div className="p-6 border border-zinc-900 space-y-4">
                                <span className="text-[9px] text-zinc-700 uppercase tracking-widest font-bold">
                                    Launch Behavior
                                </span>
                                <div className={`text-xl font-light tabular-nums ${
                                    settings.fix_on_app_launch ? 'text-white' : 'text-zinc-600'
                                }`}>
                                    {settings.fix_on_app_launch ? 'AUTO' : 'MANUAL'}
                                </div>
                                <div className={`text-[8px] uppercase tracking-widest ${
                                    settings.fix_on_app_launch ? 'text-zinc-500' : 'text-zinc-700'
                                }`}>
                                    {settings.fix_on_app_launch ? '✓ APPLY_ON_START' : '○ USER_TRIGGERED'}
                                </div>
                            </div>

                            <div className="p-6 border border-zinc-900 space-y-4">
                                <span className="text-[9px] text-zinc-700 uppercase tracking-widest font-bold">
                                    Controller
                                </span>
                                <div className="text-xl font-light text-white tabular-nums">
                                    WINDOWS
                                </div>
                                <div className="text-[8px] uppercase tracking-widest text-zinc-500">
                                    ✓ PRIORITY_TARGET
                                </div>
                            </div>
                        </div>
                    </section>

                    {/* Footer State */}
                    <footer className="pt-10 flex items-center justify-between border-t border-zinc-900 opacity-20 group hover:opacity-100 transition-opacity">
                        <div className="flex items-center gap-2">
                            <Shield size={12} />
                            <span className="text-[9px] uppercase tracking-[0.3em]">
                                Registry_Access_Secure
                            </span>
                        </div>
                        <span className="text-[9px] font-mono tracking-tighter">
                            {new Date().toLocaleTimeString('en-US', { hour12: false })}_READY
                        </span>
                    </footer>

                </div>
            </div>

        </div>
    );
}