import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import './service.scss';

function Service() {
  const [logs, setLogs] = useState<string[]>([]);
  const [isRunning, setIsRunning] = useState(false);
  const [port, setPort] = useState('8080');
  const [iframeKey, setIframeKey] = useState(0);

  const startService = async () => {
    setIsRunning(true);
    setLogs(prev => [...prev, `正在启动服务，端口: ${port}...`]);
    
    try {
      const response = await invoke('start_http_service', { port: parseInt(port) }) as string;
      setLogs(prev => [...prev, response]);
      // 仅在成功时保持isRunning为true
    } catch (error) {
      setLogs(prev => [...prev, `服务启动失败: ${error}`]);
      setIsRunning(false); // 失败时设为false
    }
    // 移除finally块
  };

  const stopService = async () => {
    try {
      const response = await invoke('stop_http_service') as string;
      setLogs(prev => [...prev, response]);
      setIsRunning(false);
    } catch (error) {
      setLogs(prev => [...prev, `服务停止失败: ${error}`]);
    }
  };

  // 当服务状态变化时刷新iframe
  useEffect(() => {
    setIframeKey(prev => prev + 1);
  }, [isRunning]);

  const refreshIframe = () => {
    setIframeKey(prev => prev + 1);
    setLogs(prev => [...prev, `正在刷新服务页面...`]);
  };

  return (
    <div className="view__service">
      <div className="service__title">
        <span>Service</span>
        <div className="service__controls">
          <input
            type="number"
            value={port}
            onChange={(e) => setPort(e.target.value)}
            className="service__port-input"
            min="1024"
            max="65535"
          />
          <button 
            onClick={startService}
            disabled={isRunning}
            className="service__button"
          >
            启动服务
          </button>
          <button 
            onClick={stopService}
            disabled={!isRunning}
            className="service__button stop"
          >
            停止服务
          </button>
        </div>
      </div>
      <div className="service__logs">
        {logs.map((log, index) => (
          <div key={index} className="log__item">
            {log}
          </div>
        ))}
      </div>
      {/* 添加iframe验证区域 */}
      <div className="service__iframe-container">
        {isRunning && (
          <>
            <div className="iframe__controls">
              <button 
                onClick={refreshIframe}
                className="service__button refresh"
              >
                刷新页面
              </button>
            </div>
            <iframe
              key={iframeKey}
              src={`http://localhost:${port}`}
              title="服务验证"
              className="service__iframe"
            />
          </>
        )}
      </div>
    </div>
  );
}

export default Service;