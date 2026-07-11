using System;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Windows.Forms;

namespace RGBController.Controls
{
    /// <summary>
    /// A small on-screen display popup that briefly shows the active preset name
    /// in the bottom-right corner of the primary screen, then fades out.
    /// </summary>
    public class OsdPopup : Form
    {
        private static OsdPopup? _current;
        private static readonly object _lock = new();

        private readonly System.Windows.Forms.Timer _fadeTimer;
        private readonly System.Windows.Forms.Timer _holdTimer;
        private readonly string _presetName;

        private const int PopupWidth = 320;
        private const int PopupHeight = 64;
        private const int CornerRadius = 10;
        private const int ScreenMargin = 20;
        private const double FadeInStep = 0.12;
        private const double FadeOutStep = 0.06;
        private const int HoldDurationMs = 1800;
        private const int FadeIntervalMs = 16; // ~60fps

        private enum FadeState { FadingIn, Holding, FadingOut }
        private FadeState _state = FadeState.FadingIn;

        private OsdPopup(string presetDisplayName)
        {
            _presetName = presetDisplayName;

            // Form setup — borderless, topmost, non-activating
            this.FormBorderStyle = FormBorderStyle.None;
            this.StartPosition = FormStartPosition.Manual;
            this.ShowInTaskbar = false;
            this.TopMost = true;
            this.Size = new Size(PopupWidth, PopupHeight);
            this.Opacity = 0;
            this.BackColor = Color.Magenta; // transparency key
            this.TransparencyKey = Color.Magenta;
            this.DoubleBuffered = true;

            // Position: bottom-right of primary screen
            var screen = Screen.PrimaryScreen?.WorkingArea ?? new Rectangle(0, 0, 1920, 1080);
            this.Location = new Point(
                screen.Right - PopupWidth - ScreenMargin,
                screen.Bottom - PopupHeight - ScreenMargin
            );

            // Fade timer (~60fps)
            _fadeTimer = new System.Windows.Forms.Timer();
            _fadeTimer.Interval = FadeIntervalMs;
            _fadeTimer.Tick += FadeTimer_Tick;

            // Hold timer
            _holdTimer = new System.Windows.Forms.Timer();
            _holdTimer.Interval = HoldDurationMs;
            _holdTimer.Tick += HoldTimer_Tick;
        }

        /// <summary>
        /// Show a brief OSD popup with the given preset display name.
        /// If an existing popup is visible, it is immediately replaced.
        /// Thread-safe — can be called from any thread.
        /// </summary>
        public static void Show(string presetDisplayName, Form? owner = null)
        {
            void DoShow()
            {
                lock (_lock)
                {
                    // Dismiss any existing popup
                    if (_current != null && !_current.IsDisposed)
                    {
                        _current._fadeTimer.Stop();
                        _current._holdTimer.Stop();
                        _current.Close();
                        _current.Dispose();
                        _current = null;
                    }

                    var popup = new OsdPopup(presetDisplayName);
                    _current = popup;
                    popup.ShowPopup();
                }
            }

            if (owner != null && owner.InvokeRequired)
            {
                owner.BeginInvoke(new Action(DoShow));
            }
            else
            {
                DoShow();
            }
        }

        /// <summary>
        /// Immediately dismiss and dispose the current OSD popup if one exists.
        /// Called during MainForm shutdown to prevent OpenForms collection modification during form closure.
        /// </summary>
        public static void DismissCurrent()
        {
            lock (_lock)
            {
                if (_current != null && !_current.IsDisposed)
                {
                    _current._fadeTimer.Stop();
                    _current._holdTimer.Stop();
                    _current.Close();
                    _current.Dispose();
                    _current = null;
                }
            }
        }

        private void ShowPopup()
        {
            // Show without stealing focus
            this.Show();
            _state = FadeState.FadingIn;
            _fadeTimer.Start();
        }

        protected override bool ShowWithoutActivation => true;

