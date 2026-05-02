export interface TagProps { name?: string; color?: string; value?: string; }
export default function Tag({ name = '', value = '', color = '#64748b' }: TagProps) { return <span className="badge" style={{ backgroundColor: color }} data-source="components/Tag.vue">{name || value}</span>; }
