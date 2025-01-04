import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef, useState } from 'react';


function PerformanceInfo() {

  const isInit = useRef(true);
  const [systemInfo, setSystemInfo] = useState<{ prop: string, value: string | number }[]>([]);

  useEffect(() => {
    if(isInit.current) {
      getSystemInfo();
      isInit.current = false;
    }
  }, []);

  async function getSystemInfo() {
    const res = await invoke('get_system_info').catch(() => ({})) as Record<string, string | number>;
    const list = [];
    for(const key in res) {
      list.push({
        prop: key,
        value: res[key]
      })
    }
    setSystemInfo(list);
  }

  return <div className="view__performance">
    <div>
      {systemInfo.map(item => (
        <div>
          <span>{item.prop}</span>
          <span>{item.value}</span>
        </div>
      ))}
    </div>
  </div>
}

export default PerformanceInfo;