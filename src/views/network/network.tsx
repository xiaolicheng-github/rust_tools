import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef, useState } from "react";
import * as echarts from 'echarts';
import dayjs from 'dayjs';
import './network.scss';

function bytesToMB(bytes: number) {
  if (typeof bytes !== 'number' || bytes < 0) {
    throw new Error('Input must be a non-negative number');
  }
  const MB = 1024 * 1024;
  return Math.round((bytes / MB) * 100) / 100;; // 保留两位小数
}

function Network() {

  const chartRef = useRef(null);
  const chartInstance = useRef<echarts.ECharts | null>(null);
  const series = useRef([
    {
      name: 'Received',
      type: 'line',
      showSymbol: false,
      data: [] as { name: string; value: [number, number] }[]
    },
    {
      name: 'Transmitted',
      type: 'line',
      showSymbol: false,
      data: []
    }
  ]);
  const options = useRef({
    title: {
      text: '当前下载和上传的数据(MB/s)'
    },
    tooltip: {
      trigger: 'axis',
      formatter: function (params: any) {
        const received = params[0];
        const transmitted = params[1];
        return `<div>
          <div>${received.name}</div>
          <div>下载：${received.value[1]} MB/s</div>
          <div>上传：${transmitted.value[1]} MB/s</div>
        <div>`;
      },
      axisPointer: {
        animation: false
      }
    },
    xAxis: {
      type: 'time',
      splitLine: {
        show: false
      }
    },
    yAxis: {
      type: 'value',
      boundaryGap: [0, '100%'],
      splitLine: {
        show: false
      }
    },
    series: []
  });
  const setIntervalInstance = useRef<number>(0);
  const preCache = useRef({
    received: -1,
    transmitted: -1,
    time: 0,
  });
  const [receivedCount, setReceivedCount] = useState(0);
  const [transmittedCount, setTransmittedCount] = useState(0);

  useEffect(() => {
    chartInstance.current = echarts.init(chartRef.current);
    setChartData();
    setIntervalInstance.current = setInterval(() => {
      getNetReceivedTransmitted();
    }, 1000);
    return () => {
      clearInterval(setIntervalInstance.current);
      chartInstance.current = null;
      preCache.current = {
        received: -1,
        transmitted: -1,
        time: 0,
      };
    }
  }, []);

  async function getNetReceivedTransmitted() {
    const res = await invoke('get_net_received_transmitted').catch(() => ({})) as any;
    const received = res.received[0];
    const transmitted = res.transmitted[0];
    const timeDate = dayjs();
    let receivedSpeed = 0;
    let transmittedSpeed = 0;
    const timeDiff = (dayjs().valueOf() - preCache.current.time) / 1000;
    if(preCache.current.received >= 0) {
      receivedSpeed = bytesToMB((received - preCache.current.received) / timeDiff);
      const diff = received - preCache.current.received;
      setReceivedCount(n => n + diff);
    }
    if(preCache.current.transmitted >= 0) {
      transmittedSpeed = bytesToMB((transmitted - preCache.current.transmitted) / timeDiff);
      const diff = transmitted - preCache.current.transmitted;
      setTransmittedCount(n => n + diff);
    }
    if(series.current[0].data.length >= 3600) {
      series.current[0].data.splice(0, 1);
    }
    if(series.current[1].data.length >= 3600) {
      series.current[0].data.splice(0, 1);
    }
    series.current[0].data.push({
      name: timeDate.format('HH:mm:ss'),
      value: [
        timeDate.unix() * 1000,
        receivedSpeed
      ]
    })
    series.current[1].data.push({
      name: timeDate.format('HH:mm:ss'),
      value: [
        timeDate.unix() * 1000,
        transmittedSpeed
      ]
    });
    preCache.current.received = received;
    preCache.current.transmitted = transmitted;
    preCache.current.time = dayjs().valueOf();
    setChartData();
  }

  function setChartData() {
    chartInstance.current?.setOption({
      ...options.current,
      series: series.current
    });
  }

  return <div className="view__network">
    <div className="main-chart" ref={chartRef}></div>
    <div>
      <span>下载总量：</span>
      <span>{`${bytesToMB(receivedCount)} MB`}</span>
    </div>
    <div>
      <span>上传总量：</span>
      <span>{`${bytesToMB(transmittedCount)} MB`}</span>
    </div>
  </div>
}

export default Network;