import { useTranslation } from 'react-i18next';
import { debugAction } from '../lib/logger';
export interface ConfirmProps { show?: boolean; title?: string; message?: string; yesText?: string; noText?: string; onYes?: () => void; onNo?: () => void; }
export default function Confirm({ show = true, title = 'Confirm', message = 'Are you sure?', yesText = 'Yes', noText = 'No', onYes, onNo }: ConfirmProps) { const { t }=useTranslation(); if(!show) return null; return <section className="card" data-source="components/Confirm.vue"><h2>{t(title)}</h2><p>{t(message)}</p><button className="btn btn-primary" onClick={()=>{debugAction('Confirm','yes'); onYes?.();}}>{t(yesText)}</button> <button className="btn" onClick={()=>{debugAction('Confirm','no'); onNo?.();}}>{t(noText)}</button></section>; }
