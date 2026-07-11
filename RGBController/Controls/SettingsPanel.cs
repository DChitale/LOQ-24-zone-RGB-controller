#pragma warning disable CS8618

using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Text.Json;
using System.Threading.Tasks;
using System.Windows.Forms;
using Microsoft.Win32;
using RGBController.Interop;
using RGBController.Models;
using Color = System.Drawing.Color;

namespace RGBController.Controls
{
    public class SettingsPanel : UserControl
    {
        private AppSettings _settings = new();
        private List<PresetMetadata> _presets = new();
        private List<PresetMetadata> _cyclePresets = new();
        private bool _isListeningShortcut = false;
        private bool _isStartupInstalled = false;
        private MainForm parentForm;

        // UI Controls
        private Label titleLabel;
        private Label descLabel;
        private Label versionLabel;

        private Panel statusPanel;
        private Label statusTextLabel;

        private Label overrideLabel;
        private Panel overrideCard;
        private Label overrideDesc;
        private Button takeControlButton;

        private Label automationLabel;
        private Panel automationCard;

        private CheckBox autoFixCheckBox;
        private Panel delayPanel;
        private Label delayLabel;
        private TrackBar delayTrackBar;
        private Label delayValueLabel;

        private CheckBox launchOnStartupCheckBox;
        private CheckBox fixOnAppLaunchCheckBox;

        private Label hotkeyLabel;
        private Button shortcutButton;

        private Label cycleLabel;
        private ComboBox addPresetComboBox;
        private ListBox cycleListBox;
        private Button removeCyclePresetButton;

        private Button saveConfigButton;
        private System.Windows.Forms.Timer statusTimer;

        public SettingsPanel(MainForm parent)
        {
            this.parentForm = parent;
            InitializeComponent();
            ApplyTheme();

            statusTimer = new System.Windows.Forms.Timer();
            statusTimer.Interval = 3000;
            statusTimer.Tick += StatusTimer_Tick;

            this.Load += SettingsPanel_Load;
        }

        private void InitializeComponent()
        {
            this.titleLabel = new Label();
            this.descLabel = new Label();
            this.versionLabel = new Label();

            this.statusPanel = new Panel();
            this.statusTextLabel = new Label();

            this.overrideLabel = new Label();
            this.overrideCard = new Panel();
            this.overrideDesc = new Label();
            this.takeControlButton = new Button();

            this.automationLabel = new Label();
            this.automationCard = new Panel();

            this.autoFixCheckBox = new CheckBox();
            this.delayPanel = new Panel();
            this.delayLabel = new Label();
            this.delayTrackBar = new TrackBar();
            this.delayValueLabel = new Label();

            this.launchOnStartupCheckBox = new CheckBox();
            this.fixOnAppLaunchCheckBox = new CheckBox();

            this.hotkeyLabel = new Label();
            this.shortcutButton = new Button();

            this.cycleLabel = new Label();
            this.addPresetComboBox = new ComboBox();
            this.cycleListBox = new ListBox();
            this.removeCyclePresetButton = new Button();

            this.saveConfigButton = new Button();

            this.overrideCard.SuspendLayout();
            this.automationCard.SuspendLayout();
            this.delayPanel.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.delayTrackBar)).BeginInit();
            this.statusPanel.SuspendLayout();
            this.SuspendLayout();

            // 
            // titleLabel
            // 
            this.titleLabel.AutoSize = true;
            this.titleLabel.Location = new Point(24, 24);
            this.titleLabel.Name = "titleLabel";
            this.titleLabel.Size = new Size(130, 25);
            this.titleLabel.Text = "LIGHTING_CONTROL";

            // 
            // descLabel
            // 
            this.descLabel.AutoSize = true;
            this.descLabel.Location = new Point(24, 54);
            this.descLabel.Name = "descLabel";
            this.descLabel.Size = new Size(58, 15);
            this.descLabel.Text = "Settings";

            // 
            // versionLabel
            // 
            this.versionLabel.AutoSize = true;
            this.versionLabel.Location = new Point(24, 76);
            this.versionLabel.Name = "versionLabel";
            this.versionLabel.Size = new Size(99, 15);
            this.versionLabel.Text = "v3.1.2-STABLE";

