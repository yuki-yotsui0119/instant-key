import { useEffect, useMemo, useState } from "react";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { readTextFile } from "@tauri-apps/api/fs";
import { appDataDir, join } from "@tauri-apps/api/path";

// css
import "./App.scss";
import { paths } from "./cache";
import { Key } from "../src-tauri/bindings/Key";
import { appWindow } from "@tauri-apps/api/window";
import { Config } from "../src-tauri/bindings/Config";

type KeyWithImagePath = Key & {
  imageSrc?: string | null;
};

function App() {

  return (
    <div className="app-wrapper">
      <Keyboard />
    </div>
  );
}

export default App;


const Keyboard = () => {
  const [keyDatas, setKeyDatas] = useState<KeyWithImagePath[]>([]);

  const topRow = [
    'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'
  ];
  const middleRow = [
    'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'
  ];

  const bottomRow = [
    'Z', 'X', 'C', 'V', 'B', 'N', 'M'
  ];

  const fetchKeyDatas = async () => {
    const getKeyDatas: Config = await readTextFile(
      await join(paths.get('appDataDirPath'), "config", `config.json`)
    ).then(data => JSON.parse(data));

    const result = await Promise.all(getKeyDatas.keys.map(async (key: KeyWithImagePath)=>{
      if (key.imagePath) {
        key.imageSrc = convertFileSrc(key.imagePath);
      }
      return key;
    }));

    setKeyDatas(result || []);
    // keyDatas.splice(0, keyDatas.length, ...(getKeyDatas.keys || []));
  };
    
  useEffect(() => {
    (async () => {
      // get and set values
      paths.set('appDataDirPath', await appDataDir());
      await fetchKeyDatas();    
    })();

    const handleKeyDown = async (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        appWindow.hide();
      }

      if (event.key.match(/^[a-zA-Z]$/)) {
        if (event.key) {
          // invokeとappWindowのロジックをここに移動
          await invoke('execute_command', {key: event.key.toUpperCase()});
          appWindow.hide();
        }
      }
    };

    // Event listenerを追加
    document.addEventListener('keydown', handleKeyDown);

    // Event listenerをクリーンアップ
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [])

  const keyData: {
    top: Map<string, KeyWithImagePath | null>,
    middle: Map<string, KeyWithImagePath | null>,
    bottom: Map<string, KeyWithImagePath | null>
  } = useMemo(() => {
    // topRowの順番通りに変数に入れる、その際keydatasの中のkeyと一致するものを入れる
    const top = new Map();
    topRow.forEach((key) => {
      const keyData = keyDatas.find(keyData => keyData.key === key);
      if (keyData) {
        top.set(key, keyData);
      } else {
        top.set(key, null);
      }
    });

    const middle = new Map();
    middleRow.forEach((key) => {
      const keyData = keyDatas.find(keyData => keyData.key === key);
      if (keyData) {
        middle.set(key, keyData);
      } else {
        middle.set(key, null);
      }
    });

    const bottom = new Map();
    bottomRow.forEach((key) => {
      const keyData = keyDatas.find(keyData => keyData.key === key);
      if (keyData) {
        bottom.set(key, keyData);
      } else {
        bottom.set(key, null);
      }
    });

    return {
      top,
      middle,
      bottom
    };
  }, [keyDatas]);

  console.log(keyData);

  return (

    <div className="keyboard">
      <div className="top row">
        <div className="key tab flex-grow extra-key"><div>Tab</div></div>
        {
          Array.from(keyData.top).map(([key, value]) => <KeyBody3 keyData={value} keyLabel={key} />)
        }
        <div className="key common extra-key">[</div>
        <div className="key common extra-key">]</div>
        <div className="key common extra-key">\</div>
      </div>
      <div className="middle row">
        <div className="key caps-lock flex-grow extra-key"><div>Caps Lock</div></div>
        {
          Array.from(keyData.middle).map(([key, value]) => <KeyBody3 keyData={value} keyLabel={key} />)
        }
        <div className="key common extra-key">;</div>
        <div className="key common extra-key">'</div>
        <div className="key return flex-grow extra-key"><div>Return</div></div>
      </div>
      <div className="bottom row">
        <div className="key shift flex-grow extra-key"><div>Shift</div></div>
        {
          Array.from(keyData.bottom).map(([key, value]) => <KeyBody3 keyData={value} keyLabel={key} />)
        }
        <div className="key common extra-key">,</div>
        <div className="key common extra-key">{"."}</div>
        <div className="key common extra-key">/</div>
        <div className="key shift flex-grow extra-key"><div>Shift</div></div>
      </div>
    </div>
  );
};

const KeyBody3 = ({ keyLabel, keyData }: KeyProps) => {
  return (
    <div className="add-image common">
      <div className="key add-image-wrapper">
        <div className="key-cotnent">
          <div className="key-label">
            {keyLabel}
          </div>
          {
            keyData?.imageSrc && (
              <img src={keyData.imageSrc} alt={keyLabel} />
            )
          }
        </div>
      </div>
      <div className="key-title">
        {keyData?.title}
      </div>
    </div>
  )

}


type KeyProps = {
  keyLabel: string;
  keyData: KeyWithImagePath | null
}
