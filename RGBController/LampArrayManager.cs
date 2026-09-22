using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Windows.Devices.Enumeration;
using Windows.Devices.Lights;

namespace RGBController
{
    public static class LampArrayManager
    {
        private static DeviceWatcher? _watcher;
        private static System.Threading.Timer? _enableTimer;
        private static readonly List<LampArray> _lampArrays = new List<LampArray>();
        private static readonly object _lock = new object();

        private static void Log(string msg)
        {
            try
            {
                string logPath = @"D:\Projects\LOQ-24-zone-RGB-controller\lamparray_log.txt";
                System.IO.File.AppendAllText(logPath, $"[{DateTime.Now:yyyy-MM-dd HH:mm:ss.fff}] {msg}\r\n");
            }
            catch {}
        }

        public static void StartWatcher()
        {
            lock (_lock)
            {
                if (_watcher != null) return;

                Log("StartWatcher called.");
                string selector = LampArray.GetDeviceSelector();
                _watcher = DeviceInformation.CreateWatcher(selector);
                _watcher.Added += Watcher_Added;
                _watcher.Removed += Watcher_Removed;
                _watcher.Start();

                // Periodically force enable the keyboard backlight to prevent OS Dynamic Lighting from disabling it in background/minimized states
                _enableTimer = new System.Threading.Timer((state) =>
                {
                    ForceEnableAll();
                }, null, 1000, 500);

                Log("Watcher and keep-alive timer started.");
            }
        }

        public static void StopWatcher()
        {
            lock (_lock)
            {
                Log("StopWatcher called.");
                if (_enableTimer != null)
                {
                    _enableTimer.Dispose();
                    _enableTimer = null;
                }
                if (_watcher != null)
                {
                    _watcher.Stop();
                    _watcher = null;
                }
                lock (_lampArrays)
                {
                    _lampArrays.Clear();
                }
                Log("Watcher and keep-alive timer stopped.");
            }
        }

        private static async void Watcher_Added(DeviceWatcher sender, DeviceInformation args)
        {
            Log($"Watcher_Added triggered for device: {args.Id}");
            try
            {
                var lampArray = await LampArray.FromIdAsync(args.Id);
                if (lampArray != null)
                {
                    // Force the device enabled so it accepts software HID commands
                    lampArray.IsEnabled = true;
                    lock (_lampArrays)
                    {
                        _lampArrays.Add(lampArray);
                    }
                    Log($"Added device and set IsEnabled = true. DeviceId: {args.Id}, LampCount: {lampArray.LampCount}");
                }
                else
                {
                    Log($"FromIdAsync returned null for: {args.Id}");
                }
            }
            catch (Exception ex)
            {
                Log($"Error adding device {args.Id}: {ex.Message}\n{ex.StackTrace}");
            }
        }

        private static void Watcher_Removed(DeviceWatcher sender, DeviceInformationUpdate args)
        {
            Log($"Watcher_Removed triggered for device: {args.Id}");
            lock (_lampArrays)
            {
                _lampArrays.RemoveAll(x => x.DeviceId == args.Id);
            }
        }

        public static void ForceEnableAll()
        {
            lock (_lampArrays)
            {
                if (_lampArrays.Count == 0)
                {
                    // Log occasionally to avoid spam, e.g. every few seconds, but since it's 500ms, let's log only when we actually have devices
                    return;
                }

                foreach (var lampArray in _lampArrays)
                {
                    try
                    {
                        if (!lampArray.IsEnabled)
                        {
                            lampArray.IsEnabled = true;
                            Log($"Periodic Keep-Alive: Force enabled setting to true for {lampArray.DeviceId}");
                        }
                    }
                    catch (Exception ex)
                    {
                        Log($"Periodic Keep-Alive Error enabling {lampArray.DeviceId}: {ex.Message}");
                    }
                }
            }
        }
    }
}
