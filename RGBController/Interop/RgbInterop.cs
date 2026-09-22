using System;
using System.Runtime.InteropServices;
using RGBController.Models;

namespace RGBController.Interop
{
    public static class RgbInterop
    {
        private const string DllName = "rgb_backend.dll";

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate void FrameCallback(IntPtr buffer, int len);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int rgb_init();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void rgb_shutdown();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void rgb_start_frame_callback(FrameCallback? callback);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern float rgb_get_brightness();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int rgb_set_brightness(float brightness);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int rgb_get_frame(IntPtr buffer, int len);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr rgb_get_preset_metadata();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr rgb_set_preset(
            [MarshalAs(UnmanagedType.LPUTF8Str)] string presetName,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string paramsJson);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr rgb_adjust_parameter(
            [MarshalAs(UnmanagedType.LPUTF8Str)] string presetName,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string paramName,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string valueJson);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int rgb_connect_keyboard(ushort vid, ushort pid);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int rgb_set_light_control_owner(int enable);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int rgb_enable_dynamic_lighting();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int rgb_disable_dynamic_lighting();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int rgb_is_dynamic_lighting_enabled();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr rgb_set_lighting_priority();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int rgb_check_startup_installed();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr rgb_install_startup_task(uint delaySeconds);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr rgb_uninstall_startup_task();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr rgb_get_settings();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr rgb_save_settings([MarshalAs(UnmanagedType.LPUTF8Str)] string settingsJson);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr rgb_cycle_preset();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void rgb_free_string(IntPtr ptr);

        // Helper wrappers to automatically convert native string pointers and free them
        public static string GetPresetMetadata() => MarshalStringAndFree(rgb_get_preset_metadata());
        public static string SetPreset(string presetName, string paramsJson) => MarshalStringAndFree(rgb_set_preset(presetName, paramsJson));
        public static string AdjustParameter(string presetName, string paramName, string valueJson) => MarshalStringAndFree(rgb_adjust_parameter(presetName, paramName, valueJson));
        public static string SetLightingPriority() => MarshalStringAndFree(rgb_set_lighting_priority());
        public static string InstallStartupTask(uint delaySeconds) => MarshalStringAndFree(rgb_install_startup_task(delaySeconds));
        public static string UninstallStartupTask() => MarshalStringAndFree(rgb_uninstall_startup_task());
        public static string GetSettings() => MarshalStringAndFree(rgb_get_settings());
        public static string SaveSettings(string settingsJson) => MarshalStringAndFree(rgb_save_settings(settingsJson));
        public static string CyclePreset() => MarshalStringAndFree(rgb_cycle_preset());

        private static string MarshalStringAndFree(IntPtr ptr)
        {
            if (ptr == IntPtr.Zero) return string.Empty;
            try
            {
                string? result = Marshal.PtrToStringUTF8(ptr);
                return result ?? string.Empty;
            }
            finally
            {
                rgb_free_string(ptr);
            }
        }
    }
}
