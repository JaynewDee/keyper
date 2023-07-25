import './App.css'
import { useEffect, useState } from 'react'
import { listen, emit } from '@tauri-apps/api/event'
// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event


// await invoke("initialize");
import { BsArrowRightShort as RightArrow, BsArrowLeftShort as LeftArrow, BsArrowUpShort as UpArrow, BsArrowDownShort as DownArrow } from 'react-icons/bs';
import { MdOutlineSpaceBar as Spacebar } from "react-icons/md"
import { IconType } from 'react-icons';

function App() {
  const [keyHistory, setKeyHistory] = useState<string[]>([])

  useEffect(() => {
    listen('key_event', (event) => {
      let { key } = event.payload as any;
      setKeyHistory(prev => {
        if (prev[prev.length - 1] === key) return prev;

        return [...prev, key]
      })
    })
  }, [])

  return (
    <>
      <div className="keyscroll-container">
        {keyHistory.map(key => <p>{displaySwitch(key)}</p>)}
      </div>
    </>
  )
}

const withProps = (Icon: IconType, size: string) => <Icon size={size} />;
const keyTranslationTable: { [key: string]: any } = {
  "LeftArrow": withProps(LeftArrow, "3rem"),
  "RightArrow": withProps(RightArrow, "3rem"),
  "UpArrow": withProps(UpArrow, "3rem"),
  "DownArrow": withProps(DownArrow, "3rem"),
  "Space": withProps(Spacebar, "3rem")
}

const displaySwitch = (key: string) => keyTranslationTable[key] || key;

export default App;
