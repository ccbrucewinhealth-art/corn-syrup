import dayjs from 'dayjs';
import { debugAction } from '../lib/logger';
export interface DatetimeProps { value?: string | number | Date | null; dateOnly?: boolean; }
export default function Datetime({ value = null, dateOnly = false }: DatetimeProps) {
  const text = value ? dayjs(value).format(dateOnly ? 'YYYY-MM-DD' : 'YYYY-MM-DD HH:mm:ss') : '';
  debugAction('Datetime','format', { value, dateOnly, text });
  return <span data-source="components/Datetime.vue">{text}</span>;
}
