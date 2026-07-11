#pragma warning disable CS8618
#pragma warning disable CS8602

using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Windows.Forms;

namespace RGBController.Controls
{
    public class ConsolePanel : UserControl
    {
        [DllImport("dwmapi.dll")]
        private static extern int DwmSetWindowAttribute(IntPtr hwnd, int attr, ref int attrValue, int attrSize);

        public class ScriptEntry
        {
            public string Id { get; set; } = string.Empty;
            public string Name { get; set; } = string.Empty;
            public string Content { get; set; } = string.Empty;
        }

        private List<ScriptEntry> _scripts = new();
        private ScriptEntry? _activeScript;
        private readonly string _defaultCode = "// core_init.json\n{\n  \"sequence\": \"intercept\",\n  \"zones\": [1, 24],\n  \"rgb\": [255, 255, 255],\n  \"pulse\": true\n}";

        // UI Controls
        private Panel leftPanel;
        private Label storageLabel;
        private Label archivesLabel;
        private ListBox archivesListBox;
        private Button deleteButton;
        private Label fsReadyLabel;

        private Panel rightPanel;
        private Panel toolbarPanel;
        private Label bufferLabel;
        private Button commitButton;
        private Button executeButton;
        private TextBox scriptEditor;
        private Label statsLabel;

        public ConsolePanel()
        {
            InitializeComponent();
            ApplyTheme();

            scriptEditor.Text = _defaultCode;
            UpdateStats();

            this.Load += ConsolePanel_Load;
        }

