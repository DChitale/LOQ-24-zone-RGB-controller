// import { useState, useRef, useEffect } from "react";
// import { ChevronDown } from "lucide-react";
// import { motion, AnimatePresence } from "framer-motion";

// export default function DropdownMenu({
//   label = "Select option",
//   options = [],
//   onSelect,
// }) {
//   const [open, setOpen] = useState(false);
//   const [selected, setSelected] = useState(null);
//   const ref = useRef<HTMLDivElement | null>(null);

//   useEffect(() => {
//     function handleClickOutside(e: MouseEvent) {
//       if (ref.current && !ref.current.contains(e.target)) {
//         setOpen(false);
//       }
//     }
//     document.addEventListener("mousedown", handleClickOutside);
//     return () => document.removeEventListener("mousedown", handleClickOutside);
//   }, []);

//   function handleSelect(option) {
//     setSelected(option);
//     setOpen(false);
//     onSelect?.(option);
//   }

//   return (
//     <div ref={ref} className="relative inline-block w-64">
//       <button
//         onClick={() => setOpen((o) => !o)}
//         className="w-full flex items-center justify-between rounded-2xl border border-border bg-background px-4 py-2 text-sm shadow-sm hover:bg-muted transition"
//       >
//         <span className="truncate">
//           {selected ? selected.label : label}
//         </span>
//         <ChevronDown
//           className={`h-4 w-4 opacity-60 transition-transform ${open ? "rotate-180" : ""}`}
//         />
//       </button>

//       <AnimatePresence>
//         {open && (
//           <motion.div
//             initial={{ opacity: 0, y: -6 }}
//             animate={{ opacity: 1, y: 0 }}
//             exit={{ opacity: 0, y: -6 }}
//             transition={{ duration: 0.15, ease: "easeOut" }}
//             className="absolute z-50 mt-2 w-full"
//           >
//             <div className="rounded-2xl border border-border bg-background shadow-lg overflow-hidden">
//               <ul className="max-h-60 overflow-auto">
//                 {options.map((option) => (
//                   <li
//                     key={option.value}
//                     onClick={() => handleSelect(option)}
//                     className="cursor-pointer px-4 py-2 text-sm hover:bg-muted transition-colors"
//                   >
//                     {option.label}
//                   </li>
//                 ))}
//               </ul>
//             </div>
//           </motion.div>
//         )}
//       </AnimatePresence>
//     </div>
//   );
// }
