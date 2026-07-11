#pragma warning disable CS8618

using System;
using System.Drawing;
using System.Globalization;
using System.Windows.Forms;
using RGBController.Interop;

namespace RGBController.Controls
{
    public class HardwarePanel : UserControl
    {
        private Label titleLabel;
        private Label descLabel;
        private Label vidLabel;
        private TextBox vidTextBox;
        private Label pidLabel;
        private TextBox pidTextBox;
        private Button connectButton;
        private Panel statusPanel;
        private Label statusHeaderLabel;
        private Label statusTextLabel;

        public HardwarePanel()
        {
            InitializeComponent();
            ApplyTheme();
        }

        private void InitializeComponent()
        {
            this.titleLabel = new Label();
            this.descLabel = new Label();
            this.vidLabel = new Label();
            this.vidTextBox = new TextBox();
            this.pidLabel = new Label();
            this.pidTextBox = new TextBox();
            this.connectButton = new Button();
            this.statusPanel = new Panel();
            this.statusHeaderLabel = new Label();
            this.statusTextLabel = new Label();

            this.SuspendLayout();

            // 
            // titleLabel
            // 
            this.titleLabel.AutoSize = true;
            this.titleLabel.Location = new Point(24, 24);
            this.titleLabel.Name = "titleLabel";
            this.titleLabel.Size = new Size(130, 25);
            this.titleLabel.Text = "HARDWARE";

            // 
            // descLabel
            // 
            this.descLabel.AutoSize = true;
            this.descLabel.Location = new Point(24, 54);
            this.descLabel.Name = "descLabel";
            this.descLabel.Size = new Size(295, 15);
            this.descLabel.Text = "Configure physical USB connection parameters for the controller.";

            // 
            // vidLabel
            // 
            this.vidLabel.AutoSize = true;
            this.vidLabel.Location = new Point(24, 100);
            this.vidLabel.Name = "vidLabel";
            this.vidLabel.Size = new Size(106, 15);
            this.vidLabel.Text = "VENDOR ID (HEX)";

            // 
            // vidTextBox
            // 
            this.vidTextBox.Location = new Point(24, 120);
            this.vidTextBox.Name = "vidTextBox";
            this.vidTextBox.Size = new Size(120, 23);
            this.vidTextBox.TabIndex = 1;
            this.vidTextBox.Text = "048D";

            // 
            // pidLabel
            // 
            this.pidLabel.AutoSize = true;
            this.pidLabel.Location = new Point(170, 100);
            this.pidLabel.Name = "pidLabel";
            this.pidLabel.Size = new Size(116, 15);
            this.pidLabel.Text = "PRODUCT ID (HEX)";

            // 
            // pidTextBox
            // 
            this.pidTextBox.Location = new Point(170, 120);
            this.pidTextBox.Name = "pidTextBox";
            this.pidTextBox.Size = new Size(120, 23);
            this.pidTextBox.TabIndex = 2;
            this.pidTextBox.Text = "C195";

            // 
            // connectButton
            // 
            this.connectButton.Location = new Point(24, 164);
            this.connectButton.Name = "connectButton";
            this.connectButton.Size = new Size(266, 32);
            this.connectButton.TabIndex = 3;
            this.connectButton.Text = "Initialize Interface";
            this.connectButton.UseVisualStyleBackColor = true;
            this.connectButton.Click += new EventHandler(this.ConnectButton_Click);

            // 
            // statusPanel
            // 
            this.statusPanel.Anchor = AnchorStyles.Top | AnchorStyles.Left | AnchorStyles.Right;
            this.statusPanel.BorderStyle = BorderStyle.FixedSingle;
            this.statusPanel.Controls.Add(this.statusTextLabel);
            this.statusPanel.Controls.Add(this.statusHeaderLabel);
            this.statusPanel.Location = new Point(24, 220);
            this.statusPanel.Name = "statusPanel";
            this.statusPanel.Size = new Size(820, 80);
            this.statusPanel.TabIndex = 4;
            this.statusPanel.Visible = false;

            // 
            // statusHeaderLabel
            // 
            this.statusHeaderLabel.AutoSize = true;
            this.statusHeaderLabel.Location = new Point(12, 12);
            this.statusHeaderLabel.Name = "statusHeaderLabel";
            this.statusHeaderLabel.Size = new Size(122, 15);
            this.statusHeaderLabel.Text = "INTERFACE STATUS";

            // 
            // statusTextLabel
            // 
            this.statusTextLabel.Anchor = AnchorStyles.Top | AnchorStyles.Left | AnchorStyles.Right;
            this.statusTextLabel.Location = new Point(12, 32);
            this.statusTextLabel.Name = "statusTextLabel";
            this.statusTextLabel.Size = new Size(794, 36);
            this.statusTextLabel.Text = "Interface ready.";

            // 
            // HardwarePanel
            // 
            this.BackColor = Color.FromArgb(5, 5, 5);
            this.Controls.Add(this.statusPanel);
            this.Controls.Add(this.connectButton);
            this.Controls.Add(this.pidTextBox);
            this.Controls.Add(this.pidLabel);
            this.Controls.Add(this.vidTextBox);
            this.Controls.Add(this.vidLabel);
            this.Controls.Add(this.descLabel);
            this.Controls.Add(this.titleLabel);
            this.Name = "HardwarePanel";
            this.Size = new Size(868, 592);
            this.statusPanel.ResumeLayout(false);
            this.statusPanel.PerformLayout();
            this.ResumeLayout(false);
            this.PerformLayout();
        }

