#pragma warning disable CS8618

using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Windows.Forms;
using RGBController.Interop;
using RGBController.Models;
using Color = System.Drawing.Color;

namespace RGBController.Controls
{
    public class HomePanel : UserControl
    {
        private List<PresetMetadata> _presets = new();
        private RgbInterop.FrameCallback? _frameCallback;
        private Models.Color[] _zoneColors = new Models.Color[24];
        private AppSettings _settings = new();
        private string _currentPresetName = string.Empty;
        private volatile bool _pendingInvalidate = false; // throttle BeginInvoke after sleep/resume

        // UI Controls
        private FlowLayoutPanel leftFlowPanel;
        private Panel rightContainerPanel;
        private KeyboardVisualizer keyboardVisualizer;
        private Label presetLabel;
        private ComboBox presetComboBox;
        private Panel presetCardPanel;
        private Label selectedPresetHeader;
        private Label selectedPresetDesc;
        private FlowLayoutPanel parametersFlowPanel;
        private Label emptyStateText;
        private ToolTip zoneToolTip;
        private MainForm parentForm;

        public HomePanel(MainForm parent)
        {
            this.parentForm = parent;
            this.AutoScaleMode = AutoScaleMode.Inherit; // Inherit auto-scaling from parent form
            InitializeComponent();
            ApplyTheme();

            // Initialize color buffer
            for (int i = 0; i < 24; i++)
            {
                _zoneColors[i] = Models.Color.Black;
            }

            // Bind hotkey cycle event
            parentForm.OnPresetCycled += ParentForm_OnPresetCycled;

            this.Load += HomePanel_Load;
            this.HandleDestroyed += HomePanel_HandleDestroyed;
        }

