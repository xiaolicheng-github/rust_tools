import { useNavigate } from "react-router-dom";
import { IToolsItem, tools } from '../../tools'
import './home.scss';

function Home() {
  const navigate = useNavigate();
  function goToView(item: IToolsItem) {
    if(item?.icon) {
      navigate(`/${item.id}`)
    }
  }

  return <div className="view__home">
    <div className='tools'>
      {
        tools.map(item => (
          <div className='tools-item' key={item.id} onClick={() => goToView(item)}>
            {!!item?.icon && <span className={`tools-item-icon icon-class ${item.icon}`}></span>}
            <div className='tools-item-name'>{item.name}</div>
          </div>
        ))
      }
    </div>
  </div>
}
export default Home;