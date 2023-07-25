import './App.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'

import { IconType } from 'react-icons';
import {
  BsArrowRightShort as RightArrow,
  BsArrowLeftShort as LeftArrow,
  BsArrowUpShort as UpArrow,
  BsArrowDownShort as DownArrow
} from 'react-icons/bs';
import {
  MdOutlineSpaceBar as Spacebar,
  MdBackspace as Backspace,
  MdKeyboardTab as Tab,
  MdKeyboardReturn as Return
} from "react-icons/md"
import { PiCursorClickBold as Click } from "react-icons/pi"

///////

function App() {
  const [keyHistory, setKeyHistory] = useState<string[]>([])

  useEffect(() => {
    listen('key_event', (event) => {
      const { key } = event.payload as any;

      setKeyHistory(prev => {
        if (prev[prev.length - 1] === key) return prev;

        const newHistory = prev.length > 15 ? prev.slice(1) : prev;

        return [...newHistory, key]
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

function displaySwitch(key: string) {
  const withProps = (Icon: IconType, size: string) => <Icon size={size} className='key-icon' />;

  const keyTranslationTable: { [key: string]: any } = {
    "LeftArrow": withProps(LeftArrow, "3rem"),
    "RightArrow": withProps(RightArrow, "3rem"),
    "UpArrow": withProps(UpArrow, "3rem"),
    "DownArrow": withProps(DownArrow, "3rem"),
    "Spacebar": withProps(Spacebar, "3rem"),
    "Backspace": withProps(Backspace, "1.66rem"),
    "Tab": withProps(Tab, "2.33rem"),
    "Enter": withProps(Return, "2.33rem"),
    "LeftClick": withProps(Click, "1.66rem"),
    "RightClick": withProps(Click, "1.66rem")
  }

  return keyTranslationTable[key] || key;
};

export default App;