            // 
            // statusPanel
            // 
            this.statusPanel.Anchor = AnchorStyles.Top | AnchorStyles.Left | AnchorStyles.Right;
            this.statusPanel.BorderStyle = BorderStyle.FixedSingle;
            this.statusPanel.Controls.Add(this.statusTextLabel);
            this.statusPanel.Location = new Point(24, 106);
            this.statusPanel.Name = "statusPanel";
            this.statusPanel.Size = new Size(820, 40);
            this.statusPanel.TabIndex = 3;
            this.statusPanel.Visible = false;

            // 
            // statusTextLabel
            // 
            this.statusTextLabel.Dock = DockStyle.Fill;
            this.statusTextLabel.Location = new Point(0, 0);
            this.statusTextLabel.Name = "statusTextLabel";
            this.statusTextLabel.Padding = new Padding(12, 0, 12, 0);
            this.statusTextLabel.Size = new Size(818, 38);
            this.statusTextLabel.TextAlign = ContentAlignment.MiddleLeft;

            // 
            // overrideLabel
            // 
            this.overrideLabel.AutoSize = true;
            this.overrideLabel.Location = new Point(24, 160);
            this.overrideLabel.Name = "overrideLabel";
            this.overrideLabel.Size = new Size(119, 15);
            this.overrideLabel.Text = "MANUAL_OVERRIDE";

            // 
            // overrideCard
            // 
            this.overrideCard.Anchor = AnchorStyles.Top | AnchorStyles.Left | AnchorStyles.Right;
            this.overrideCard.BorderStyle = BorderStyle.FixedSingle;
            this.overrideCard.Controls.Add(this.takeControlButton);
            this.overrideCard.Controls.Add(this.overrideDesc);
            this.overrideCard.Location = new Point(24, 180);
            this.overrideCard.Name = "overrideCard";
            this.overrideCard.Size = new Size(420, 110);
            this.overrideCard.TabIndex = 4;

            // 
            // overrideDesc
            // 
            this.overrideDesc.Anchor = AnchorStyles.Top | AnchorStyles.Left | AnchorStyles.Right;
            this.overrideDesc.Location = new Point(16, 16);
            this.overrideDesc.Name = "overrideDesc";
            this.overrideDesc.Size = new Size(388, 36);
            this.overrideDesc.Text = "Execute immediate controller priority swap to Windows default.";

            // 
            // takeControlButton
            // 
            this.takeControlButton.Anchor = AnchorStyles.Bottom | AnchorStyles.Left | AnchorStyles.Right;
            this.takeControlButton.Location = new Point(16, 58);
            this.takeControlButton.Name = "takeControlButton";
            this.takeControlButton.Size = new Size(388, 36);
            this.takeControlButton.TabIndex = 1;
            this.takeControlButton.Text = "TAKE_CONTROL_NOW";
            this.takeControlButton.UseVisualStyleBackColor = true;
            this.takeControlButton.Click += new EventHandler(this.TakeControlNow_Click);

            // 
            // automationLabel
            // 
            this.automationLabel.AutoSize = true;
            this.automationLabel.Location = new Point(24, 305);
            this.automationLabel.Name = "automationLabel";
            this.automationLabel.Size = new Size(160, 15);
            this.automationLabel.Text = "AUTOMATION_PARAMETERS";

            // 
            // automationCard
            // 
            this.automationCard.Anchor = AnchorStyles.Top | AnchorStyles.Bottom | AnchorStyles.Left | AnchorStyles.Right;
            this.automationCard.BorderStyle = BorderStyle.FixedSingle;
            this.automationCard.Controls.Add(this.fixOnAppLaunchCheckBox);
            this.automationCard.Controls.Add(this.launchOnStartupCheckBox);
            this.automationCard.Controls.Add(this.delayPanel);
            this.automationCard.Controls.Add(this.autoFixCheckBox);
            this.automationCard.Location = new Point(24, 325);
            this.automationCard.Name = "automationCard";
            this.automationCard.Size = new Size(420, 280);
            this.automationCard.TabIndex = 5;

