using System;
using System.Drawing;
using System.Windows.Forms;

namespace RGBController
{
    public static class Theme
    {
        public static readonly Color Background = Color.FromArgb(5, 5, 5); // #050505
        public static readonly Color Card = Color.FromArgb(9, 9, 11); // #09090b
        public static readonly Color Border = Color.FromArgb(24, 24, 27); // #18181b
        public static readonly Color Accent = Color.FromArgb(255, 255, 255); // #ffffff
        
        public static readonly Color TextPrimary = Color.FromArgb(255, 255, 255); // #ffffff
        public static readonly Color TextSecondary = Color.FromArgb(161, 161, 170); // #a1a1aa
        public static readonly Color TextMuted = Color.FromArgb(82, 82, 91); // #52525b

        public static readonly Font FontTitle = GetFont("Segoe UI", 14f, FontStyle.Regular, new Font("Segoe UI", 14f, FontStyle.Regular));
        public static readonly Font FontHeader = GetFont("Segoe UI", 10.5f, FontStyle.Regular, new Font("Segoe UI", 10.5f, FontStyle.Regular));
        public static readonly Font FontBody = GetFont("Segoe UI", 9f, FontStyle.Regular, new Font("Segoe UI", 9f, FontStyle.Regular));
        public static readonly Font FontMonospace = new Font("Consolas", 9f, FontStyle.Regular);

        private static Font GetFont(string familyName, float size, FontStyle style, Font fallback)
        {
            try
            {
                using (var test = new Font(familyName, size, style))
                {
                    if (test.Name.Equals(familyName, StringComparison.OrdinalIgnoreCase))
                    {
                        return new Font(familyName, size, style);
                    }
                }
            }
            catch
            {
                // Fallback
            }
            return fallback;
        }

        public static void StyleFlatButton(Button btn, Color backColor, Color foreColor, Color? borderColor = null)
        {
            btn.FlatStyle = FlatStyle.Flat;
            btn.BackColor = backColor;
            btn.ForeColor = foreColor;
            btn.FlatAppearance.BorderSize = borderColor.HasValue ? 1 : 0;
            if (borderColor.HasValue)
            {
                btn.FlatAppearance.BorderColor = borderColor.Value;
            }
            btn.FlatAppearance.MouseOverBackColor = Color.FromArgb(
                Math.Min(255, backColor.R + 20),
                Math.Min(255, backColor.G + 20),
                Math.Min(255, backColor.B + 20)
            );
            btn.FlatAppearance.MouseDownBackColor = Color.FromArgb(
                Math.Max(0, backColor.R - 20),
                Math.Max(0, backColor.G - 20),
                Math.Max(0, backColor.B - 20)
            );
            btn.Font = FontBody;
        }

        public static void StyleTextBox(TextBox txt)
        {
            txt.BackColor = Card;
            txt.ForeColor = TextPrimary;
            txt.BorderStyle = BorderStyle.FixedSingle;
            txt.Font = FontMonospace;
        }

        public static void StyleLabel(Label lbl, Font font, Color foreColor)
        {
            lbl.Font = font;
            lbl.ForeColor = foreColor;
            lbl.BackColor = Color.Transparent;
        }
    }
}
