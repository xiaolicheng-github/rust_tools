export enum EToolId {
  Performance = 'performance-info',
  Network = 'network'
}
export interface IToolsItem {
  id: EToolId | string;
  name: string
  icon?: string;
}
export const tools: IToolsItem[] = [
  {
    id: EToolId.Performance,
    name: '主机信息',
    icon: 'icon-zhuji',
  },
  {
    id: EToolId.Network,
    name: '网络',
    icon: 'icon-net'
  }
];