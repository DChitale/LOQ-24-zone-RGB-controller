import Sidebar from '@/app/components/Sidebar';
import './globals.css';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="flex h-screen bg-[#050505] text-white overflow-hidden">
        <Sidebar />
        {/* 'flex-1' takes all remaining horizontal space.
            'relative' allows the keyboard to scale within it.
        */}
        <main className="flex-1 relative overflow-hidden">
          {children}
        </main>
      </body>
    </html>
  );
}