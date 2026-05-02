import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { debugAction } from '../lib/logger';
export interface HiddenInputProps { modelValue?: string; placeholder?: string; onUpdateModelValue?: (value: string) => void; }
export default function HiddenInput({ modelValue = '', placeholder = '', onUpdateModelValue }: HiddenInputProps) { const { t }=useTranslation(); const [shown,setShown]=useState(false); return <div className="input-group" data-source="components/HiddenInput.vue"><input className="form-control" type={shown?'text':'password'} value={modelValue} placeholder={placeholder} onChange={(e)=>onUpdateModelValue?.(e.target.value)} /><button type="button" className="btn" onClick={()=>{debugAction('HiddenInput','toggle.visibility',{shown:!shown}); setShown(!shown);}}>{shown?t('Hide'):t('Show')}</button></div>; }
