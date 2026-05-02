import { useEffect, useState } from 'react';
import { debugAction } from '../lib/logger';
export interface CountUpProps { value?: number; durationMs?: number; formatter?: (value: number) => string; }
export default function CountUp({ value = 0, durationMs = 600, formatter = (v) => Math.round(v).toString() }: CountUpProps) {
  const [display, setDisplay] = useState(value);
  useEffect(() => { const start = display; const diff = value - start; const from = performance.now(); debugAction('CountUp','animate.start',{start,value}); let id=0; const tick=(now:number)=>{const p=Math.min(1,(now-from)/durationMs); setDisplay(start+diff*p); if(p<1) id=requestAnimationFrame(tick);}; id=requestAnimationFrame(tick); return()=>cancelAnimationFrame(id); }, [value]);
  return <span data-source="components/CountUp.vue">{formatter(display)}</span>;
}