        private void InitializeComponent()
        {
            this.leftFlowPanel = new FlowLayoutPanel();
            this.rightContainerPanel = new Panel();
            this.keyboardVisualizer = new KeyboardVisualizer(this);
            this.presetLabel = new Label();
            this.presetComboBox = new ComboBox();
            this.presetCardPanel = new Panel();
            this.selectedPresetHeader = new Label();
            this.selectedPresetDesc = new Label();
            this.parametersFlowPanel = new FlowLayoutPanel();
            this.emptyStateText = new Label();
            this.zoneToolTip = new ToolTip();

            this.SuspendLayout();
            this.leftFlowPanel.SuspendLayout();
            this.rightContainerPanel.SuspendLayout();

            // 
            // leftFlowPanel
            // 
            this.leftFlowPanel.Dock = DockStyle.Left;
            this.leftFlowPanel.Width = 532;
            this.leftFlowPanel.FlowDirection = FlowDirection.TopDown;
            this.leftFlowPanel.WrapContents = false;
            this.leftFlowPanel.Padding = new Padding(16, 16, 16, 16);
            this.leftFlowPanel.Name = "leftFlowPanel";

            // 
            // rightContainerPanel
            // 
            this.rightContainerPanel.Dock = DockStyle.Fill;
            this.rightContainerPanel.Padding = new Padding(0, 16, 16, 16);
            this.rightContainerPanel.Name = "rightContainerPanel";

            // 
            // keyboardVisualizer
            // 
            this.keyboardVisualizer.Margin = new Padding(0, 0, 0, 16);
            this.keyboardVisualizer.Name = "keyboardVisualizer";
            this.keyboardVisualizer.Size = new Size(500, 220);
            this.keyboardVisualizer.TabIndex = 0;

            // 
            // presetLabel
            // 
            this.presetLabel.AutoSize = true;
            this.presetLabel.Margin = new Padding(0, 16, 0, 4);
            this.presetLabel.Name = "presetLabel";
            this.presetLabel.Size = new Size(500, 20);
            this.presetLabel.Text = "ACTIVE LIGHTING PRESET";
            this.presetLabel.TextAlign = ContentAlignment.MiddleLeft;

            // 
            // presetComboBox
            // 
            this.presetComboBox.Margin = new Padding(0, 4, 0, 0);
            this.presetComboBox.Name = "presetComboBox";
            this.presetComboBox.Size = new Size(500, 30);
            this.presetComboBox.ItemHeight = 24;
            this.presetComboBox.TabIndex = 1;
            this.presetComboBox.DropDownStyle = ComboBoxStyle.DropDownList;
            this.presetComboBox.DrawMode = DrawMode.OwnerDrawFixed;
            this.presetComboBox.DrawItem += PresetComboBox_DrawItem;
            this.presetComboBox.SelectedIndexChanged += PresetComboBox_SelectedIndexChanged;

            // 
            // presetCardPanel
            // 
            this.presetCardPanel.Dock = DockStyle.Fill;
            this.presetCardPanel.BorderStyle = BorderStyle.FixedSingle;
            this.presetCardPanel.Controls.Add(this.emptyStateText);
            this.presetCardPanel.Controls.Add(this.parametersFlowPanel);
            this.presetCardPanel.Controls.Add(this.selectedPresetDesc);
            this.presetCardPanel.Controls.Add(this.selectedPresetHeader);
            this.presetCardPanel.Location = new Point(0, 0);
            this.presetCardPanel.Name = "presetCardPanel";
            this.presetCardPanel.Size = new Size(332, 572);
            this.presetCardPanel.TabIndex = 2;

            // 
            // selectedPresetHeader
            // 
            this.selectedPresetHeader.AutoSize = true;
            this.selectedPresetHeader.Location = new Point(16, 16);
            this.selectedPresetHeader.Name = "selectedPresetHeader";
            this.selectedPresetHeader.Size = new Size(74, 15);
            this.selectedPresetHeader.Text = "Preset Name";
            this.selectedPresetHeader.Visible = false;

            // 
            // selectedPresetDesc
            // 
            this.selectedPresetDesc.Anchor = AnchorStyles.Top | AnchorStyles.Left | AnchorStyles.Right;
            this.selectedPresetDesc.Location = new Point(16, 36);
            this.selectedPresetDesc.Name = "selectedPresetDesc";
            this.selectedPresetDesc.Size = new Size(300, 36);
            this.selectedPresetDesc.Text = "Preset Description";
            this.selectedPresetDesc.Visible = false;

            // 
            // parametersFlowPanel
            // 
            this.parametersFlowPanel.Anchor = AnchorStyles.Top | AnchorStyles.Bottom | AnchorStyles.Left | AnchorStyles.Right;
            this.parametersFlowPanel.FlowDirection = FlowDirection.TopDown;
            this.parametersFlowPanel.WrapContents = false;
            this.parametersFlowPanel.AutoScroll = true;
            this.parametersFlowPanel.Location = new Point(16, 80);
            this.parametersFlowPanel.Name = "parametersFlowPanel";
            this.parametersFlowPanel.Size = new Size(300, 476);
            this.parametersFlowPanel.TabIndex = 4;
            this.parametersFlowPanel.SizeChanged += (s, e) =>
            {
                int targetWidth = Math.Max(260, parametersFlowPanel.ClientSize.Width - 16);
                foreach (Control ctrl in parametersFlowPanel.Controls)
                {
                    if (ctrl is Panel container)
                    {
                        container.Width = targetWidth;
                        foreach (Control child in container.Controls)
                        {
                            if (child is TrackBar tb && child.Name == "trackBar")
                            {
                                tb.Width = targetWidth;
                            }
                            else if (child is Label lbl && child.Name == "valLabel")
                            {
                                lbl.Location = new Point(targetWidth - lbl.Width, lbl.Top);
                            }
                            else if (child is Label lblHex && child.Name == "hexLabel")
                            {
                                lblHex.Location = new Point(targetWidth - 130, lblHex.Top);
                            }
                            else if (child is Button btn && child.Name == "pickerButton")
                            {
                                btn.Location = new Point(targetWidth - btn.Width, btn.Top);
                            }
                        }
                    }
                }
            };

            // 
            // emptyStateText
            // 
            this.emptyStateText.Anchor = AnchorStyles.Top | AnchorStyles.Bottom | AnchorStyles.Left | AnchorStyles.Right;
            this.emptyStateText.Location = new Point(16, 80);
            this.emptyStateText.Name = "emptyStateText";
            this.emptyStateText.Size = new Size(300, 476);
            this.emptyStateText.Text = "SELECT A PRESET TO TWEAK PARAMETERS";
            this.emptyStateText.TextAlign = ContentAlignment.MiddleCenter;

            // Add controls to leftFlowPanel
            this.leftFlowPanel.Controls.Add(this.keyboardVisualizer);
            this.leftFlowPanel.Controls.Add(this.presetLabel);
            this.leftFlowPanel.Controls.Add(this.presetComboBox);

            // Add controls to rightContainerPanel
            this.rightContainerPanel.Controls.Add(this.presetCardPanel);

            // 
            // HomePanel
            // 
            this.BackColor = Color.FromArgb(5, 5, 5);
            this.Controls.Add(this.rightContainerPanel);
            this.Controls.Add(this.leftFlowPanel);
            this.Name = "HomePanel";
            this.Size = new Size(880, 604);

            this.leftFlowPanel.ResumeLayout(false);
            this.leftFlowPanel.PerformLayout();
            this.rightContainerPanel.ResumeLayout(false);
            this.presetCardPanel.ResumeLayout(false);
            this.presetCardPanel.PerformLayout();
            this.ResumeLayout(false);
        }