        private void InitializeComponent()
        {
            this.leftPanel = new Panel();
            this.storageLabel = new Label();
            this.archivesLabel = new Label();
            this.archivesListBox = new ListBox();
            this.deleteButton = new Button();
            this.fsReadyLabel = new Label();

            this.rightPanel = new Panel();
            this.toolbarPanel = new Panel();
            this.bufferLabel = new Label();
            this.commitButton = new Button();
            this.executeButton = new Button();
            this.scriptEditor = new TextBox();
            this.statsLabel = new Label();

            this.leftPanel.SuspendLayout();
            this.rightPanel.SuspendLayout();
            this.toolbarPanel.SuspendLayout();
            this.SuspendLayout();

            // 
            // leftPanel
            // 
            this.leftPanel.Controls.Add(this.fsReadyLabel);
            this.leftPanel.Controls.Add(this.deleteButton);
            this.leftPanel.Controls.Add(this.archivesListBox);
            this.leftPanel.Controls.Add(this.archivesLabel);
            this.leftPanel.Controls.Add(this.storageLabel);
            this.leftPanel.Dock = DockStyle.Left;
            this.leftPanel.Location = new Point(0, 0);
            this.leftPanel.Name = "leftPanel";
            this.leftPanel.Size = new Size(260, 592);
            this.leftPanel.TabIndex = 0;

            // 
            // storageLabel
            // 
            this.storageLabel.AutoSize = true;
            this.storageLabel.Location = new Point(24, 24);
            this.storageLabel.Name = "storageLabel";
            this.storageLabel.Size = new Size(58, 15);
            this.storageLabel.Text = "STORAGE";

            // 
            // archivesLabel
            // 
            this.archivesLabel.AutoSize = true;
            this.archivesLabel.Location = new Point(24, 44);
            this.archivesLabel.Name = "archivesLabel";
            this.archivesLabel.Size = new Size(76, 25);
            this.archivesLabel.Text = "Archives";

            // 
            // archivesListBox
            // 
            this.archivesListBox.Anchor = AnchorStyles.Top | AnchorStyles.Bottom | AnchorStyles.Left | AnchorStyles.Right;
            this.archivesListBox.BorderStyle = BorderStyle.FixedSingle;
            this.archivesListBox.DrawMode = DrawMode.OwnerDrawFixed;
            this.archivesListBox.ItemHeight = 24;
            this.archivesListBox.Location = new Point(24, 90);
            this.archivesListBox.Name = "archivesListBox";
            this.archivesListBox.Size = new Size(212, 410);
            this.archivesListBox.TabIndex = 2;
            this.archivesListBox.SelectedIndexChanged += new EventHandler(this.ArchivesListBox_SelectedIndexChanged);
            this.archivesListBox.DrawItem += new DrawItemEventHandler(this.ArchivesListBox_DrawItem);

            // 
            // deleteButton
            // 
            this.deleteButton.Anchor = AnchorStyles.Bottom | AnchorStyles.Left | AnchorStyles.Right;
            this.deleteButton.Location = new Point(24, 508);
            this.deleteButton.Name = "deleteButton";
            this.deleteButton.Size = new Size(212, 28);
            this.deleteButton.TabIndex = 3;
            this.deleteButton.Text = "Delete Selected";
            this.deleteButton.UseVisualStyleBackColor = true;
            this.deleteButton.Click += new EventHandler(this.DeleteButton_Click);

            // 
            // fsReadyLabel
            // 
            this.fsReadyLabel.Anchor = AnchorStyles.Bottom | AnchorStyles.Left;
            this.fsReadyLabel.AutoSize = true;
            this.fsReadyLabel.Location = new Point(24, 552);
            this.fsReadyLabel.Name = "fsReadyLabel";
            this.fsReadyLabel.Size = new Size(99, 15);
            this.fsReadyLabel.Text = "Filesystem Ready";

            // 
            // rightPanel
            // 
            this.rightPanel.Controls.Add(this.scriptEditor);
            this.rightPanel.Controls.Add(this.statsLabel);
            this.rightPanel.Controls.Add(this.toolbarPanel);
            this.rightPanel.Dock = DockStyle.Fill;
            this.rightPanel.Location = new Point(260, 0);
            this.rightPanel.Name = "rightPanel";
            this.rightPanel.Size = new Size(608, 592);
            this.rightPanel.TabIndex = 1;

            // 
            // toolbarPanel
            // 
            this.toolbarPanel.Controls.Add(this.executeButton);
            this.toolbarPanel.Controls.Add(this.commitButton);
            this.toolbarPanel.Controls.Add(this.bufferLabel);
            this.toolbarPanel.Dock = DockStyle.Top;
            this.toolbarPanel.Location = new Point(0, 0);
            this.toolbarPanel.Name = "toolbarPanel";
            this.toolbarPanel.Size = new Size(608, 50);
            this.toolbarPanel.TabIndex = 0;

            // 
            // bufferLabel
            // 
            this.bufferLabel.AutoSize = true;
            this.bufferLabel.Location = new Point(16, 18);
            this.bufferLabel.Name = "bufferLabel";
            this.bufferLabel.Size = new Size(64, 15);
            this.bufferLabel.Text = "scratchpad";

            // 
            // commitButton
            // 
            this.commitButton.Anchor = AnchorStyles.Top | AnchorStyles.Right;
            this.commitButton.Location = new Point(410, 11);
            this.commitButton.Name = "commitButton";
            this.commitButton.Size = new Size(80, 28);
            this.commitButton.TabIndex = 1;
            this.commitButton.Text = "Commit";
            this.commitButton.UseVisualStyleBackColor = true;
            this.commitButton.Click += new EventHandler(this.CommitButton_Click);

            // 
            // executeButton
            // 
            this.executeButton.Anchor = AnchorStyles.Top | AnchorStyles.Right;
            this.executeButton.Location = new Point(502, 11);
            this.executeButton.Name = "executeButton";
            this.executeButton.Size = new Size(90, 28);
            this.executeButton.TabIndex = 2;
            this.executeButton.Text = "Execute";
            this.executeButton.UseVisualStyleBackColor = true;
            this.executeButton.Click += new EventHandler(this.ExecuteButton_Click);

            // 
            // scriptEditor
            // 
            this.scriptEditor.AcceptsReturn = true;
            this.scriptEditor.AcceptsTab = true;
            this.scriptEditor.BorderStyle = BorderStyle.None;
            this.scriptEditor.Dock = DockStyle.Fill;
            this.scriptEditor.Multiline = true;
            this.scriptEditor.Name = "scriptEditor";
            this.scriptEditor.ScrollBars = ScrollBars.Vertical;
            this.scriptEditor.Size = new Size(608, 510);
            this.scriptEditor.TabIndex = 1;
            this.scriptEditor.TextChanged += new EventHandler(this.ScriptEditor_TextChanged);

            // 
            // statsLabel
            // 
            this.statsLabel.Dock = DockStyle.Bottom;
            this.statsLabel.Location = new Point(0, 560);
            this.statsLabel.Name = "statsLabel";
            this.statsLabel.Padding = new Padding(16, 0, 0, 0);
            this.statsLabel.Size = new Size(608, 32);
            this.statsLabel.Text = "Lines: 1  Chars: 0";
            this.statsLabel.TextAlign = ContentAlignment.MiddleLeft;

            // 
            // ConsolePanel
            // 
            this.BackColor = Color.FromArgb(5, 5, 5);
            this.Controls.Add(this.rightPanel);
            this.Controls.Add(this.leftPanel);
            this.Name = "ConsolePanel";
            this.Size = new Size(868, 592);

            this.leftPanel.ResumeLayout(false);
            this.leftPanel.PerformLayout();
            this.rightPanel.ResumeLayout(false);
            this.rightPanel.PerformLayout();
            this.toolbarPanel.ResumeLayout(false);
            this.toolbarPanel.PerformLayout();
            this.ResumeLayout(false);
        }

