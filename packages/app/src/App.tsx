import { useEffect } from "react";
import { useExtensionManager, useExtensionPanels } from "@weaverail/extensions";
import { CoreExtensions } from "@weaverail/core-extensions";

function App() {
  const { manager, api } = useExtensionManager();
  const mainPanels = useExtensionPanels("main");
  const sidePanels = useExtensionPanels("sidebar");

  useEffect(() => {
    manager.addExtension(new CoreExtensions(), api);
  }, []);

  console.log(manager);

  return (
    <div className="outer">
      <main className="container">
        <div className="main-area">
          めいｎ
          {mainPanels.map((v) => (
            <div key={v.id} className="panel-unit">
              {v.render()}
            </div>
          ))}
        </div>

        <aside className="sidebar-area">
          さいどば
          {sidePanels.map((v) => (
            <div key={v.id} className="panel-unit">
              {v.render()}
            </div>
          ))}
        </aside>
      </main>
    </div>
  );
}

export default App;