        private void ApplyTheme()
        {
            this.BackColor = Theme.Background;
            this.presetCardPanel.BackColor = Theme.Card;
            this.presetCardPanel.ForeColor = Theme.TextPrimary;

            Theme.StyleLabel(this.presetLabel, Theme.FontHeader, Theme.TextSecondary);
            this.presetComboBox.BackColor = Theme.Card;
            this.presetComboBox.ForeColor = Theme.TextSecondary;
            this.presetComboBox.Font = Theme.FontMonospace;
            this.presetComboBox.FlatStyle = FlatStyle.Flat;

            Theme.StyleLabel(this.selectedPresetHeader, Theme.FontHeader, Theme.TextPrimary);
            Theme.StyleLabel(this.selectedPresetDesc, Theme.FontMonospace, Theme.TextMuted);
            Theme.StyleLabel(this.emptyStateText, Theme.FontMonospace, Theme.TextSecondary);

            // Add thin custom border to card
            this.presetCardPanel.Paint += (s, e) =>
            {
                using (var pen = new Pen(Theme.Border, 1))
                {
                    e.Graphics.DrawRectangle(pen, 0, 0, presetCardPanel.Width - 1, presetCardPanel.Height - 1);
                }
            };
        }

        private void HomePanel_Load(object? sender, EventArgs e)
        {
            LoadSettings();
            LoadPresets();
            SetupFrameCallback();
        }

        private void HomePanel_HandleDestroyed(object? sender, EventArgs e)
        {
            parentForm.OnPresetCycled -= ParentForm_OnPresetCycled;
            try
            {
                RgbInterop.rgb_start_frame_callback(null);
            }
            catch { }
        }

        private void ParentForm_OnPresetCycled()
        {
            // Sync settings and reload
            LoadSettings();
            
            // Check currently active preset in SettingsManager
            string activePreset = SettingsManager.GetActivePreset();
            SelectPresetByName(activePreset);
        }

        public void ReloadPresetData()
        {
            LoadSettings();
            LoadPresets();
        }

        private void LoadSettings()
        {
            try
            {
                string json = RgbInterop.GetSettings();
                if (!string.IsNullOrEmpty(json))
                {
                    var s = JsonSerializer.Deserialize<AppSettings>(json);
                    if (s != null)
                    {
                        _settings = s;
                    }
                }
            }
            catch { }
        }

        private void LoadPresets()
        {
            try
            {
                string json = RgbInterop.GetPresetMetadata();
                var data = JsonSerializer.Deserialize<List<PresetMetadata>>(json);
                if (data != null)
                {
                    _presets = data;
                    
                    presetComboBox.SelectedIndexChanged -= PresetComboBox_SelectedIndexChanged;
                    presetComboBox.Items.Clear();

                    foreach (var preset in _presets)
                    {
                        presetComboBox.Items.Add(preset);
                    }

                    presetComboBox.SelectedIndexChanged += PresetComboBox_SelectedIndexChanged;

                    // Restore saved preset from local storage
                    string savedPreset = SettingsManager.GetActivePreset();
                    SelectPresetByName(savedPreset);
                }
            }
            catch { }
        }

        private void SelectPresetByName(string presetName)
        {
            var preset = _presets.FirstOrDefault(p => p.Name.Equals(presetName, StringComparison.OrdinalIgnoreCase));
            if (preset == null) return;

            // Sync ComboBox selection without triggering loop
            presetComboBox.SelectedIndexChanged -= PresetComboBox_SelectedIndexChanged;
            try
            {
                for (int i = 0; i < presetComboBox.Items.Count; i++)
                {
                    if (presetComboBox.Items[i] is PresetMetadata p && p.Name.Equals(presetName, StringComparison.OrdinalIgnoreCase))
                    {
                        presetComboBox.SelectedIndex = i;
                        break;
                    }
                }
            }
            finally
            {
                presetComboBox.SelectedIndexChanged += PresetComboBox_SelectedIndexChanged;
            }

            SelectPreset(preset);
        }

