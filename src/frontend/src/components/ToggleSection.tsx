import type { ReactNode } from 'react';
import { useState } from 'react';
import { debugAction } from '../lib/logger';
export interface ToggleSectionProps { title?: string; defaultOpen?: boolean; children?: ReactNode; }
export default function ToggleSection({ title = 'Section', defaultOpen = false, children }: ToggleSectionProps) { const [open,setOpen]=useState(defaultOpen); return <section className="card" data-source="components/ToggleSection.vue"><button type="button" className="btn" onClick={()=>{debugAction('ToggleSection','toggle',{open:!open}); setOpen(!open);}}>{open?'▾':'▸'} {title}</button>{open && <div className="my-3">{children}</div>}</section>; }
