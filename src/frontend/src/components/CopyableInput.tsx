import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { debugAction, logger } from '../lib/logger';
export interface CopyableInputProps { modelValue?: string; disabled?: boolean; onUpdateModelValue?: (value: string) => void; }
export default function CopyableInput({ modelValue = '', disabled = false, onUpdateModelValue }: CopyableInputProps) {
  const { t } = useTranslation(); const [copied,setCopied]=useState(false);
  const copy=async()=>{ try { await navigator.clipboard.writeText(modelValue); setCopied(true); debugAction('CopyableInput','clipboard.copy.success'); } catch(e) { logger.warn('[CopyableInput] clipboard.copy.failed', e); } };
  return <div className="input-group" data-source="components/CopyableInput.vue"><input className="form-control" value={modelValue} disabled={disabled} onChange={(e)=>onUpdateModelValue?.(e.target.value)} /><button type="button" className="btn btn-outline-primary" onClick={copy}>{copied?t('Copied'):t('Copy')}</button></div>;
}
