"use client";
import { Circle, Cpu, Paintbrush, Settings, Terminal } from 'lucide-react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { motion } from 'framer-motion';

export default function ClassySidebar() {
  const pathname = usePathname();

  const navLinks = [
    { id: 'Home', path: '/', icon: <Paintbrush size={18} /> },
    { id: 'Hardware', path: '/hardware', icon: <Cpu size={18} /> },
    { id: 'Console', path: '/console', icon: <Terminal size={18} /> },
    { id: 'Settings', path: '/settings', icon: <Settings size={18} /> },
  ];

  return (
    <aside className="w-24 lg:w-64 h-screen bg-[#0a0a0a] flex flex-col border-r border-zinc-900 shrink-0">
      {/* Brand */}
      <div className="h-24 flex items-center justify-center lg:justify-start lg:px-10">
        <Circle size={22} className="text-white fill-white" />
        <span className="ml-4 hidden lg:block font-medium tracking-[0.2em] text-white text-xs">CORE</span>
      </div>

      {/* Navigation using Href */}
      <nav className="flex-1 px-4 lg:px-6 space-y-6 pt-10">
        {navLinks.map((link) => (
          <Link
            key={link.id}
            href={link.path}
            className="w-full group flex items-center justify-center lg:justify-start transition-all"
          >
            <div className="relative flex items-center justify-center">
              <div className={`transition-all duration-300 ${pathname === link.path ? 'text-white' : 'text-zinc-600 group-hover:text-zinc-400'}`}>
                {link.icon}
              </div>
              {pathname === link.path && (
                <motion.div layoutId="dot" className="absolute -left-6 w-1 h-1 bg-white rounded-full hidden lg:block" />
              )}
            </div>
            <span className={`ml-6 hidden lg:block text-[11px] uppercase tracking-[0.15em] ${pathname === link.path ? 'text-white font-bold' : 'text-zinc-500 font-medium'}`}>
              {link.id}
            </span>
          </Link>
        ))}
      </nav>
    </aside>
  );
}