        private void ApplyTheme()
        {
            this.BackColor = Theme.Background;
            this.ForeColor = Theme.TextPrimary;

            Theme.StyleLabel(this.titleLabel, Theme.FontTitle, Theme.TextPrimary);
            Theme.StyleLabel(this.descLabel, Theme.FontBody, Theme.TextSecondary);
            Theme.StyleLabel(this.vidLabel, Theme.FontMonospace, Theme.TextSecondary);
            Theme.StyleLabel(this.pidLabel, Theme.FontMonospace, Theme.TextSecondary);
            Theme.StyleLabel(this.statusHeaderLabel, Theme.FontMonospace, Theme.TextSecondary);
            Theme.StyleLabel(this.statusTextLabel, Theme.FontBody, Theme.TextPrimary);

            Theme.StyleTextBox(this.vidTextBox);
            Theme.StyleTextBox(this.pidTextBox);

            Theme.StyleFlatButton(this.connectButton, Color.White, Color.Black);
            
            this.statusPanel.BackColor = Theme.Card;
            this.statusPanel.Paint += (s, e) =>
            {
                using (var pen = new Pen(Theme.Border, 1))
                {
                    e.Graphics.DrawRectangle(pen, 0, 0, statusPanel.Width - 1, statusPanel.Height - 1);
                }
            };
        }

        private void ConnectButton_Click(object? sender, EventArgs e)
        {
            statusPanel.Visible = true;
            try
            {
                string vidStr = vidTextBox.Text.Trim();
                string pidStr = pidTextBox.Text.Trim();

                if (ushort.TryParse(vidStr, NumberStyles.HexNumber, CultureInfo.InvariantCulture, out ushort vid) &&
                    ushort.TryParse(pidStr, NumberStyles.HexNumber, CultureInfo.InvariantCulture, out ushort pid))
                {
                    int result = RgbInterop.rgb_connect_keyboard(vid, pid);
                    if (result == 1)
                    {
                        statusTextLabel.ForeColor = Theme.TextPrimary;
                        statusTextLabel.Text = $"Success: device connected with VID 0x{vidStr.ToUpper()} and PID 0x{pidStr.ToUpper()}";
                    }
                    else
                    {
                        statusTextLabel.ForeColor = Color.IndianRed;
                        statusTextLabel.Text = $"Error: failed to connect to device with VID 0x{vidStr.ToUpper()} and PID 0x{pidStr.ToUpper()}";
                    }
                }
                else
                {
                    statusTextLabel.ForeColor = Color.IndianRed;
                    statusTextLabel.Text = "Error: Vendor ID and Product ID must be valid 4-digit hexadecimal numbers.";
                }
            }
            catch (Exception ex)
            {
                statusTextLabel.ForeColor = Color.IndianRed;
                statusTextLabel.Text = $"Error: {ex.Message}";
            }
        }
    }
}