            // 
            // autoFixCheckBox
            // 
            this.autoFixCheckBox.AutoSize = true;
            this.autoFixCheckBox.Location = new Point(16, 16);
            this.autoFixCheckBox.Name = "autoFixCheckBox";
            this.autoFixCheckBox.Size = new Size(244, 34);
            this.autoFixCheckBox.TabIndex = 0;
            this.autoFixCheckBox.Text = "Auto-Fix on System Startup\nExecute priority swap after login sequence.";
            this.autoFixCheckBox.UseVisualStyleBackColor = true;
            this.autoFixCheckBox.CheckedChanged += new EventHandler(this.AutoFixCheckBox_CheckedChanged);

            // 
            // delayPanel
            // 
            this.delayPanel.Controls.Add(this.delayValueLabel);
            this.delayPanel.Controls.Add(this.delayTrackBar);
            this.delayPanel.Controls.Add(this.delayLabel);
            this.delayPanel.Location = new Point(16, 60);
            this.delayPanel.Name = "delayPanel";
            this.delayPanel.Size = new Size(388, 100);
            this.delayPanel.TabIndex = 1;
            this.delayPanel.Visible = false;

            // 
            // delayLabel
            // 
            this.delayLabel.AutoSize = true;
            this.delayLabel.Location = new Point(0, 8);
            this.delayLabel.Name = "delayLabel";
            this.delayLabel.Size = new Size(182, 30);
            this.delayLabel.Text = "Execution Delay\nWait period before override activation.";

            // 
            // delayValueLabel
            // 
            this.delayValueLabel.Location = new Point(310, 8);
            this.delayValueLabel.Name = "delayValueLabel";
            this.delayValueLabel.Size = new Size(78, 30);
            this.delayValueLabel.Text = "60s";
            this.delayValueLabel.TextAlign = ContentAlignment.MiddleRight;

            // 
            // delayTrackBar
            // 
            this.delayTrackBar.Location = new Point(0, 50);
            this.delayTrackBar.Maximum = 300;
            this.delayTrackBar.Minimum = 30;
            this.delayTrackBar.Name = "delayTrackBar";
            this.delayTrackBar.Size = new Size(388, 45);
            this.delayTrackBar.TabIndex = 1;
            this.delayTrackBar.TickFrequency = 15;
            this.delayTrackBar.TickStyle = TickStyle.None;
            this.delayTrackBar.Value = 60;
            this.delayTrackBar.Scroll += new EventHandler(this.DelayTrackBar_Scroll);

            // 
            // launchOnStartupCheckBox
            // 
            this.launchOnStartupCheckBox.AutoSize = true;
            this.launchOnStartupCheckBox.Location = new Point(16, 180);
            this.launchOnStartupCheckBox.Name = "launchOnStartupCheckBox";
            this.launchOnStartupCheckBox.Size = new Size(260, 34);
            this.launchOnStartupCheckBox.TabIndex = 2;
            this.launchOnStartupCheckBox.Text = "Launch on System Startup\nAutomatically start the app when you log in.";
            this.launchOnStartupCheckBox.UseVisualStyleBackColor = true;

            // 
            // fixOnAppLaunchCheckBox
            // 
            this.fixOnAppLaunchCheckBox.AutoSize = true;
            this.fixOnAppLaunchCheckBox.Location = new Point(16, 220);
            this.fixOnAppLaunchCheckBox.Name = "fixOnAppLaunchCheckBox";
            this.fixOnAppLaunchCheckBox.Size = new Size(260, 34);
            this.fixOnAppLaunchCheckBox.TabIndex = 3;
            this.fixOnAppLaunchCheckBox.Text = "Fix on Application Launch\nApply override when control panel initializes.";
            this.fixOnAppLaunchCheckBox.UseVisualStyleBackColor = true;

