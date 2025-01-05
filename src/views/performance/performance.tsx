import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef, useState } from 'react';
import './performance.scss';

interface ISystemInfoItem {
  prop: string;
  value: Partial<string | number | Array<string>>
}

function PerformanceInfo() {

  const isInit = useRef(true);
  const [systemInfo, setSystemInfo] = useState<ISystemInfoItem[]>([]);

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
      if(Array.isArray(res[key])) {
        const values = [];
        let index = 0;
        for(const item of res[key]) {
          let str = '';
          index += 1;
          for(const itemKey in item) {
            str += `${itemKey}: ${item[itemKey] || '--'}, `;
          }
          values.push(str);
        }
        list.push({
          prop: key,
          value: values
        });
      } else {
        list.push({
          prop: key,
          value: res[key]
        });
      }
      
    }
    setSystemInfo(list);
  }

  return <div className="view__performance">
    <div className='info-list-wrap'>
      {systemInfo.map(item => (
        <div key={item.prop} className='info-list-item'>
          <span className='item-prop'>{item.prop}</span>
          <span className='item-value'>{Array.isArray(item.value) ? item.value.map((v, vIndex) => (
            <div key={`${v}_${vIndex}`}>{v}</div>
          )) : item.value}</span>
        </div>
      ))}
    </div>
  </div>
}

export default PerformanceInfo;