        private void ApplyTheme()
        {
            this.BackColor = Theme.Background;
            this.ForeColor = Theme.TextPrimary;

            // Left panel
            this.leftPanel.BackColor = Theme.Background;
            Theme.StyleLabel(this.storageLabel, Theme.FontMonospace, Theme.TextSecondary);
            Theme.StyleLabel(this.archivesLabel, Theme.FontTitle, Theme.TextPrimary);
            Theme.StyleLabel(this.fsReadyLabel, Theme.FontMonospace, Theme.TextMuted);

            this.archivesListBox.BackColor = Theme.Background;
            this.archivesListBox.ForeColor = Theme.TextPrimary;
            this.archivesListBox.Font = Theme.FontBody;

            Theme.StyleFlatButton(this.deleteButton, Theme.Card, Theme.TextSecondary, Theme.Border);

            // Left panel border on paint
            this.leftPanel.Paint += (s, e) =>
            {
                using (var pen = new Pen(Theme.Border, 1))
                {
                    e.Graphics.DrawLine(pen, leftPanel.Width - 1, 0, leftPanel.Width - 1, leftPanel.Height);
                }
            };

            // Right panel
            this.rightPanel.BackColor = Theme.Background;
            this.toolbarPanel.BackColor = Theme.Background;
            Theme.StyleLabel(this.bufferLabel, Theme.FontMonospace, Theme.TextSecondary);
            Theme.StyleLabel(this.statsLabel, Theme.FontMonospace, Theme.TextMuted);

            this.scriptEditor.BackColor = Color.FromArgb(2, 2, 2);
            this.scriptEditor.ForeColor = Theme.TextPrimary;
            this.scriptEditor.Font = Theme.FontMonospace;

            Theme.StyleFlatButton(this.commitButton, Theme.Card, Theme.TextSecondary, Theme.Border);
            Theme.StyleFlatButton(this.executeButton, Color.White, Color.Black);

            this.toolbarPanel.Paint += (s, e) =>
            {
                using (var pen = new Pen(Theme.Border, 1))
                {
                    e.Graphics.DrawLine(pen, 0, toolbarPanel.Height - 1, toolbarPanel.Width, toolbarPanel.Height - 1);
                }
            };
            this.statsLabel.Paint += (s, e) =>
            {
                using (var pen = new Pen(Theme.Border, 1))
                {
                    e.Graphics.DrawLine(pen, 0, 0, statsLabel.Width, 0);
                }
            };
        }

        private void ConsolePanel_Load(object? sender, EventArgs e)
        {
            LoadScripts();
        }

        private string GetScriptsFilePath()
        {
            string appData = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData);
            string folder = Path.Combine(appData, "LightingControl");
            Directory.CreateDirectory(folder);
            return Path.Combine(folder, "scripts.json");
        }

