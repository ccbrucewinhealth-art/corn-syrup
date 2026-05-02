import ActionInput from './ActionInput';
export interface TemplatedInputProps { modelValue?: string; template?: string; placeholder?: string; onUpdateModelValue?: (value: string) => void; }
export default function TemplatedInput({ modelValue='', template='{{name}}', placeholder='', onUpdateModelValue }: TemplatedInputProps) { return <ActionInput modelValue={modelValue} placeholder={placeholder} icon="{}" actionAriaLabel="Insert template" action={()=>onUpdateModelValue?.(`${modelValue}${template}`)} onUpdateModelValue={onUpdateModelValue} />; }