            // 
            // hotkeyLabel
            // 
            this.hotkeyLabel.Anchor = AnchorStyles.Top | AnchorStyles.Right;
            this.hotkeyLabel.AutoSize = true;
            this.hotkeyLabel.Location = new Point(468, 160);
            this.hotkeyLabel.Name = "hotkeyLabel";
            this.hotkeyLabel.Size = new Size(140, 15);
            this.hotkeyLabel.Text = "PRESET_CYCLE_HOTKEY";

            // 
            // shortcutButton
            // 
            this.shortcutButton.Anchor = AnchorStyles.Top | AnchorStyles.Right;
            this.shortcutButton.Location = new Point(468, 180);
            this.shortcutButton.Name = "shortcutButton";
            this.shortcutButton.Size = new Size(376, 36);
            this.shortcutButton.TabIndex = 6;
            this.shortcutButton.Text = "CLICK TO SET SHORTCUT";
            this.shortcutButton.UseVisualStyleBackColor = true;
            this.shortcutButton.Click += new EventHandler(this.ShortcutButton_Click);
            this.shortcutButton.KeyDown += new KeyEventHandler(this.ShortcutButton_KeyDown);

            // 
            // cycleLabel
            // 
            this.cycleLabel.Anchor = AnchorStyles.Top | AnchorStyles.Right;
            this.cycleLabel.AutoSize = true;
            this.cycleLabel.Location = new Point(468, 230);
            this.cycleLabel.Name = "cycleLabel";
            this.cycleLabel.Size = new Size(125, 15);
            this.cycleLabel.Text = "Cycling Presets (0)";

            // 
            // addPresetComboBox
            // 
            this.addPresetComboBox.Anchor = AnchorStyles.Top | AnchorStyles.Right;
            this.addPresetComboBox.DropDownStyle = ComboBoxStyle.DropDownList;
            this.addPresetComboBox.Location = new Point(468, 250);
            this.addPresetComboBox.Name = "addPresetComboBox";
            this.addPresetComboBox.Size = new Size(376, 23);
            this.addPresetComboBox.TabIndex = 7;
            this.addPresetComboBox.SelectedIndexChanged += new EventHandler(this.AddPresetComboBox_SelectedIndexChanged);

            // 
            // cycleListBox
            // 
            this.cycleListBox.Anchor = AnchorStyles.Top | AnchorStyles.Bottom | AnchorStyles.Right;
            this.cycleListBox.BorderStyle = BorderStyle.FixedSingle;
            this.cycleListBox.Location = new Point(468, 290);
            this.cycleListBox.Name = "cycleListBox";
            this.cycleListBox.Size = new Size(376, 190);
            this.cycleListBox.TabIndex = 8;

            // 
            // removeCyclePresetButton
            // 
            this.removeCyclePresetButton.Anchor = AnchorStyles.Bottom | AnchorStyles.Right;
            this.removeCyclePresetButton.Location = new Point(468, 490);
            this.removeCyclePresetButton.Name = "removeCyclePresetButton";
            this.removeCyclePresetButton.Size = new Size(376, 28);
            this.removeCyclePresetButton.TabIndex = 9;
            this.removeCyclePresetButton.Text = "Remove Selected Effect";
            this.removeCyclePresetButton.UseVisualStyleBackColor = true;
            this.removeCyclePresetButton.Click += new EventHandler(this.RemoveCyclePresetButton_Click);

            // 
            // saveConfigButton
            // 
            this.saveConfigButton.Anchor = AnchorStyles.Bottom | AnchorStyles.Left | AnchorStyles.Right;
            this.saveConfigButton.Location = new Point(24, 580);
            this.saveConfigButton.Name = "saveConfigButton";
            this.saveConfigButton.Size = new Size(820, 36);
            this.saveConfigButton.TabIndex = 10;
            this.saveConfigButton.Text = "Save Configuration";
            this.saveConfigButton.UseVisualStyleBackColor = true;
            this.saveConfigButton.Click += new EventHandler(this.SaveSettings_Click);