        private void PresetComboBox_SelectedIndexChanged(object? sender, EventArgs e)
        {
            if (presetComboBox.SelectedItem is PresetMetadata preset)
            {
                if (_currentPresetName != preset.Name)
                {
                    SelectPresetByName(preset.Name);
                    OsdPopup.Show(preset.DisplayName, parentForm);
                }
            }
        }

        private void PresetComboBox_DrawItem(object? sender, DrawItemEventArgs e)
        {
            if (e.Index < 0) return;

            var combo = (ComboBox)sender!;
            var item = combo.Items[e.Index] as PresetMetadata;
            if (item == null) return;

            bool isSelected = (e.State & DrawItemState.Selected) == DrawItemState.Selected;
            Color bg = isSelected ? Color.FromArgb(24, 24, 27) : Theme.Card;
            Color fg = isSelected ? Theme.TextPrimary : Theme.TextSecondary;

            using (var brushBg = new SolidBrush(bg))
            using (var brushFg = new SolidBrush(fg))
            {
                e.Graphics.FillRectangle(brushBg, e.Bounds);
                
                string text = item.DisplayName.ToUpperInvariant();
                var sf = new StringFormat
                {
                    LineAlignment = StringAlignment.Center,
                    Alignment = StringAlignment.Near
                };
                
                var textRect = new Rectangle(e.Bounds.X + 6, e.Bounds.Y, e.Bounds.Width - 12, e.Bounds.Height);
                e.Graphics.DrawString(text, Theme.FontMonospace, brushFg, textRect, sf);
            }

            using (var pen = new Pen(Theme.Border, 1))
            {
                e.Graphics.DrawRectangle(pen, e.Bounds.X, e.Bounds.Y, e.Bounds.Width - 1, e.Bounds.Height - 1);
            }
        }

        private void SetupFrameCallback()
        {
            try
            {
                _frameCallback = OnNewFrame;
                RgbInterop.rgb_start_frame_callback(_frameCallback);
            }
            catch { }
        }

        private void OnNewFrame(IntPtr buffer, int len)
        {
            if (len >= 24 && buffer != IntPtr.Zero)
            {
                unsafe
                {
                    var ptr = (Models.Color*)buffer;
                    for (int i = 0; i < 24; i++)
                    {
                        _zoneColors[i] = ptr[i];
                    }
                }

                // Invalidate keyboard underglow on UI thread.
                // Throttle: skip if we already have a pending invalidate queued,
                // which prevents flooding the UI thread after sleep/resume.
                if (this.IsDisposed || !this.IsHandleCreated) return;

                if (this.InvokeRequired)
                {
                    if (!_pendingInvalidate)
                    {
                        _pendingInvalidate = true;
                        try
                        {
                            this.BeginInvoke(new Action(() =>
                            {
                                _pendingInvalidate = false;
                                if (!this.IsDisposed && this.IsHandleCreated)
                                {
                                    this.keyboardVisualizer.Invalidate();
                                }
                            }));
                        }
                        catch
                        {
                            _pendingInvalidate = false;
                        }
                    }
                }
                else
                {
                    this.keyboardVisualizer.Invalidate();
                }
            }
        }

