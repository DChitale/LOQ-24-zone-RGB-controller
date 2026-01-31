import Sidebar from '@/app/components/Sidebar';
import './globals.css';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className="bg-[#050505]">
      <body className="flex h-screen bg-[#050505] text-white overflow-hidden selection:bg-zinc-800">
        <Sidebar />
        
        {/* 'flex-1' fills the remaining width.
            'overflow-y-auto' enables the themed scrollbar here.
        */}
        <main className="flex-1 h-screen overflow-y-auto overflow-x-hidden relative">
          {children}
        </main>
      </body>
    </html>
  );
}