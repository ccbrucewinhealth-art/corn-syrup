import type { ReactNode } from 'react';
export interface TooltipProps { text?: string; children?: ReactNode; }
export default function Tooltip({ text = '', children }: TooltipProps) { return <span title={text} data-source="components/Tooltip.vue">{children ?? text}</span>; }