        private void SelectPreset(PresetMetadata preset)
        {
            _currentPresetName = preset.Name;
            
            // Save to SettingsManager
            SettingsManager.SaveActivePreset(preset.Name);

            // Load preset tweaks from settings
            Dictionary<string, ParameterValue> tweaks = new();
            if (_settings.PresetTweaks != null && _settings.PresetTweaks.TryGetValue(preset.Name, out var presetTweak))
            {
                tweaks = presetTweak;
            }

            var configParameters = new Dictionary<string, ParameterValue>();

            // Setup dynamic parameter list
            emptyStateText.Visible = false;
            selectedPresetHeader.Text = preset.DisplayName.ToUpperInvariant();
            selectedPresetHeader.Visible = true;
            selectedPresetDesc.Text = preset.Description.ToUpperInvariant();
            selectedPresetDesc.Visible = true;

            parametersFlowPanel.Controls.Clear();

            foreach (var param in preset.Parameters)
            {
                if (param.ParamType.Type == "Float")
                {
                    float currentVal = param.Default;
                    if (tweaks.TryGetValue(param.Name, out var tweakVal) && tweakVal.Type == "Float")
                    {
                        currentVal = tweakVal.FloatValue;
                        configParameters[param.Name] = tweakVal;
                    }
                    else
                    {
                        configParameters[param.Name] = ParameterValue.CreateFloat(param.Default);
                    }

                    CreateFloatControl(param, currentVal);
                }
                else if (param.ParamType.Type == "Color")
                {
                    Models.Color currentVal = new Models.Color(
                        param.ParamType.R ?? 255,
                        param.ParamType.G ?? 0,
                        param.ParamType.B ?? 0
                    );

                    if (tweaks.TryGetValue(param.Name, out var tweakVal) && tweakVal.Type == "Color")
                    {
                        currentVal = tweakVal.ColorValue;
                        configParameters[param.Name] = tweakVal;
                    }
                    else
                    {
                        configParameters[param.Name] = ParameterValue.CreateColor(currentVal.r, currentVal.g, currentVal.b);
                    }

                    CreateColorControl(param, currentVal);
                }
            }

            // Sync preset load to Rust backend
            try
            {
                string paramsJson = JsonSerializer.Serialize(configParameters);
                RgbInterop.SetPreset(preset.Name, paramsJson);
            }
            catch { }
        }

        private void CreateFloatControl(ParameterConfig param, float currentValue)
        {
            int targetWidth = Math.Max(260, parametersFlowPanel.ClientSize.Width - 16);
            var container = new Panel
            {
                Width = targetWidth,
                Height = 60,
                Margin = new Padding(0, 4, 0, 8)
            };

            var label = new Label
            {
                Text = param.Label.ToUpperInvariant(),
                Location = new Point(0, 0),
                Size = new Size(targetWidth - 100, 20),
                TextAlign = ContentAlignment.MiddleLeft
            };
            Theme.StyleLabel(label, Theme.FontMonospace, Theme.TextSecondary);

            var valLabel = new Label
            {
                Name = "valLabel",
                Text = param.Min == param.Max ? "FIXED" : $"{CalculatePercentage(currentValue, param.Min, param.Max)}%",
                Location = new Point(targetWidth - 90, 0),
                Size = new Size(90, 20),
                TextAlign = ContentAlignment.MiddleRight
            };
            Theme.StyleLabel(valLabel, Theme.FontMonospace, Theme.TextSecondary);

            var trackBar = new TrackBar
            {
                Name = "trackBar",
                Minimum = 0,
                Location = new Point(0, 20),
                Size = new Size(targetWidth, 45),
                TickStyle = TickStyle.None,
                BackColor = Theme.Card
            };

            // Map floats using steps
            float range = param.Max - param.Min;
            int steps = range > 0 && param.Step > 0 ? (int)Math.Round(range / param.Step) : 1;
            trackBar.Maximum = steps;

            int currentStep = range > 0 && param.Step > 0 ? (int)Math.Round((currentValue - param.Min) / param.Step) : 0;
            trackBar.Value = Math.Clamp(currentStep, 0, steps);

            trackBar.Scroll += (s, e) =>
            {
                float newVal = param.Min + (trackBar.Value * param.Step);
                newVal = Math.Clamp(newVal, param.Min, param.Max);

                valLabel.Text = param.Min == param.Max ? "FIXED" : $"{CalculatePercentage(newVal, param.Min, param.Max)}%";

                try
                {
                    var paramVal = ParameterValue.CreateFloat(newVal);
                    string valJson = JsonSerializer.Serialize(paramVal);
                    RgbInterop.AdjustParameter(_currentPresetName, param.Name, valJson);
                    
                    // Save to local tweak config
                    SaveParameterTweak(_currentPresetName, param.Name, paramVal);
                }
                catch { }
            };

            container.Controls.Add(label);
            container.Controls.Add(valLabel);
            container.Controls.Add(trackBar);

            parametersFlowPanel.Controls.Add(container);
        }