        private void LoadScripts()
        {
            try
            {
                string path = GetScriptsFilePath();
                if (File.Exists(path))
                {
                    string json = File.ReadAllText(path);
                    var list = JsonSerializer.Deserialize<List<ScriptEntry>>(json);
                    if (list != null)
                    {
                        _scripts = list;
                        RefreshArchivesList();
                    }
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to load scripts: {ex.Message}");
            }
        }

        private void PersistScripts()
        {
            try
            {
                string path = GetScriptsFilePath();
                string json = JsonSerializer.Serialize(_scripts, new JsonSerializerOptions { WriteIndented = true });
                File.WriteAllText(path, json);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to save scripts: {ex.Message}");
            }
        }

        private void RefreshArchivesList()
        {
            archivesListBox.Items.Clear();
            foreach (var s in _scripts)
            {
                archivesListBox.Items.Add(s.Name);
            }
        }

        private void ArchivesListBox_SelectedIndexChanged(object? sender, EventArgs e)
        {
            int idx = archivesListBox.SelectedIndex;
            if (idx >= 0 && idx < _scripts.Count)
            {
                var script = _scripts[idx];
                _activeScript = script;
                scriptEditor.Text = script.Content;
                bufferLabel.Text = $"active_buffer ({script.Name})";
            }
        }

        private void CommitButton_Click(object? sender, EventArgs e)
        {
            string name = ShowInputDialog("Commit Script Identifier", "e.g., test_sweep");
            if (string.IsNullOrEmpty(name)) return;

            var entry = new ScriptEntry
            {
                Id = Guid.NewGuid().ToString(),
                Name = name,
                Content = scriptEditor.Text
            };

            _scripts.Insert(0, entry);
            PersistScripts();
            RefreshArchivesList();
            
            archivesListBox.SelectedIndex = 0;
        }

        private void ExecuteButton_Click(object? sender, EventArgs e)
        {
            MessageBox.Show(
                "Sequence parsed successfully. Applying intercepts...\n\n[Mock Execution Complete]",
                "Execute Script",
                MessageBoxButtons.OK,
                MessageBoxIcon.Information);
        }

        private void DeleteButton_Click(object? sender, EventArgs e)
        {
            int idx = archivesListBox.SelectedIndex;
            if (idx >= 0 && idx < _scripts.Count)
            {
                var script = _scripts[idx];
                _scripts.RemoveAt(idx);
                PersistScripts();
                RefreshArchivesList();

                if (_activeScript?.Id == script.Id)
                {
                    _activeScript = null;
                    scriptEditor.Text = _defaultCode;
                    bufferLabel.Text = "scratchpad";
                }
            }
        }

        private void ScriptEditor_TextChanged(object? sender, EventArgs e)
        {
            UpdateStats();
        }

        private void UpdateStats()
        {
            int lines = scriptEditor.Text.Split('\n').Length;
            int chars = scriptEditor.Text.Length;
            statsLabel.Text = $"Lines: {lines}  Chars: {chars}";
        }

        private void ArchivesListBox_DrawItem(object? sender, DrawItemEventArgs e)
        {
            if (e.Index < 0) return;

            e.DrawBackground();

            bool isSelected = (e.State & DrawItemState.Selected) == DrawItemState.Selected;
            System.Drawing.Color backColor = isSelected ? Color.FromArgb(20, 20, 23) : Theme.Background;
            System.Drawing.Color textColor = isSelected ? Theme.TextPrimary : Theme.TextSecondary;

            using (var brush = new SolidBrush(backColor))
            {
                e.Graphics.FillRectangle(brush, e.Bounds);
            }

            string text = archivesListBox.Items[e.Index].ToString() ?? "";
            using (var brush = new SolidBrush(textColor))
            {
                // Align vertically centered
                var format = new StringFormat
                {
                    LineAlignment = StringAlignment.Center,
                    Alignment = StringAlignment.Near
                };
                e.Graphics.DrawString(text, Theme.FontBody, brush, e.Bounds, format);
            }

            e.DrawFocusRectangle();
        }

        private string ShowInputDialog(string title, string prompt, string defaultVal = "")
        {
            Form promptForm = new Form()
            {
                Width = 360,
                Height = 160,
                FormBorderStyle = FormBorderStyle.FixedDialog,
                Text = title,
                StartPosition = FormStartPosition.CenterParent,
                BackColor = Theme.Background,
                ForeColor = Theme.TextPrimary,
                MaximizeBox = false,
                MinimizeBox = false
            };
            
            int darkMode = 1;
            DwmSetWindowAttribute(promptForm.Handle, 20, ref darkMode, sizeof(int));

            Label textLabel = new Label() { Left = 20, Top = 16, Width = 300, Text = prompt };
            Theme.StyleLabel(textLabel, Theme.FontBody, Theme.TextPrimary);

            TextBox textBox = new TextBox() { Left = 20, Top = 40, Width = 300, Text = defaultVal };
            Theme.StyleTextBox(textBox);

            Button confirmation = new Button() { Text = "Commit", Left = 220, Width = 100, Top = 80, DialogResult = DialogResult.OK };
            Button cancel = new Button() { Text = "Cancel", Left = 110, Width = 100, Top = 80, DialogResult = DialogResult.Cancel };
            Theme.StyleFlatButton(confirmation, Color.White, Color.Black);
            Theme.StyleFlatButton(cancel, Theme.Card, Theme.TextSecondary, Theme.Border);

            promptForm.Controls.Add(textBox);
            promptForm.Controls.Add(textLabel);
            promptForm.Controls.Add(confirmation);
            promptForm.Controls.Add(cancel);
            promptForm.AcceptButton = confirmation;
            promptForm.CancelButton = cancel;

            return promptForm.ShowDialog() == DialogResult.OK ? textBox.Text : string.Empty;
        }
    }
}
