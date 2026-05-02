import { useState } from 'react';
import { FormSection, setByPath, type AnyRecord } from '../common';
import { debugAction } from '../../lib/logger';
export interface ThreeSixtymessengerProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; }
export default function ThreeSixtymessenger({ model = {}, onChange }: ThreeSixtymessengerProps) { const [draft,setDraft]=useState<AnyRecord>(model); const update=(name:string,value:any)=>{const next={...draft}; setByPath(next,name,value); setDraft(next); debugAction('360messenger','payload.map',{name,value}); onChange?.(next);}; return <FormSection scope="components/notifications/360messenger.vue" model={draft} onChange={update} fields={[{name:'apiKey',label:'API Key',type:'password',required:true},{name:'recipient',label:'Recipient',type:'text',required:true},{name:'senderName',label:'Sender Name',type:'text'}]} />; }