        // Prevent the popup from stealing focus
        protected override CreateParams CreateParams
        {
            get
            {
                var cp = base.CreateParams;
                cp.ExStyle |= 0x08000000; // WS_EX_NOACTIVATE
                cp.ExStyle |= 0x00000080; // WS_EX_TOOLWINDOW (hide from Alt+Tab)
                return cp;
            }
        }

        private void FadeTimer_Tick(object? sender, EventArgs e)
        {
            switch (_state)
            {
                case FadeState.FadingIn:
                    this.Opacity += FadeInStep;
                    if (this.Opacity >= 1.0)
                    {
                        this.Opacity = 1.0;
                        _fadeTimer.Stop();
                        _state = FadeState.Holding;
                        _holdTimer.Start();
                    }
                    break;

                case FadeState.FadingOut:
                    this.Opacity -= FadeOutStep;
                    if (this.Opacity <= 0)
                    {
                        _fadeTimer.Stop();
                        this.Close();
                        this.Dispose();
                        lock (_lock)
                        {
                            if (_current == this) _current = null;
                        }
                    }
                    break;
            }
        }

        private void HoldTimer_Tick(object? sender, EventArgs e)
        {
            _holdTimer.Stop();
            _state = FadeState.FadingOut;
            _fadeTimer.Start();
        }

        protected override void OnPaint(PaintEventArgs e)
        {
            base.OnPaint(e);
            var g = e.Graphics;
            g.SmoothingMode = SmoothingMode.AntiAlias;
            g.TextRenderingHint = System.Drawing.Text.TextRenderingHint.ClearTypeGridFit;

            var rect = new Rectangle(0, 0, this.Width - 1, this.Height - 1);

            // Rounded rectangle path
            using var path = CreateRoundedRectPath(rect, CornerRadius);

            // Background fill — dark translucent card
            using (var bgBrush = new SolidBrush(Color.FromArgb(240, 12, 12, 14)))
            {
                g.FillPath(bgBrush, path);
            }

            // Subtle accent border
            using (var borderPen = new Pen(Color.FromArgb(100, 255, 255, 255), 1f))
            {
                g.DrawPath(borderPen, path);
            }

            // Top accent line (thin glow bar)
            using (var accentPen = new Pen(Color.FromArgb(180, 255, 255, 255), 2f))
            {
                g.DrawLine(accentPen, CornerRadius, 1, this.Width - CornerRadius, 1);
            }

            // Icon/label: "PRESET" subtitle
            using (var subtitleFont = new Font("Segoe UI", 8f, FontStyle.Regular))
            using (var subtitleBrush = new SolidBrush(Color.FromArgb(161, 161, 170))) // TextSecondary
            {
                g.DrawString("PRESET ACTIVE", subtitleFont, subtitleBrush, 16, 10);
            }

            // Preset name
            using (var nameFont = new Font("Segoe UI", 13f, FontStyle.Regular))
            using (var nameBrush = new SolidBrush(Color.White))
            {
                var nameRect = new RectangleF(16, 28, this.Width - 32, 32);
                var sf = new StringFormat
                {
                    Trimming = StringTrimming.EllipsisCharacter,
                    FormatFlags = StringFormatFlags.NoWrap,
                    LineAlignment = StringAlignment.Center
                };
                g.DrawString(_presetName, nameFont, nameBrush, nameRect, sf);
            }
        }

        private static GraphicsPath CreateRoundedRectPath(Rectangle rect, int radius)
        {
            var path = new GraphicsPath();
            int d = radius * 2;
            path.AddArc(rect.X, rect.Y, d, d, 180, 90);
            path.AddArc(rect.Right - d, rect.Y, d, d, 270, 90);
            path.AddArc(rect.Right - d, rect.Bottom - d, d, d, 0, 90);
            path.AddArc(rect.X, rect.Bottom - d, d, d, 90, 90);
            path.CloseFigure();
            return path;
        }

        protected override void Dispose(bool disposing)
        {
            if (disposing)
            {
                _fadeTimer?.Dispose();
                _holdTimer?.Dispose();
            }
            base.Dispose(disposing);
        }
    }
}
