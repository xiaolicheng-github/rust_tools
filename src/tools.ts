export enum EToolId {
  Performance = 'performance-info',
  Internet = 'internet'
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
    id: EToolId.Internet,
    name: '网络',
  },
  {
    id: 'test',
    name: '网络'
  },
  {
    id: 'test1',
    name: 'test1'
  },
  {
    id: 'test2',
    name: 'test2'
  },
  {
    id: 'test3',
    name: 'test3'
  }
];