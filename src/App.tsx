import { Route, HashRouter as Router, Routes} from "react-router-dom";
import { EToolId } from "./tools";
import Home from './views/home/home';
import PerformanceInfo from "./views/performance/performance";
import './assets/iconfont/iconfont.css';
import "./App.scss";

function App() {

  function back() {
    window.history.back()
  }
  function goToHome() {
    window.location.href = '/';
  }
  return (
    <div className="app-root">
      <div className="app-root-header">
        <div className="app-root-back-btn" onClick={() => back()}>
          <span className="icon-class icon-fanhui"></span>
        </div>
        <div className="app-root-home-btn" onClick={() => goToHome()}>
          <span className="icon-class icon-home"></span>
        </div>
      </div>
      <Router>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path={`/${EToolId.Performance}`} element={<PerformanceInfo />} />
        </Routes>
      </Router>
    </div>
  );
}

export default App;
