using System;
using System.Threading;
using System.Windows.Forms;
using RGBController.Interop;

namespace RGBController
{
    internal static class Program
    {
        private static Mutex? _appMutex;

        [STAThread]
        private static void Main()
        {
            const string mutexName = @"Global\LOQ-RGB-Controller-SingleInstance-Mutex";
            _appMutex = new Mutex(true, mutexName, out bool createdNew);

            if (!createdNew)
            {
                // Already running. Check if we should notify the user.
                bool startMinimized = false;
                string[] args = Environment.GetCommandLineArgs();
                foreach (string arg in args)
                {
                    if (arg.Equals("--minimized", StringComparison.OrdinalIgnoreCase) ||
                        arg.Equals("-minimized", StringComparison.OrdinalIgnoreCase) ||
                        arg.Equals("/minimized", StringComparison.OrdinalIgnoreCase))
                    {
                        startMinimized = true;
                        break;
                    }
                }

                if (!startMinimized)
                {
                    MessageBox.Show(
                        "RGB Controller is already running. You can access it from the system tray.",
                        "Already Running",
                        MessageBoxButtons.OK,
                        MessageBoxIcon.Information);
                }

                _appMutex.Dispose();
                return;
            }

            Application.ThreadException += Application_ThreadException;
            AppDomain.CurrentDomain.UnhandledException += CurrentDomain_UnhandledException;

            ApplicationConfiguration.Initialize();

            // Start LampArray watcher to override disabled settings in OS
            try
            {
                LampArrayManager.StartWatcher();
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to start LampArrayManager: {ex.Message}");
            }

            // Initialize Rust backend
            try
            {
                int initResult = RgbInterop.rgb_init();
                if (initResult != 1 && initResult != 0)
                {
                    MessageBox.Show(
                        $"Failed to initialize the RGB Controller Rust backend! (Error code: {initResult})",
                        "Backend Initialization Error",
                        MessageBoxButtons.OK,
                        MessageBoxIcon.Error);
                    return;
                }
            }
            catch (DllNotFoundException)
            {
                MessageBox.Show(
                    "Failed to load the RGB Controller Rust backend DLL (rgb_backend.dll)!\n" +
                    "Verify that the Rust DLL is present in the application directory.",
                    "DLL Missing Error",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Error);
                return;
            }
            catch (Exception ex)
            {
                MessageBox.Show(
                    $"An error occurred while loading the Rust backend:\n{ex.Message}",
                    "Load Error",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Error);
                return;
            }

            try
            {
                bool startMinimized = false;
                string[] args = Environment.GetCommandLineArgs();
                foreach (string arg in args)
                {
                    if (arg.Equals("--minimized", StringComparison.OrdinalIgnoreCase) ||
                        arg.Equals("-minimized", StringComparison.OrdinalIgnoreCase) ||
                        arg.Equals("/minimized", StringComparison.OrdinalIgnoreCase))
                    {
                        startMinimized = true;
                        break;
                    }
                }
                Application.Run(new MainForm(startMinimized));
            }
            finally
            {
                try
                {
                    LampArrayManager.StopWatcher();
                }
                catch {}
                RgbInterop.rgb_shutdown();
                if (_appMutex != null)
                {
                    _appMutex.ReleaseMutex();
                    _appMutex.Dispose();
                }
            }
        }

        private static void Application_ThreadException(object sender, System.Threading.ThreadExceptionEventArgs e)
        {
            MessageBox.Show(
                $"An unhandled thread exception occurred: {e.Exception.Message}\n\nStack trace:\n{e.Exception.StackTrace}",
                "Unhandled Exception",
                MessageBoxButtons.OK,
                MessageBoxIcon.Error);
        }

        private static void CurrentDomain_UnhandledException(object sender, UnhandledExceptionEventArgs e)
        {
            if (e.ExceptionObject is Exception ex)
            {
                MessageBox.Show(
                    $"An unhandled domain exception occurred: {ex.Message}\n\nStack trace:\n{ex.StackTrace}",
                    "Unhandled Exception",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Error);
            }
        }
    }
}