        private void CreateColorControl(ParameterConfig param, Models.Color currentValue)
        {
            int targetWidth = Math.Max(260, parametersFlowPanel.ClientSize.Width - 16);
            var container = new Panel
            {
                Width = targetWidth,
                Height = 36,
                Margin = new Padding(0, 4, 0, 8)
            };

            var label = new Label
            {
                Text = param.Label.ToUpperInvariant(),
                Location = new Point(0, 8),
                Size = new Size(targetWidth - 140, 20),
                TextAlign = ContentAlignment.MiddleLeft
            };
            Theme.StyleLabel(label, Theme.FontMonospace, Theme.TextSecondary);

            string hexVal = $"#{currentValue.r:X2}{currentValue.g:X2}{currentValue.b:X2}";
            var hexLabel = new Label
            {
                Name = "hexLabel",
                Text = hexVal,
                Location = new Point(targetWidth - 130, 8),
                Size = new Size(70, 20),
                TextAlign = ContentAlignment.MiddleRight
            };
            Theme.StyleLabel(hexLabel, Theme.FontMonospace, Theme.TextSecondary);

            var pickerButton = new Button
            {
                Name = "pickerButton",
                Location = new Point(targetWidth - 50, 4),
                Width = 50,
                Height = 28,
                BackColor = System.Drawing.Color.FromArgb(currentValue.r, currentValue.g, currentValue.b),
                FlatStyle = FlatStyle.Flat
            };
            pickerButton.FlatAppearance.BorderSize = 1;
            pickerButton.FlatAppearance.BorderColor = Theme.Border;

            pickerButton.Click += (s, e) =>
            {
                using (var cd = new ColorDialog())
                {
                    cd.Color = pickerButton.BackColor;
                    if (cd.ShowDialog() == DialogResult.OK)
                    {
                        var sysColor = cd.Color;
                        pickerButton.BackColor = sysColor;
                        hexLabel.Text = $"#{sysColor.R:X2}{sysColor.G:X2}{sysColor.B:X2}";

                        try
                        {
                            var paramVal = ParameterValue.CreateColor(sysColor.R, sysColor.G, sysColor.B);
                            string valJson = JsonSerializer.Serialize(paramVal);
                            RgbInterop.AdjustParameter(_currentPresetName, param.Name, valJson);
                            
                            // Save to local tweak config
                            SaveParameterTweak(_currentPresetName, param.Name, paramVal);
                        }
                        catch { }
                    }
                }
            };

            container.Controls.Add(label);
            container.Controls.Add(hexLabel);
            container.Controls.Add(pickerButton);

            parametersFlowPanel.Controls.Add(container);
        }

        private void SaveParameterTweak(string presetName, string paramName, ParameterValue val)
        {
            try
            {
                string settingsJson = RgbInterop.GetSettings();
                if (!string.IsNullOrEmpty(settingsJson))
                {
                    var settings = JsonSerializer.Deserialize<AppSettings>(settingsJson);
                    if (settings != null)
                    {
                        if (settings.PresetTweaks == null)
                        {
                            settings.PresetTweaks = new();
                        }

                        if (!settings.PresetTweaks.TryGetValue(presetName, out var tweaks))
                        {
                            tweaks = new();
                            settings.PresetTweaks[presetName] = tweaks;
                        }

                        tweaks[paramName] = val;

                        string updatedJson = JsonSerializer.Serialize(settings);
                        RgbInterop.SaveSettings(updatedJson);
                        _settings = settings; // Update local copy
                    }
                }
            }
            catch { }
        }

        private int CalculatePercentage(float val, float min, float max)
        {
            if (max <= min) return 0;
            float percent = ((val - min) / (max - min)) * 100f;
            return (int)Math.Round(percent);
        }

        // Subclass for high-performance GDI+ blur rendering
        private class KeyboardVisualizer : Control
        {
            private Image? layoutImage;
            private HomePanel parent;
            private Bitmap blurBmp;
            private System.Drawing.Imaging.ImageAttributes imageAttributes;

