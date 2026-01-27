"use client"; // Required for App Router client components

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'; // Use the official import
import KeyboardDisplay from '@/app/components/Keyboard';

export default function Home() {
  const [status, setStatus] = useState("Initializing...");

  useEffect(() => {
    // This ensures the code ONLY runs in the browser
    const initLighting = async () => {
      try {
        await invoke("test_set_red");
        setStatus("Red set successfully!");
      } catch (err) {
        console.error("Failed to invoke:", err);
        setStatus("Failed to connect to backend.");
      }
    };

    initLighting();
  }, []); // Empty dependency array means it runs once on mount

  return (
    <main>
      <h1>LOQ RGB Controller</h1>
      <p>Status: {status}</p>
      {/* <KeyboardDisplay /> */}
      
    </main>
  );
}