            // 
            // SettingsPanel
            // 
            this.BackColor = Color.FromArgb(5, 5, 5);
            this.Controls.Add(this.saveConfigButton);
            this.Controls.Add(this.removeCyclePresetButton);
            this.Controls.Add(this.cycleListBox);
            this.Controls.Add(this.addPresetComboBox);
            this.Controls.Add(this.cycleLabel);
            this.Controls.Add(this.shortcutButton);
            this.Controls.Add(this.hotkeyLabel);
            this.Controls.Add(this.automationCard);
            this.Controls.Add(this.automationLabel);
            this.Controls.Add(this.overrideCard);
            this.Controls.Add(this.overrideLabel);
            this.Controls.Add(this.statusPanel);
            this.Controls.Add(this.versionLabel);
            this.Controls.Add(this.descLabel);
            this.Controls.Add(this.titleLabel);
            this.Name = "SettingsPanel";
            this.Size = new Size(868, 632);

            this.overrideCard.ResumeLayout(false);
            this.overrideCard.PerformLayout();
            this.automationCard.ResumeLayout(false);
            this.automationCard.PerformLayout();
            this.delayPanel.ResumeLayout(false);
            this.delayPanel.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.delayTrackBar)).EndInit();
            this.statusPanel.ResumeLayout(false);
            this.ResumeLayout(false);
            this.PerformLayout();
        }

        private void ApplyTheme()
        {
            this.BackColor = Theme.Background;
            this.ForeColor = Theme.TextPrimary;

            Theme.StyleLabel(this.titleLabel, Theme.FontTitle, Theme.TextPrimary);
            Theme.StyleLabel(this.descLabel, Theme.FontHeader, Theme.TextPrimary);
            Theme.StyleLabel(this.versionLabel, Theme.FontMonospace, Theme.TextMuted);

            Theme.StyleLabel(this.overrideLabel, Theme.FontMonospace, Theme.TextSecondary);
            this.overrideCard.BackColor = Theme.Card;
            Theme.StyleLabel(this.overrideDesc, Theme.FontBody, Theme.TextSecondary);
            Theme.StyleFlatButton(this.takeControlButton, Color.White, Color.Black);

            Theme.StyleLabel(this.automationLabel, Theme.FontMonospace, Theme.TextSecondary);
            this.automationCard.BackColor = Theme.Card;
            
            // Styled checkboxes
            this.autoFixCheckBox.BackColor = Theme.Card;
            this.autoFixCheckBox.ForeColor = Theme.TextPrimary;
            this.autoFixCheckBox.Font = Theme.FontBody;

            this.launchOnStartupCheckBox.BackColor = Theme.Card;
            this.launchOnStartupCheckBox.ForeColor = Theme.TextPrimary;
            this.launchOnStartupCheckBox.Font = Theme.FontBody;

            this.fixOnAppLaunchCheckBox.BackColor = Theme.Card;
            this.fixOnAppLaunchCheckBox.ForeColor = Theme.TextPrimary;
            this.fixOnAppLaunchCheckBox.Font = Theme.FontBody;

            this.delayPanel.BackColor = Theme.Card;
            Theme.StyleLabel(this.delayLabel, Theme.FontBody, Theme.TextSecondary);
            Theme.StyleLabel(this.delayValueLabel, Theme.FontTitle, Theme.TextPrimary);
            this.delayTrackBar.BackColor = Theme.Card;

            Theme.StyleLabel(this.hotkeyLabel, Theme.FontMonospace, Theme.TextSecondary);
            Theme.StyleFlatButton(this.shortcutButton, Theme.Card, Theme.TextSecondary, Theme.Border);

            Theme.StyleLabel(this.cycleLabel, Theme.FontMonospace, Theme.TextSecondary);
            this.addPresetComboBox.BackColor = Theme.Card;
            this.addPresetComboBox.ForeColor = Theme.TextPrimary;
            this.addPresetComboBox.Font = Theme.FontBody;
            this.addPresetComboBox.FlatStyle = FlatStyle.Flat;

            this.cycleListBox.BackColor = Theme.Card;
            this.cycleListBox.ForeColor = Theme.TextPrimary;
            this.cycleListBox.Font = Theme.FontBody;

            Theme.StyleFlatButton(this.removeCyclePresetButton, Theme.Card, Theme.TextSecondary, Theme.Border);
            Theme.StyleFlatButton(this.saveConfigButton, Color.White, Color.Black);

            this.statusPanel.BackColor = Theme.Card;
            Theme.StyleLabel(this.statusTextLabel, Theme.FontMonospace, Theme.TextPrimary);

            // Card custom borders
            this.statusPanel.Paint += (s, e) => DrawBorder(e.Graphics, statusPanel);
            this.overrideCard.Paint += (s, e) => DrawBorder(e.Graphics, overrideCard);
            this.automationCard.Paint += (s, e) => DrawBorder(e.Graphics, automationCard);
        }

        private void DrawBorder(Graphics g, Panel p)
        {
            using (var pen = new Pen(Theme.Border, 1))
            {
                g.DrawRectangle(pen, 0, 0, p.Width - 1, p.Height - 1);
            }
        }

        private void SettingsPanel_Load(object? sender, EventArgs e)
        {
            LoadPresets();
            LoadSettings();
            CheckStartupStatus();
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
                    
                    // Bind combobox
                    addPresetComboBox.SelectedIndexChanged -= AddPresetComboBox_SelectedIndexChanged;
                    addPresetComboBox.DataSource = null;
                    addPresetComboBox.DataSource = _presets;
                    addPresetComboBox.DisplayMember = "DisplayName";
                    addPresetComboBox.SelectedIndex = -1;
                    addPresetComboBox.SelectedIndexChanged += AddPresetComboBox_SelectedIndexChanged;
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to load presets: {ex.Message}");
            }
        }

        private void LoadSettings()
        {
            try
            {
                string json = RgbInterop.GetSettings();
                var s = JsonSerializer.Deserialize<AppSettings>(json);
                if (s != null)
                {
                    _settings = s;

                    autoFixCheckBox.Checked = _settings.AutoFixOnStartup;
                    delayTrackBar.Value = (int)Math.Clamp(_settings.StartupDelaySeconds, 30, 300);
                    delayValueLabel.Text = $"{_settings.StartupDelaySeconds}s";
                    launchOnStartupCheckBox.Checked = IsLaunchOnStartupEnabled();
                    fixOnAppLaunchCheckBox.Checked = _settings.FixOnAppLaunch;

                    if (!string.IsNullOrEmpty(_settings.PresetCycleShortcut))
                    {
                        shortcutButton.Text = _settings.PresetCycleShortcut;
                    }
                    else
                    {
                        shortcutButton.Text = "CLICK TO SET SHORTCUT";
                    }

                    RefreshCyclePresetsList();
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to load settings: {ex.Message}");
            }
        }

        private async void CheckStartupStatus()
        {
            try
            {
                int installed = await Task.Run(() => RgbInterop.rgb_check_startup_installed());
                _isStartupInstalled = installed == 1;
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to check startup status: {ex.Message}");
            }
        }

        private void RefreshCyclePresetsList()
        {
            _cyclePresets.Clear();
            cycleListBox.Items.Clear();

            if (_settings.PresetCycleEffects != null)
            {
                foreach (var name in _settings.PresetCycleEffects)
                {
                    var p = _presets.FirstOrDefault(x => x.Name.Equals(name, StringComparison.OrdinalIgnoreCase));
                    if (p != null)
                    {
                        _cyclePresets.Add(p);
                        cycleListBox.Items.Add(p.DisplayName);
                    }
                    else
                    {
                        var mockP = new PresetMetadata { Name = name, DisplayName = name };
                        _cyclePresets.Add(mockP);
                        cycleListBox.Items.Add(mockP.DisplayName);
                    }
                }
            }

            cycleLabel.Text = $"Cycling Presets ({_cyclePresets.Count})";
        }

        private void TakeControlNow_Click(object? sender, EventArgs e)
        {
            ShowStatus("PROCESSING OVERRIDE...", true);
            try
            {
                string result = RgbInterop.SetLightingPriority();
                if (result.Contains("Error"))
                {
                    ShowStatus($"✗ OVERRIDE FAILED: {result}", false);
                }
                else
                {
                    ShowStatus("✓ CONTROL ACQUIRED successfully.", true);
                }
            }
            catch (Exception ex)
            {
                ShowStatus($"✗ OVERRIDE ERROR: {ex.Message}", false);
            }
        }

        private void AutoFixCheckBox_CheckedChanged(object? sender, EventArgs e)
        {
            delayPanel.Visible = autoFixCheckBox.Checked;
        }

        private void DelayTrackBar_Scroll(object? sender, EventArgs e)
        {
            // Snap to 15s step
            int val = delayTrackBar.Value;
            int step = 15;
            int snapped = (int)(Math.Round((double)val / step) * step);
            delayTrackBar.Value = Math.Clamp(snapped, 30, 300);
            delayValueLabel.Text = $"{delayTrackBar.Value}s";
        }

        private void ShortcutButton_Click(object? sender, EventArgs e)
        {
            parentForm.UnregisterCurrentHotkey();
            _isListeningShortcut = true;
            shortcutButton.Text = "PRESS KEY COMBINATION... (ESC to cancel, BACKSPACE/DELETE to clear)";
            shortcutButton.Focus();
        }

        private void ShortcutButton_KeyDown(object? sender, KeyEventArgs e)
        {
            if (!_isListeningShortcut) return;
            e.Handled = true;
            e.SuppressKeyPress = true;

            Keys key = e.KeyCode;

            if (key == Keys.Escape)
            {
                _isListeningShortcut = false;
                shortcutButton.Text = string.IsNullOrEmpty(_settings.PresetCycleShortcut) ? "CLICK TO SET SHORTCUT" : _settings.PresetCycleShortcut;
                parentForm.LoadSettingsAndRegisterHotkey();
                return;
            }

            if (key == Keys.Back || key == Keys.Delete)
            {
                _settings.PresetCycleShortcut = null;
                _isListeningShortcut = false;
                shortcutButton.Text = "CLICK TO SET SHORTCUT";
                parentForm.LoadSettingsAndRegisterHotkey();
                return;
            }

            // Ignore just modifier keys themselves
            if (key == Keys.ControlKey || key == Keys.ShiftKey || key == Keys.Menu || key == Keys.LWin || key == Keys.RWin)
            {
                return;
            }

            List<string> modifiers = new List<string>();
            if (e.Control)
            {
                modifiers.Add("CommandOrControl");
            }
            if (e.Shift)
            {
                modifiers.Add("Shift");
            }
            if (e.Alt)
            {
                modifiers.Add("Alt");
            }

            string keyName = key.ToString();
            // Simplify some common key name mappings
            if (key >= Keys.D0 && key <= Keys.D9)
            {
                keyName = ((int)key - (int)Keys.D0).ToString();
            }
            else if (key >= Keys.NumPad0 && key <= Keys.NumPad9)
            {
                keyName = "num" + ((int)key - (int)Keys.NumPad0).ToString();
            }

            modifiers.Add(keyName);
            string shortcut = string.Join("+", modifiers);

            _settings.PresetCycleShortcut = shortcut;
            _isListeningShortcut = false;
            shortcutButton.Text = shortcut;
            parentForm.LoadSettingsAndRegisterHotkey();
        }

        private void AddPresetComboBox_SelectedIndexChanged(object? sender, EventArgs e)
        {
            if (addPresetComboBox.SelectedItem is PresetMetadata preset)
            {
                if (_settings.PresetCycleEffects == null)
                {
                    _settings.PresetCycleEffects = new List<string>();
                }

                if (!_settings.PresetCycleEffects.Contains(preset.Name))
                {
                    _settings.PresetCycleEffects.Add(preset.Name);
                    RefreshCyclePresetsList();
                }
                
                // Reset combobox selection
                addPresetComboBox.SelectedIndexChanged -= AddPresetComboBox_SelectedIndexChanged;
                addPresetComboBox.SelectedIndex = -1;
                addPresetComboBox.SelectedIndexChanged += AddPresetComboBox_SelectedIndexChanged;
            }
        }

        private void RemoveCyclePresetButton_Click(object? sender, EventArgs e)
        {
            int idx = cycleListBox.SelectedIndex;
            if (idx >= 0 && idx < _cyclePresets.Count)
            {
                var preset = _cyclePresets[idx];
                if (_settings.PresetCycleEffects != null && _settings.PresetCycleEffects.Contains(preset.Name))
                {
                    _settings.PresetCycleEffects.Remove(preset.Name);
                    RefreshCyclePresetsList();
                }
            }
        }

        private void SaveSettings_Click(object? sender, EventArgs e)
        {
            ShowStatus("SAVING CONFIGURATION...", true);

            _settings.AutoFixOnStartup = autoFixCheckBox.Checked;
            _settings.StartupDelaySeconds = (uint)delayTrackBar.Value;
            _settings.FixOnAppLaunch = fixOnAppLaunchCheckBox.Checked;

            try
            {
                string json = JsonSerializer.Serialize(_settings);
                string res = RgbInterop.SaveSettings(json);

                if (res.Contains("Error"))
                {
                    ShowStatus($"✗ SAVE FAILED: {res}", false);
                    return;
                }

                // Handle startup task installation based on toggle
                if (_settings.AutoFixOnStartup)
                {
                    string startupRes = RgbInterop.InstallStartupTask(_settings.StartupDelaySeconds);
                    if (startupRes.Contains("Error"))
                    {
                        ShowStatus($"✗ STARTUP TASK INSTALLATION FAILED: {startupRes}", false);
                        return;
                    }
                }
                else
                {
                    if (_isStartupInstalled)
                    {
                        string uninstallRes = RgbInterop.UninstallStartupTask();
                        if (uninstallRes.Contains("Error"))
                        {
                            ShowStatus($"✗ STARTUP TASK UNINSTALLATION FAILED: {uninstallRes}", false);
                            return;
                        }
                    }
                }

                CheckStartupStatus();

                // Handle launch-on-startup registry entry
                try
                {
                    SetLaunchOnStartup(launchOnStartupCheckBox.Checked);
                }
                catch (Exception regEx)
                {
                    ShowStatus($"✗ LAUNCH ON STARTUP FAILED: {regEx.Message}", false);
                    return;
                }

                // Refresh key mapping in main window
                parentForm.LoadSettingsAndRegisterHotkey();

                ShowStatus("✓ CONFIGURATION_APPLIED successfully.", true);
                statusTimer.Start();
            }
            catch (Exception ex)
            {
                ShowStatus($"✗ SAVE ERROR: {ex.Message}", false);
            }
        }

        private void ShowStatus(string message, bool isSuccess)
        {
            statusPanel.Visible = true;
            statusTextLabel.Text = message;
            if (isSuccess)
            {
                statusPanel.ForeColor = Theme.TextPrimary;
                statusPanel.BackColor = Theme.Card;
            }
            else
            {
                statusPanel.ForeColor = Color.FromArgb(248, 113, 113); // light red
                statusPanel.BackColor = Color.FromArgb(127, 29, 29); // dark red
            }
        }

        private void StatusTimer_Tick(object? sender, EventArgs e)
        {
            statusTimer.Stop();
            statusPanel.Visible = false;
        }

        // --- Launch on Startup via Registry Run key ---

        private const string RunKeyPath = @"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
        private const string RunValueName = "Legion RGB Controller";

        private static bool IsLaunchOnStartupEnabled()
        {
            try
            {
                using var key = Registry.CurrentUser.OpenSubKey(RunKeyPath, false);
                return key?.GetValue(RunValueName) != null;
            }
            catch
            {
                return false;
            }
        }

        private static void SetLaunchOnStartup(bool enable)
        {
            using var key = Registry.CurrentUser.OpenSubKey(RunKeyPath, true)
                ?? throw new InvalidOperationException("Cannot open registry Run key for writing.");

            if (enable)
            {
                // Quote the exe path so paths with spaces work correctly
                string exePath = $"\"{System.Diagnostics.Process.GetCurrentProcess().MainModule!.FileName}\" --minimized";
                key.SetValue(RunValueName, exePath, RegistryValueKind.String);
            }
            else
            {
                key.DeleteValue(RunValueName, throwOnMissingValue: false);
            }
        }
    }
}