            public KeyboardVisualizer(HomePanel parent)
            {
                this.parent = parent;
                this.DoubleBuffered = true;
                this.BackColor = Theme.Background;

                // Pre-allocate rendering resources to avoid GC allocations in OnPaint frame loop
                this.blurBmp = new Bitmap(24, 4);
                this.imageAttributes = new System.Drawing.Imaging.ImageAttributes();
                var colorMatrix = new System.Drawing.Imaging.ColorMatrix { Matrix33 = 0.6f };
                this.imageAttributes.SetColorMatrix(colorMatrix, System.Drawing.Imaging.ColorMatrixFlag.Default, System.Drawing.Imaging.ColorAdjustType.Bitmap);

                // Load keyboard layout image
                string imgPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "Assets", "layout.png");
                if (File.Exists(imgPath))
                {
                    try
                    {
                        layoutImage = Image.FromFile(imgPath);
                    }
                    catch { }
                }
                else
                {
                    // Fallback search
                    imgPath = Path.Combine(Directory.GetCurrentDirectory(), "Assets", "layout.png");
                    if (File.Exists(imgPath))
                    {
                        try
                        {
                            layoutImage = Image.FromFile(imgPath);
                        }
                        catch { }
                    }
                }

                this.MouseMove += KeyboardVisualizer_MouseMove;
                this.MouseLeave += KeyboardVisualizer_MouseLeave;
            }

            private int HoveredZoneIndex = -1;

            private void KeyboardVisualizer_MouseMove(object? sender, MouseEventArgs e)
            {
                if (Width > 0)
                {
                    float zoneWidth = (float)Width / 24f;
                    int newHover = (int)(e.X / zoneWidth);
                    newHover = Math.Clamp(newHover, 0, 23);
                    if (newHover != HoveredZoneIndex)
                    {
                        HoveredZoneIndex = newHover;
                        this.Invalidate();
                        parent.zoneToolTip.SetToolTip(this, $"Zone {HoveredZoneIndex + 1:D2}");
                    }
                }
            }

            private void KeyboardVisualizer_MouseLeave(object? sender, EventArgs e)
            {
                if (HoveredZoneIndex != -1)
                {
                    HoveredZoneIndex = -1;
                    this.Invalidate();
                    parent.zoneToolTip.SetToolTip(this, "");
                }
            }

            protected override void OnPaint(PaintEventArgs e)
            {
                base.OnPaint(e);

                if (Width <= 0 || Height <= 0) return;

                // 1. Draw smooth horizontal/vertical blur using low-resolution bitmap scaling
                for (int x = 0; x < 24; x++)
                {
                    var c = parent._zoneColors[x];
                    blurBmp.SetPixel(x, 0, System.Drawing.Color.FromArgb(0, c.r, c.g, c.b));
                    blurBmp.SetPixel(x, 1, System.Drawing.Color.FromArgb(160, c.r, c.g, c.b));
                    blurBmp.SetPixel(x, 2, System.Drawing.Color.FromArgb(160, c.r, c.g, c.b));
                    blurBmp.SetPixel(x, 3, System.Drawing.Color.FromArgb(0, c.r, c.g, c.b));
                }

                e.Graphics.InterpolationMode = System.Drawing.Drawing2D.InterpolationMode.HighQualityBicubic;

                e.Graphics.DrawImage(
                    blurBmp,
                    new Rectangle(0, 0, Width, Height),
                    0, 0, 24, 4,
                    GraphicsUnit.Pixel,
                    imageAttributes
                );

                // 2. Draw keyboard layout asset overlay
                if (layoutImage != null)
                {
                    e.Graphics.DrawImage(layoutImage, 0, 0, Width, Height);
                }
                else
                {
                    using (var borderPen = new Pen(Theme.Border, 1))
                    {
                        e.Graphics.DrawRectangle(borderPen, 0, 0, Width - 1, Height - 1);
                    }
                }

                // 3. Draw hover feedback highlighted column
                if (HoveredZoneIndex >= 0 && HoveredZoneIndex < 24)
                {
                    float colWidth = (float)Width / 24f;
                    float x = HoveredZoneIndex * colWidth;
                    
                    using (var hoverBrush = new SolidBrush(System.Drawing.Color.FromArgb(20, 255, 255, 255)))
                    using (var hoverPen = new Pen(System.Drawing.Color.FromArgb(60, 255, 255, 255), 1))
                    {
                        e.Graphics.FillRectangle(hoverBrush, x, 0, colWidth, Height);
                        e.Graphics.DrawRectangle(hoverPen, x, 0, colWidth - 1, Height - 1);
                    }
                }
            }

            protected override void Dispose(bool disposing)
            {
                if (disposing)
                {
                    blurBmp?.Dispose();
                    imageAttributes?.Dispose();
                    layoutImage?.Dispose();
                }
                base.Dispose(disposing);
            }
        }
    }
}
