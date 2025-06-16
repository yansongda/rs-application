export function substr(data: string, length?: number): string {
  const len = length || 12;

  return data.length > len ? `${data.substr(0, len)}...` : data;
}
