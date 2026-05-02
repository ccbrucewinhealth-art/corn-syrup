import { debugAction } from '../lib/logger';
export interface Heartbeat { status?: number; ping?: number; time?: string; }
export interface HeartbeatBarProps { heartbeats?: Heartbeat[]; }
export default function HeartbeatBar({ heartbeats = [] }: HeartbeatBarProps) { debugAction('HeartbeatBar','render',{count:heartbeats.length}); return <div className="input-group" data-source="components/HeartbeatBar.vue">{heartbeats.map((h,i)=>{const c=h.status===1?'#22c55e':h.status===0?'#ef4444':h.status===3?'#7c3aed':'#f59e0b'; return <span key={i} title={`${h.time ?? ''} ${h.ping ?? ''}ms`} style={{display:'inline-block',width:8,height:24,background:c,borderRadius:3}} />;})}</div>; }
